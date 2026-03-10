mod app;
mod planetarium;
mod ui;
mod weather;

use std::io::stdout;
use std::sync::mpsc;
use std::time::Duration;

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use app::{App, InputMode, Tab};
use weather::HourlyForecast;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let (lat, lon, height) = parse_args(&args);

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, lat, lon, height);

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    result
}

fn parse_args(args: &[String]) -> (f64, f64, f64) {
    let mut lat = 38.93;
    let mut lon = -92.36;
    let mut height = 0.0;
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
    (lat, lon, height)
}

fn spawn_weather(
    tx: &mpsc::Sender<Result<Vec<HourlyForecast>>>,
    lat: f64,
    lon: f64,
) {
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
) -> Result<()> {
    let mut app = App::new(lat, lon, height);
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

        // Update live mode
        if app.live_mode {
            app.datetime = Utc::now();
            app.recompute();
        }

        terminal.draw(|f| ui::render(f, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                // Ctrl+C always quits
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('c')
                {
                    break;
                }

                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Char('p') | KeyCode::Char('P') => {
                            app.tab = Tab::Planetarium;
                        }
                        KeyCode::Char('w') | KeyCode::Char('W') => {
                            app.tab = Tab::Weather;
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
                            app.input_buf =
                                app.datetime.format("%Y-%m-%d %H:%M").to_string();
                        }
                        KeyCode::Char(' ') => {
                            app.live_mode = !app.live_mode;
                            if app.live_mode {
                                app.datetime = Utc::now();
                                app.recompute();
                            }
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app.max_mag = (app.max_mag + 0.5).min(8.0);
                            app.recompute();
                        }
                        KeyCode::Char('-') => {
                            app.max_mag = (app.max_mag - 0.5).max(0.0);
                            app.recompute();
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
                    InputMode::EditingLat
                    | InputMode::EditingLon
                    | InputMode::EditingDatetime => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.input_buf.clear();
                        }
                        KeyCode::Enter => {
                            apply_input(&mut app);
                            app.recompute();
                            spawn_weather(&tx, app.lat, app.lon);
                            app.weather_loading = true;
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
                    },
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
            }
        }
        InputMode::EditingLon => {
            if let Ok(v) = app.input_buf.parse::<f64>() {
                app.lon = v.clamp(-180.0, 180.0);
            }
        }
        InputMode::EditingDatetime => {
            if let Ok(naive) =
                NaiveDateTime::parse_from_str(&app.input_buf, "%Y-%m-%d %H:%M")
            {
                app.datetime = DateTime::from_naive_utc_and_offset(naive, Utc);
                app.live_mode = false;
            }
        }
        InputMode::Normal => {}
    }
}
