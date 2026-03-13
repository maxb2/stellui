mod app;
mod config;
mod image_render;
mod sky;
mod ui;
mod weather;

use std::io::stdout;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol as RatatuiImageState;

use app::{App, InputMode, ORRERY_SPEED_PRESETS, SKY_SPEED_PRESETS, Tab, resolve_tz};
use weather::HourlyForecast;

fn main() -> Result<()> {
    let cfg = config::Config::load();
    let args: Vec<String> = std::env::args().collect();
    let (lat, lon, height, timezone_str, max_mag) = parse_args(&args, &cfg);

    let timezone_override = timezone_str
        .as_deref()
        .and_then(|s| s.parse::<chrono_tz::Tz>().ok());

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, lat, lon, height, timezone_override, max_mag);

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    result
}

fn parse_args(args: &[String], cfg: &config::Config) -> (f64, f64, f64, Option<String>, Option<f64>) {
    // Start from config defaults (which already fall back to built-in defaults via Option)
    let mut lat = cfg.lat.unwrap_or(40.71);
    let mut lon = cfg.lon.unwrap_or(-74.01);
    let mut height = cfg.height.unwrap_or(0.0);
    // timezone and max_mag come from config; CLI flags can override lat/lon/height only
    let timezone = cfg.timezone.clone();
    let max_mag = cfg.max_mag;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--lat" if i + 1 < args.len() => {
                lat = args[i + 1].parse().unwrap_or(lat);
                i += 2;
            }
            "--lon" if i + 1 < args.len() => {
                lon = args[i + 1].parse().unwrap_or(lon);
                i += 2;
            }
            "--height" if i + 1 < args.len() => {
                height = args[i + 1].parse().unwrap_or(height);
                i += 2;
            }
            _ => i += 1,
        }
    }
    (lat, lon, height, timezone, max_mag)
}

fn spawn_weather(tx: &mpsc::Sender<Result<Vec<HourlyForecast>>>, lat: f64, lon: f64) {
    let tx = tx.clone();
    std::thread::spawn(move || {
        tx.send(weather::fetch_forecast(lat, lon)).ok();
    });
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    lat: f64,
    lon: f64,
    height: f64,
    timezone_override: Option<chrono_tz::Tz>,
    max_mag_override: Option<f64>,
) -> Result<()> {
    let mut app = App::new(lat, lon, height, timezone_override, max_mag_override);
    let (tx, rx) = mpsc::channel::<Result<Vec<HourlyForecast>>>();

    // Fetch weather on startup
    spawn_weather(&tx, app.lat, app.lon);
    app.weather_loading = true;

    let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::halfblocks());
    let mut image_state: Option<RatatuiImageState> = None;
    let mut last_image_gen: u64 = u64::MAX;

    loop {
        // Poll weather result (non-blocking)
        if let Ok(result) = rx.try_recv() {
            app.weather_loading = false;
            match result {
                Ok(forecasts) => {
                    app.forecasts = Some(forecasts);
                    app.weather_error = None;
                }
                Err(e) => {
                    app.weather_error = Some(e.to_string());
                }
            }
        }

        // Update time
        let now = Instant::now();
        let elapsed_wall = now.duration_since(app.last_tick);
        app.last_tick = now;

        if app.live_mode {
            app.datetime = Utc::now();
            app.recompute();
        } else if !app.time_paused {
            let speed = match app.tab {
                Tab::Sky | Tab::Weather | Tab::Almanac => SKY_SPEED_PRESETS[app.sky_speed_index].0,
                Tab::SolarSystem => ORRERY_SPEED_PRESETS[app.orrery_speed_index].0,
            };
            let sim_nanos = (speed as f64 * elapsed_wall.as_secs_f64() * 1_000_000_000.0) as i64;
            app.datetime += chrono::Duration::nanoseconds(sim_nanos);
            app.recompute();
        }

        if app.use_image_renderer {
            if app.sky_image_gen != last_image_gen
                && let Some(img) = &app.sky_image
            {
                image_state = Some(picker.new_resize_protocol(img.clone()));
                last_image_gen = app.sky_image_gen;
            }
        } else {
            image_state = None;
            last_image_gen = u64::MAX;
        }
        terminal.draw(|f| ui::render(f, &app, image_state.as_mut() as Option<&mut RatatuiImageState>))?;

        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            // Ctrl+C always quits
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                break;
            }

            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        app.tab = Tab::Sky;
                    }
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        app.tab = Tab::Weather;
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        app.tab = Tab::SolarSystem;
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        app.tab = Tab::Almanac;
                        app.almanac = sky::compute_almanac(app.lat, app.lon, app.height, app.datetime, app.timezone);
                    }
                    KeyCode::Char('l') | KeyCode::Char('L') => {
                        app.input_mode = InputMode::EditingLat;
                        app.input_buf = format!("{:.6}", app.lat);
                    }
                    KeyCode::Char('o') | KeyCode::Char('O') => {
                        app.input_mode = InputMode::EditingLon;
                        app.input_buf = format!("{:.6}", app.lon);
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        app.input_mode = InputMode::EditingDatetime;
                        app.input_buf = if let Some(tz) = app.timezone {
                            app.datetime.with_timezone(&tz).format("%Y-%m-%d %H:%M").to_string()
                        } else {
                            app.datetime.format("%Y-%m-%d %H:%M").to_string()
                        };
                    }
                    KeyCode::Char('z') | KeyCode::Char('Z') => {
                        app.input_mode = InputMode::EditingTimezone;
                        app.input_buf = app.timezone
                            .map(|tz| tz.name().to_string())
                            .unwrap_or_default();
                    }
                    KeyCode::Char(' ') => {
                        if !app.live_mode {
                            app.time_paused = !app.time_paused;
                            if !app.time_paused {
                                app.last_tick = Instant::now();
                            }
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        app.live_mode = true;
                        app.time_paused = false;
                        app.datetime = Utc::now();
                        app.last_tick = Instant::now();
                        app.recompute();
                    }
                    KeyCode::Char(',') => {
                        match app.tab {
                            Tab::Sky | Tab::Weather | Tab::Almanac => {
                                if app.sky_speed_index > 0 { app.sky_speed_index -= 1; }
                            }
                            Tab::SolarSystem => {
                                if app.orrery_speed_index > 0 { app.orrery_speed_index -= 1; }
                            }
                        }
                        app.live_mode = false;
                        app.last_tick = Instant::now();
                    }
                    KeyCode::Char('.') => {
                        match app.tab {
                            Tab::Sky | Tab::Weather | Tab::Almanac => {
                                if app.sky_speed_index + 1 < SKY_SPEED_PRESETS.len() { app.sky_speed_index += 1; }
                            }
                            Tab::SolarSystem => {
                                if app.orrery_speed_index + 1 < ORRERY_SPEED_PRESETS.len() { app.orrery_speed_index += 1; }
                            }
                        }
                        app.live_mode = false;
                        app.last_tick = Instant::now();
                    }
                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        app.max_mag = (app.max_mag + 0.5).min(8.0);
                        app.recompute();
                    }
                    KeyCode::Char('-') => {
                        app.max_mag = (app.max_mag - 0.5).max(0.0);
                        app.recompute();
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        app.test_mode = !app.test_mode;
                        app.recompute();
                    }
                    KeyCode::Char('i') | KeyCode::Char('I') => {
                        if matches!(app.tab, Tab::Sky) {
                            app.use_image_renderer = !app.use_image_renderer;
                            if app.use_image_renderer {
                                app.sky_image = Some(image_render::generate_sky_image(&app));
                                app.sky_image_gen += 1;
                            }
                        }
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        if !app.weather_loading {
                            spawn_weather(&tx, app.lat, app.lon);
                            app.weather_loading = true;
                        }
                    }
                    KeyCode::Up => {
                        app.weather_scroll = app.weather_scroll.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        let max = app
                            .forecasts
                            .as_ref()
                            .map(|f| f.len().saturating_sub(1))
                            .unwrap_or(0);
                        if app.weather_scroll < max {
                            app.weather_scroll += 1;
                        }
                    }
                    _ => {}
                },
                InputMode::EditingLat | InputMode::EditingLon | InputMode::EditingDatetime | InputMode::EditingTimezone => {
                    match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.input_buf.clear();
                        }
                        KeyCode::Enter => {
                            apply_input(&mut app);
                            app.recompute();
                            if !matches!(app.input_mode, InputMode::EditingTimezone) {
                                spawn_weather(&tx, app.lat, app.lon);
                                app.weather_loading = true;
                            }
                            app.input_mode = InputMode::Normal;
                            app.input_buf.clear();
                        }
                        KeyCode::Backspace => {
                            app.input_buf.pop();
                        }
                        KeyCode::Char(c) => {
                            app.input_buf.push(c);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

fn apply_input(app: &mut App) {
    match app.input_mode {
        InputMode::EditingLat => {
            if let Ok(v) = app.input_buf.parse::<f64>() {
                app.lat = v.clamp(-90.0, 90.0);
                app.timezone = resolve_tz(app.lat, app.lon);
            }
        }
        InputMode::EditingLon => {
            if let Ok(v) = app.input_buf.parse::<f64>() {
                app.lon = v.clamp(-180.0, 180.0);
                app.timezone = resolve_tz(app.lat, app.lon);
            }
        }
        InputMode::EditingDatetime => {
            if let Ok(naive) = NaiveDateTime::parse_from_str(&app.input_buf, "%Y-%m-%d %H:%M") {
                app.datetime = if let Some(tz) = app.timezone {
                    tz.from_local_datetime(&naive)
                        .earliest()
                        .map(|dt: chrono::DateTime<chrono_tz::Tz>| dt.to_utc())
                        .unwrap_or_else(|| DateTime::from_naive_utc_and_offset(naive, Utc))
                } else {
                    DateTime::from_naive_utc_and_offset(naive, Utc)
                };
                app.live_mode = false;
                app.time_paused = true;
            }
        }
        InputMode::EditingTimezone => {
            if let Ok(tz) = app.input_buf.parse::<chrono_tz::Tz>() {
                app.timezone = Some(tz);
            }
        }
        InputMode::Normal => {}
    }
}
