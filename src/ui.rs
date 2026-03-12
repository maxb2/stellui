use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Paragraph, Row, Sparkline, Table, TableState, Tabs,
        canvas::{Canvas, Circle, Line as CanvasLine, Points},
    },
};

use crate::app::{App, InputMode, ORRERY_SPEED_PRESETS, SKY_SPEED_PRESETS, Tab};
use crate::sky::{self, ALMANAC_STEPS};
use stellui::astro::CartesianCoordinates;

fn planet_color(name: &str) -> Color {
    match name {
        "Mercury" => Color::Gray,
        "Venus" => Color::Yellow,
        "Mars" => Color::Red,
        "Jupiter" => Color::White,
        "Saturn" => Color::Yellow,
        "Uranus" => Color::Cyan,
        "Neptune" => Color::Blue,
        _ => Color::White,
    }
}

fn moon_phase_char(cycle_degrees: f64) -> &'static str {
    // cycle_degrees: 0° = new moon, 90° = first quarter, 180° = full moon, 270° = last quarter
    match (cycle_degrees / 45.0) as u8 {
        0 => "🌑",
        1 => "🌒",
        2 => "🌓",
        3 => "🌔",
        4 => "🌕",
        5 => "🌖",
        6 => "🌗",
        7 => "🌘",
        _ => "🌑",
    }
}

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ])
    .split(f.area());

    render_tabs(f, app, chunks[0]);

    match app.tab {
        Tab::Sky => render_sky(f, app, chunks[1]),
        Tab::Weather => render_weather(f, app, chunks[1]),
        Tab::SolarSystem => render_solar_system(f, app, chunks[1]),
        Tab::Almanac => render_almanac(f, app, chunks[1]),
    }

    render_status(f, app, chunks[2]);
}

fn render_tabs(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let selected = match app.tab {
        Tab::Sky => 0,
        Tab::Weather => 1,
        Tab::SolarSystem => 2,
        Tab::Almanac => 3,
    };
    let tabs = Tabs::new(vec!["[S] Sky", "[W] Weather", "[P] Solar System", "[A] Almanac"])
        .select(selected)
        .block(Block::default().borders(Borders::ALL).title(" Stellui "))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider("|");
    f.render_widget(tabs, area);
}

fn render_sky(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let cols =
        Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)]).split(area);

    render_canvas(f, app, cols[0]);
    render_info_panel(f, app, cols[1]);
}

fn render_canvas(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let test_mode = app.test_mode;
    // Pre-compute star coordinate groups before the closure
    let bright: Vec<(f64, f64)> = app
        .stars
        .iter()
        .filter(|s| s.mag <= 2.0)
        .map(|s| (s.x, s.y))
        .collect();
    let medium: Vec<(f64, f64)> = app
        .stars
        .iter()
        .filter(|s| s.mag > 2.0 && s.mag <= 4.0)
        .map(|s| (s.x, s.y))
        .collect();
    let dim: Vec<(f64, f64)> = app
        .stars
        .iter()
        .filter(|s| s.mag > 4.0)
        .map(|s| (s.x, s.y))
        .collect();

    let planet_positions: Vec<(&str, &str, f64, f64, ratatui::style::Color)> = app
        .planets
        .iter()
        .map(|p| (p.name, p.symbol, p.x, p.y, planet_color(p.name)))
        .collect();

    let sun_pos = app.sun_moon.sun_stereo.as_ref().map(|p| {
        let c = CartesianCoordinates::from(p);
        (c.x, c.y)
    });
    let moon_pos = app.sun_moon.moon_stereo.as_ref().map(|p| {
        let c = CartesianCoordinates::from(p);
        (c.x, c.y)
    });
    let phase_angle = app.sun_moon.moon_cycle_degrees;

    let southern = app.lat < 0.0;
    let canvas_title = if test_mode {
        if southern { " Sky View (horizon circle, N=top) [ORION ONLY] " }
        else { " Sky View (horizon circle, N=bottom) [ORION ONLY] " }
    } else {
        if southern { " Sky View (horizon circle, N=top) " }
        else { " Sky View (horizon circle, N=bottom) " }
    };

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(canvas_title))
        .x_bounds([-2.2, 2.2])
        .y_bounds([-2.2, 2.2])
        .background_color(Color::Black)
        .paint(move |ctx| {
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: 2.0,
                color: Color::DarkGray,
            });

            if !dim.is_empty() {
                ctx.draw(&Points {
                    coords: &dim,
                    color: Color::DarkGray,
                });
            }
            if !medium.is_empty() {
                ctx.draw(&Points {
                    coords: &medium,
                    color: Color::Gray,
                });
            }
            if !bright.is_empty() {
                ctx.draw(&Points {
                    coords: &bright,
                    color: Color::White,
                });
            }

            // Cardinal labels
            let (n_y, s_y) = if southern { (2.15, -2.15) } else { (-2.15, 2.15) };
            let (e_x, w_x) = if southern { (-2.15, 2.15) } else { (2.15, -2.15) };
            ctx.print(0.0, n_y, "N");
            ctx.print(0.0, s_y, "S");
            ctx.print(e_x, 0.0, "E");
            ctx.print(w_x, 0.0, "W");

            if let Some((sx, sy)) = sun_pos {
                ctx.print(
                    sx,
                    sy,
                    Line::from(Span::styled("🌞", Style::default().fg(Color::Yellow))),
                );
            }

            if let Some((mx, my)) = moon_pos {
                ctx.print(
                    mx,
                    my,
                    Line::from(Span::styled(
                        moon_phase_char(phase_angle),
                        Style::default().fg(Color::White),
                    )),
                );
            }

            for (_name, symbol, x, y, color) in &planet_positions {
                ctx.print(
                    *x,
                    *y,
                    Line::from(Span::styled(*symbol, Style::default().fg(*color))),
                );
            }
        });

    f.render_widget(canvas, area);
}

fn render_info_panel(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let sun_status = match &app.sun_moon.sun_stereo {
        Some(_) => "Above horizon",
        None => "Below horizon",
    };
    let moon_status = match &app.sun_moon.moon_stereo {
        Some(_) => "Above horizon",
        None => "Below horizon",
    };
    let phase_pct = (1.0 - app.sun_moon.moon_cycle_degrees.to_radians().cos()) / 2.0 * 100.0;

    let mut text = vec![
        Line::from(Span::styled(
            " Sky Info",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!(" Stars: {}", app.stars.len())),
        Line::from(format!(" Max mag: {:.1}", app.max_mag)),
        Line::from(""),
        Line::from(Span::styled(" Sun", Style::default().fg(Color::Yellow))),
        Line::from(format!("  {sun_status}")),
        Line::from(""),
        Line::from(Span::styled(" Moon", Style::default().fg(Color::White))),
        Line::from(format!("  {moon_status}")),
        Line::from(format!("  Phase: {phase_pct:.0}%")),
        Line::from(format!("  Cycle: {:.1}°", app.sun_moon.moon_cycle_degrees)),
        Line::from(""),
        Line::from(Span::styled(
            " Planets",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ];
    if app.planets.is_empty() {
        text.push(Line::from("  none above horizon"));
    } else {
        for p in &app.planets {
            text.push(Line::from(Span::styled(
                format!("  {} {} ({:.1})", p.symbol, p.name, p.mag),
                Style::default().fg(planet_color(p.name)),
            )));
        }
    }

    let para = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(" Info "));
    f.render_widget(para, area);
}

fn render_weather(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if app.weather_loading {
        let para = Paragraph::new("Loading weather forecast...")
            .block(Block::default().borders(Borders::ALL).title(" Weather "));
        f.render_widget(para, area);
        return;
    }

    if let Some(err) = &app.weather_error {
        let para = Paragraph::new(format!("Error: {err}"))
            .block(Block::default().borders(Borders::ALL).title(" Weather "));
        f.render_widget(para, area);
        return;
    }

    let Some(forecasts) = &app.forecasts else {
        let para = Paragraph::new("Press [R] to fetch weather forecast.")
            .block(Block::default().borders(Borders::ALL).title(" Weather "));
        f.render_widget(para, area);
        return;
    };

    let cols = Layout::horizontal([
        Constraint::Percentage(55),
        Constraint::Percentage(45),
    ])
    .split(area);

    let left_chunks = Layout::vertical([
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Min(0),
    ])
    .split(cols[0]);

    // Time formatter: API returns UTC; convert to observer local time via app.timezone
    use chrono::{NaiveDateTime, TimeZone, Utc};
    let first_local_date = forecasts.first()
        .and_then(|f| NaiveDateTime::parse_from_str(&f.time, "%Y-%m-%dT%H:%M").ok())
        .map(|ndt| {
            if let Some(tz) = app.timezone {
                Utc.from_utc_datetime(&ndt).with_timezone(&tz).date_naive()
            } else {
                ndt.date()
            }
        });
    let format_time = |time_str: &str| -> String {
        let Ok(ndt) = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M") else {
            return time_str.get(11..16).unwrap_or(time_str).to_string();
        };
        if let Some(tz) = app.timezone {
            let local_dt = Utc.from_utc_datetime(&ndt).with_timezone(&tz);
            if first_local_date == Some(local_dt.date_naive()) {
                local_dt.format("%H:%M").to_string()
            } else {
                local_dt.format("%a %H:%M").to_string()
            }
        } else {
            // No timezone info — show UTC
            if first_local_date == Some(ndt.date()) {
                ndt.format("%H:%M").to_string()
            } else {
                ndt.format("%a %H:%M").to_string()
            }
        }
    };

    // Sparklines (left panel)
    let cloud_data: Vec<u64> = forecasts.iter().map(|f| f.cloud_cover as u64).collect();
    let cloud_sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title(" Cloud Cover (%) "))
        .data(&cloud_data)
        .max(100)
        .style(Style::default().fg(Color::Cyan));

    let humidity_data: Vec<u64> = forecasts.iter().map(|f| f.relative_humidity as u64).collect();
    let humidity_sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title(" Humidity (%) "))
        .data(&humidity_data)
        .max(100)
        .style(Style::default().fg(Color::Blue));

    let precip_data: Vec<u64> = forecasts.iter().map(|f| f.precip_probability as u64).collect();
    let precip_sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title(" Precip Probability (%) "))
        .data(&precip_data)
        .max(100)
        .style(Style::default().fg(Color::Yellow));

    let temps: Vec<f64> = forecasts.iter().map(|f| f.temperature_c).collect();
    let temp_min = temps.iter().cloned().fold(f64::INFINITY, f64::min);
    let temp_max = temps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let shift = if temp_min < 0.0 { temp_min.abs().ceil() as u64 } else { 0 };
    let temp_data: Vec<u64> = temps.iter().map(|&t| (t + shift as f64) as u64).collect();
    let temp_range = ((temp_max - temp_min).ceil() as u64 + 1).max(1);
    let temp_sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title(format!(" Temperature [{:.0}..{:.0} °C] ", temp_min, temp_max)))
        .data(&temp_data)
        .max(temp_range + shift)
        .style(Style::default().fg(Color::Red));

    let vis_data: Vec<u64> = forecasts.iter().map(|f| (f.visibility_km * 10.0) as u64).collect();
    let vis_max = vis_data.iter().cloned().max().unwrap_or(1).max(1);
    let vis_sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title(" Visibility (km) "))
        .data(&vis_data)
        .max(vis_max)
        .style(Style::default().fg(Color::Green));

    f.render_widget(cloud_sparkline,    left_chunks[0]);
    f.render_widget(humidity_sparkline, left_chunks[1]);
    f.render_widget(precip_sparkline,   left_chunks[2]);
    f.render_widget(temp_sparkline,     left_chunks[3]);
    f.render_widget(vis_sparkline,      left_chunks[4]);

    // Table (right panel)
    let header = Row::new(vec!["Time", "Cld", "Hum", "Prc", "Vis", "Tmp", "Seeing"])
        .style(Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED));

    let rows: Vec<Row> = forecasts
        .iter()
        .map(|f| {
            use crate::weather::SeeingQuality;
            let color = match f.seeing {
                SeeingQuality::Excellent | SeeingQuality::Good => Color::Green,
                SeeingQuality::Fair => Color::Yellow,
                SeeingQuality::Poor | SeeingQuality::Bad => Color::Red,
            };
            Row::new(vec![
                format_time(&f.time),
                format!("{:.0}", f.cloud_cover),
                format!("{:.0}", f.relative_humidity),
                format!("{:.0}", f.precip_probability),
                format!("{:.1}", f.visibility_km),
                format!("{:.1}", f.temperature_c),
                f.seeing.label().to_string(),
            ])
            .style(Style::default().fg(color))
        })
        .collect();

    let widths = [
        Constraint::Length(9),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(6),
        Constraint::Length(6),
        Constraint::Min(5),
    ];

    let table = Table::new(rows, widths).header(header).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Weather Forecast (↑/↓ scroll) "),
    );

    let mut state = TableState::default();
    state.select(Some(app.weather_scroll));
    f.render_stateful_widget(table, cols[1], &mut state);
}

fn orrery_planet_color(name: &str) -> Color {
    match name {
        "Mercury" => Color::Gray,
        "Venus" => Color::Yellow,
        "Earth" => Color::Cyan,
        "Mars" => Color::Red,
        "Jupiter" => Color::White,
        "Saturn" => Color::Yellow,
        "Uranus" => Color::Cyan,
        "Neptune" => Color::Blue,
        _ => Color::White,
    }
}

fn render_solar_system(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let cols =
        Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)]).split(area);
    render_orrery_canvas(f, app, cols[0]);
    render_orrery_info(f, app, cols[1]);
}

fn render_orrery_canvas(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let orbit_radii: Vec<f64> = sky::PLANET_SEMI_MAJOR_AXES
        .iter()
        .map(|&(_, sma)| sky::orrery_scale(sma))
        .collect();
    let planet_data: Vec<(f64, f64, &str, Color)> = app
        .orrery
        .planets
        .iter()
        .map(|p| (p.cx, p.cy, p.symbol, orrery_planet_color(p.name)))
        .collect();

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Solar System (top-down, sqrt AU scale) "),
        )
        .x_bounds([-6.2, 6.2])
        .y_bounds([-6.2, 6.2])
        .background_color(Color::Black)
        .paint(move |ctx| {
            for r in &orbit_radii {
                ctx.draw(&Circle {
                    x: 0.0,
                    y: 0.0,
                    radius: *r,
                    color: Color::DarkGray,
                });
            }

            ctx.print(
                0.0,
                0.0,
                Line::from(Span::styled("☀", Style::default().fg(Color::Yellow))),
            );

            for (cx, cy, symbol, color) in &planet_data {
                ctx.print(
                    *cx,
                    *cy,
                    Line::from(Span::styled(*symbol, Style::default().fg(*color))),
                );
            }
        });

    f.render_widget(canvas, area);
}

fn render_orrery_info(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let mut text = vec![
        Line::from(Span::styled(
            " Solar System",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " Scale: sqrt(AU)",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " Planets",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ];

    for p in &app.orrery.planets {
        text.push(Line::from(Span::styled(
            format!("  {} {} {:.2} AU", p.symbol, p.name, p.dist_au),
            Style::default().fg(orrery_planet_color(p.name)),
        )));
    }

    let para =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(" Info "));
    f.render_widget(para, area);
}

fn render_status(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let dt_str = app.datetime.format("%Y-%m-%d %H:%M UTC").to_string();
    let mode_str = if app.live_mode {
        " [LIVE]".to_string()
    } else if app.time_paused {
        " [PAUSED]".to_string()
    } else {
        let label = match app.tab {
            Tab::Sky | Tab::Weather | Tab::Almanac => SKY_SPEED_PRESETS[app.sky_speed_index].1,
            Tab::SolarSystem => ORRERY_SPEED_PRESETS[app.orrery_speed_index].1,
        };
        format!(" [{}]", label)
    };

    let local_str = if let Some(tz) = app.timezone {
        let local = app.datetime.with_timezone(&tz);
        format!("  {}", local.format("%H:%M %Z"))
    } else {
        String::new()
    };

    let editing_hint = match app.input_mode {
        InputMode::Normal => String::new(),
        InputMode::EditingLat => format!(" Editing lat: {}_", app.input_buf),
        InputMode::EditingLon => format!(" Editing lon: {}_", app.input_buf),
        InputMode::EditingDatetime => format!(" Editing time (local): {}_", app.input_buf),
        InputMode::EditingTimezone => format!(" Editing timezone: {}_", app.input_buf),
    };

    let line1 = if editing_hint.is_empty() {
        format!(
            " Lat:{:.4} Lon:{:.4}  {}{}{}",
            app.lat, app.lon, dt_str, local_str, mode_str
        )
    } else {
        editing_hint
    };

    let line2 = match app.tab {
        Tab::Sky =>
            " [L]lat [O]lon [T]time [Z]tz [N]now [Space]pause [,/.]speed [+/-]mag [D]orion [S/W/P/A]tab [Q]quit",
        Tab::Weather =>
            " [L]lat [O]lon [R]weather [↑/↓]scroll [S/W/P/A]tab [Q]quit",
        Tab::SolarSystem =>
            " [L]lat [O]lon [T]time [Z]tz [N]now [Space]pause [,/.]speed [S/W/P/A]tab [Q]quit",
        Tab::Almanac =>
            " [L]lat [O]lon [T]time [Z]tz [N]now [Space]pause [,/.]speed [S/W/P/A]tab [Q]quit",
    };

    let text = vec![Line::from(line1), Line::from(line2)];
    let para =
        Paragraph::new(text).block(Block::default().borders(Borders::TOP).title(" Controls "));
    f.render_widget(para, area);
}

fn render_almanac(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let cols =
        Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]).split(area);
    render_almanac_canvas(f, app, cols[0]);
    render_almanac_legend(f, app, cols[1]);
}

fn render_almanac_canvas(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    use std::f64::consts::PI;

    let almanac = &app.almanac;
    let current_step = almanac.current_step;

    // Pre-compute arc segments: Vec<(x1, y1, x2, y2, r, g, b)>
    let mut segments: Vec<(f64, f64, f64, f64, u8, u8, u8)> = Vec::new();
    for track in &almanac.tracks {
        let (base_r, base_g, base_b) = track.color_rgb;
        for k in 0..96usize {
            let idx0 = (current_step + k) % ALMANAC_STEPS;
            let idx1 = (current_step + k + 1) % ALMANAC_STEPS;
            let alt0 = track.altitudes[idx0];
            let alt1 = track.altitudes[idx1];
            if alt0 <= 0.0 && alt1 <= 0.0 {
                continue;
            }
            let r0 = alt0.max(0.0) / 90.0;
            let r1 = alt1.max(0.0) / 90.0;
            let h0 = idx0 as f64 * 15.0 / 60.0;
            let h1 = idx1 as f64 * 15.0 / 60.0;
            let angle0 = 2.0 * PI * h0 / 24.0;
            let angle1 = 2.0 * PI * h1 / 24.0;
            let x0 = angle0.sin() * r0;
            let y0 = angle0.cos() * r0;
            let x1 = angle1.sin() * r1;
            let y1 = angle1.cos() * r1;
            // Stay fully opaque for first 3/4 of the day, smoothstep fade in last quarter
            let fade = if k < 72 {
                1.0
            } else {
                let u = (k - 72) as f64 / 24.0; // 0..1 over last 6h
                1.0 - u * u * (3.0 - 2.0 * u)  // smoothstep
            };
            let cr = (base_r as f64 * fade) as u8;
            let cg = (base_g as f64 * fade) as u8;
            let cb = (base_b as f64 * fade) as u8;
            segments.push((x0, y0, x1, y1, cr, cg, cb));
        }
    }

    // Clock hand endpoint
    let hand_h = current_step as f64 * 15.0 / 60.0;
    let hand_angle = 2.0 * PI * hand_h / 24.0;
    let hand_x = hand_angle.sin();
    let hand_y = hand_angle.cos();

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Almanac — 24h Altitude (local midnight top, clockwise) "),
        )
        .x_bounds([-1.3, 1.3])
        .y_bounds([-1.3, 1.3])
        .background_color(Color::Black)
        .paint(move |ctx| {
            // Reference altitude circles at 30°, 60°, 90°
            for &r in &[1.0 / 3.0, 2.0 / 3.0, 1.0] {
                ctx.draw(&Circle {
                    x: 0.0,
                    y: 0.0,
                    radius: r,
                    color: Color::DarkGray,
                });
            }

            // Hour tick lines and labels
            use std::f64::consts::PI;
            for h in 0..24u32 {
                let angle = 2.0 * PI * h as f64 / 24.0;
                let sin_a = angle.sin();
                let cos_a = angle.cos();
                let inner = 0.92;
                ctx.draw(&CanvasLine {
                    x1: sin_a * inner,
                    y1: cos_a * inner,
                    x2: sin_a,
                    y2: cos_a,
                    color: Color::DarkGray,
                });
            }
            // Hour labels at 0, 6, 12, 18
            ctx.print(0.0, 1.12, "0h");
            ctx.print(1.08, 0.0, "6h");
            ctx.print(0.0, -1.12, "12h");
            ctx.print(-1.15, 0.0, "18h");

            // Altitude labels
            ctx.print(0.02, 1.0 / 3.0, "30°");
            ctx.print(0.02, 2.0 / 3.0, "60°");
            ctx.print(0.02, 0.97, "90°");

            // Body arcs
            for &(x0, y0, x1, y1, cr, cg, cb) in &segments {
                ctx.draw(&CanvasLine {
                    x1: x0,
                    y1: y0,
                    x2: x1,
                    y2: y1,
                    color: Color::Rgb(cr, cg, cb),
                });
            }

            // Clock hand
            ctx.draw(&CanvasLine {
                x1: 0.0,
                y1: 0.0,
                x2: hand_x,
                y2: hand_y,
                color: Color::White,
            });
        });

    f.render_widget(canvas, area);
}

fn render_almanac_legend(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let mut text = vec![
        Line::from(Span::styled(
            " Almanac",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " local midnight=top, clockwise",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            " arcs = next 24h",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            " Body  Alt",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ];

    for track in &app.almanac.tracks {
        let alt = track.altitudes[app.almanac.current_step];
        let (r, g, b) = track.color_rgb;
        let label = if alt > 0.0 {
            format!(" {} {} {:.1}°", track.symbol, track.name, alt)
        } else {
            format!(" {} {} below", track.symbol, track.name)
        };
        text.push(Line::from(Span::styled(
            label,
            Style::default().fg(Color::Rgb(r, g, b)),
        )));
    }

    let para =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(" Legend "));
    f.render_widget(para, area);
}
