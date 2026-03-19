mod app;
mod config;
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

use app::{App, FovDraft, InputMode, NewLocationDraft, ORRERY_SPEED_PRESETS, SKY_SPEED_PRESETS, Tab};
use config::Location;
use weather::HourlyForecast;

fn main() -> Result<()> {
    let cfg = config::Config::load();
    let args: Vec<String> = std::env::args().collect();
    let (locations, max_mag) = parse_args(&args, &cfg);

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, locations, max_mag);

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    result
}

fn parse_args(args: &[String], cfg: &config::Config) -> (Vec<Location>, Option<f64>) {
    let mut locations = cfg.effective_locations();
    let max_mag = cfg.max_mag;

    // CLI --lat/--lon/--height override the first location
    let mut cli_lat: Option<f64> = None;
    let mut cli_lon: Option<f64> = None;
    let mut cli_height: Option<f64> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--lat" if i + 1 < args.len() => {
                cli_lat = args[i + 1].parse().ok();
                i += 2;
            }
            "--lon" if i + 1 < args.len() => {
                cli_lon = args[i + 1].parse().ok();
                i += 2;
            }
            "--height" if i + 1 < args.len() => {
                cli_height = args[i + 1].parse().ok();
                i += 2;
            }
            _ => i += 1,
        }
    }

    if cli_lat.is_some() || cli_lon.is_some() || cli_height.is_some() {
        let first = &locations[0];
        locations[0] = Location {
            name: first.name.clone(),
            lat: cli_lat.unwrap_or(first.lat),
            lon: cli_lon.unwrap_or(first.lon),
            height: cli_height.unwrap_or(first.height),
            timezone: first.timezone.clone(),
        };
    }

    (locations, max_mag)
}

fn spawn_weather(tx: &mpsc::Sender<Result<Vec<HourlyForecast>>>, lat: f64, lon: f64) {
    let tx = tx.clone();
    std::thread::spawn(move || {
        tx.send(weather::fetch_forecast(lat, lon)).ok();
    });
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    locations: Vec<Location>,
    max_mag_override: Option<f64>,
) -> Result<()> {
    let mut app = App::new(locations, 0, max_mag_override);
    let (tx, rx) = mpsc::channel::<Result<Vec<HourlyForecast>>>();

    // Fetch weather on startup
    spawn_weather(&tx, app.lat, app.lon);
    app.weather_loading = true;

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

        terminal.draw(|f| ui::render(f, &app))?;

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
                        while app.selected_bodies.len() < app.almanac.tracks.len() {
                            app.selected_bodies.push(true);
                        }
                    }
                    KeyCode::Char('b') | KeyCode::Char('B') if matches!(app.tab, Tab::Almanac) => {
                        app.input_mode = InputMode::AlmanacBodyPicker;
                        app.almanac_picker_sel = 0;
                    }
                    KeyCode::Char('f') | KeyCode::Char('F') if matches!(app.tab, Tab::Sky) => {
                        if app.fov_active {
                            app.fov_active = false;
                        } else {
                            app.fov_draft = Some(FovDraft {
                                bufs: [
                                    format!("{:.1}", app.fov_alt),
                                    format!("{:.1}", app.fov_az),
                                    format!("{:.1}", app.fov_deg),
                                ],
                                field: 0,
                                error: None,
                            });
                            app.input_mode = InputMode::FovInput;
                        }
                    }
                    KeyCode::Esc if app.fov_active => {
                        app.fov_active = false;
                    }
                    KeyCode::Char('[') if app.fov_active && matches!(app.tab, Tab::Sky) => {
                        app.fov_deg = (app.fov_deg * 1.5).min(90.0);
                    }
                    KeyCode::Char(']') if app.fov_active && matches!(app.tab, Tab::Sky) => {
                        app.fov_deg = (app.fov_deg / 1.5).max(1.0);
                    }
                    KeyCode::Char('o') | KeyCode::Char('O') if matches!(app.tab, Tab::Sky) => {
                        app.show_dsos = !app.show_dsos;
                    }
                    KeyCode::Char('l') | KeyCode::Char('L') => {
                        app.input_mode = InputMode::LocationPicker;
                        app.picker_sel = app.location_index;
                    }
                    KeyCode::Char('t') if matches!(app.tab, Tab::Almanac) => {
                        app.almanac_show_times = !app.almanac_show_times;
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
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        if !app.weather_loading {
                            spawn_weather(&tx, app.lat, app.lon);
                            app.weather_loading = true;
                        }
                    }
                    KeyCode::Up => {
                        if app.fov_active && matches!(app.tab, Tab::Sky) {
                            let step = (app.fov_deg / 6.0).max(0.5);
                            app.fov_alt = (app.fov_alt + step).min(90.0);
                        } else {
                            app.weather_scroll = app.weather_scroll.saturating_sub(1);
                        }
                    }
                    KeyCode::Down => {
                        if app.fov_active && matches!(app.tab, Tab::Sky) {
                            let step = (app.fov_deg / 6.0).max(0.5);
                            app.fov_alt = (app.fov_alt - step).max(-90.0);
                        } else {
                            let max = app
                                .forecasts
                                .as_ref()
                                .map(|f| f.len().saturating_sub(1))
                                .unwrap_or(0);
                            if app.weather_scroll < max {
                                app.weather_scroll += 1;
                            }
                        }
                    }
                    KeyCode::Left => {
                        if app.fov_active && matches!(app.tab, Tab::Sky) {
                            let step = (app.fov_deg / 6.0).max(0.5);
                            app.fov_az = (app.fov_az - step).rem_euclid(360.0);
                        }
                    }
                    KeyCode::Right => {
                        if app.fov_active && matches!(app.tab, Tab::Sky) {
                            let step = (app.fov_deg / 6.0).max(0.5);
                            app.fov_az = (app.fov_az + step).rem_euclid(360.0);
                        }
                    }
                    _ => {}
                },
                InputMode::LocationPicker => match key.code {
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Up => {
                        if app.picker_sel > 0 { app.picker_sel -= 1; }
                    }
                    KeyCode::Down => {
                        if app.picker_sel + 1 < app.locations.len() { app.picker_sel += 1; }
                    }
                    KeyCode::Enter => {
                        app.switch_location(app.picker_sel);
                        spawn_weather(&tx, app.lat, app.lon);
                        app.weather_loading = true;
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        app.new_loc_draft = Some(NewLocationDraft {
                            bufs: [String::new(), String::new(), String::new(), String::new()],
                            field: 0,
                            error: None,
                        });
                        app.input_mode = InputMode::AddingLocation;
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        if app.locations.len() > 1 {
                            app.locations.remove(app.picker_sel);
                            if app.picker_sel >= app.locations.len() {
                                app.picker_sel = app.locations.len() - 1;
                            }
                            if app.location_index >= app.locations.len() {
                                app.location_index = app.locations.len() - 1;
                                app.switch_location(app.location_index);
                                spawn_weather(&tx, app.lat, app.lon);
                                app.weather_loading = true;
                            } else if app.location_index > app.picker_sel {
                                app.location_index -= 1;
                            }
                            config::Config::save(&app.locations, Some(app.max_mag));
                        }
                    }
                    _ => {}
                },
                InputMode::AddingLocation => {
                    let draft = app.new_loc_draft.as_mut().unwrap();
                    match key.code {
                        KeyCode::Esc => {
                            app.new_loc_draft = None;
                            app.input_mode = InputMode::LocationPicker;
                        }
                        KeyCode::Tab | KeyCode::Down => {
                            let draft = app.new_loc_draft.as_mut().unwrap();
                            if draft.field < 3 { draft.field += 1; }
                        }
                        KeyCode::Up => {
                            let draft = app.new_loc_draft.as_mut().unwrap();
                            if draft.field > 0 { draft.field -= 1; }
                        }
                        KeyCode::Backspace => {
                            draft.bufs[draft.field].pop();
                        }
                        KeyCode::Char(c) => {
                            draft.bufs[draft.field].push(c);
                        }
                        KeyCode::Enter => {
                            let draft = app.new_loc_draft.as_mut().unwrap();
                            if draft.field < 3 {
                                draft.field += 1;
                            } else {
                                // Validate and commit
                                let name = draft.bufs[0].trim().to_string();
                                let lat_str = draft.bufs[1].trim().to_string();
                                let lon_str = draft.bufs[2].trim().to_string();
                                let height_str = draft.bufs[3].trim().to_string();

                                let lat_v = lat_str.parse::<f64>().ok().filter(|v| v.abs() <= 90.0);
                                let lon_v = lon_str.parse::<f64>().ok().filter(|v| v.abs() <= 180.0);
                                let height_v = if height_str.is_empty() {
                                    Some(0.0f64)
                                } else {
                                    height_str.parse::<f64>().ok()
                                };

                                if name.is_empty() {
                                    draft.error = Some("Name cannot be empty".to_string());
                                } else if lat_v.is_none() {
                                    draft.error = Some("Lat must be -90..90".to_string());
                                } else if lon_v.is_none() {
                                    draft.error = Some("Lon must be -180..180".to_string());
                                } else if height_v.is_none() {
                                    draft.error = Some("Height must be a number".to_string());
                                } else if let (Some(lat), Some(lon), Some(height)) = (lat_v, lon_v, height_v) {
                                    let new_loc = Location {
                                        name,
                                        lat,
                                        lon,
                                        height,
                                        timezone: None,
                                    };
                                    app.locations.push(new_loc);
                                    let new_index = app.locations.len() - 1;
                                    app.picker_sel = new_index;
                                    config::Config::save(&app.locations, Some(app.max_mag));
                                    app.new_loc_draft = None;
                                    app.input_mode = InputMode::LocationPicker;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                InputMode::AlmanacBodyPicker => match key.code {
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.almanac_picker_sel > 0 { app.almanac_picker_sel -= 1; }
                        else { app.almanac_picker_sel = app.almanac.tracks.len().saturating_sub(1); }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.almanac_picker_sel + 1 < app.almanac.tracks.len() {
                            app.almanac_picker_sel += 1;
                        } else {
                            app.almanac_picker_sel = 0;
                        }
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        let sel = app.almanac_picker_sel;
                        if let Some(v) = app.selected_bodies.get_mut(sel) {
                            *v = !*v;
                        }
                    }
                    _ => {}
                },
                InputMode::FovInput => {
                    let draft = app.fov_draft.as_mut().unwrap();
                    match key.code {
                        KeyCode::Esc => {
                            app.fov_draft = None;
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Tab | KeyCode::Down => {
                            if draft.field < 2 { draft.field += 1; }
                        }
                        KeyCode::Up => {
                            if draft.field > 0 { draft.field -= 1; }
                        }
                        KeyCode::Backspace => {
                            draft.bufs[draft.field].pop();
                        }
                        KeyCode::Char(c) => {
                            draft.bufs[draft.field].push(c);
                        }
                        KeyCode::Enter => {
                            let draft = app.fov_draft.as_mut().unwrap();
                            if draft.field < 2 {
                                draft.field += 1;
                            } else {
                                let alt_v = draft.bufs[0].trim().parse::<f64>().ok()
                                    .filter(|v| v.abs() <= 90.0);
                                let az_v  = draft.bufs[1].trim().parse::<f64>().ok()
                                    .filter(|v| *v >= 0.0 && *v < 360.0);
                                let fov_v = draft.bufs[2].trim().parse::<f64>().ok()
                                    .filter(|v| *v >= 1.0 && *v <= 90.0);
                                if let (Some(alt), Some(az), Some(fov)) = (alt_v, az_v, fov_v) {
                                    app.fov_alt = alt;
                                    app.fov_az = az;
                                    app.fov_deg = fov;
                                    app.fov_active = true;
                                    app.fov_draft = None;
                                    app.input_mode = InputMode::Normal;
                                } else {
                                    draft.error = Some("Alt:-90..90  Az:0..360  FoV:1..90".to_string());
                                }
                            }
                        }
                        _ => {}
                    }
                }
                InputMode::EditingDatetime | InputMode::EditingTimezone => {
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
        InputMode::Normal | InputMode::LocationPicker | InputMode::AddingLocation | InputMode::AlmanacBodyPicker | InputMode::FovInput => {}
    }
}
