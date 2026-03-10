use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Paragraph, Row, Sparkline, Table, TableState, Tabs,
        canvas::{Canvas, Circle, Points},
    },
};

use crate::app::{App, InputMode, Tab};
use planetuium::astro::CartesianCoordinates;

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ])
    .split(f.area());

    render_tabs(f, app, chunks[0]);

    match app.tab {
        Tab::Planetarium => render_planetarium(f, app, chunks[1]),
        Tab::Weather => render_weather(f, app, chunks[1]),
    }

    render_status(f, app, chunks[2]);
}

fn render_tabs(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let selected = match app.tab {
        Tab::Planetarium => 0,
        Tab::Weather => 1,
    };
    let tabs = Tabs::new(vec!["[P] Planetarium", "[W] Weather"])
        .select(selected)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Planetuium "),
        )
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .divider("|");
    f.render_widget(tabs, area);
}

fn render_planetarium(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let cols = Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(area);

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

    let sun_pos = app.sun_moon.sun_stereo.as_ref().map(|p| {
        let c = CartesianCoordinates::from(p);
        (c.x, c.y)
    });
    let moon_pos = app.sun_moon.moon_stereo.as_ref().map(|p| {
        let c = CartesianCoordinates::from(p);
        (c.x, c.y)
    });
    let phase_angle = app.sun_moon.moon_phase_angle;
    let phase_pct = (1.0 - phase_angle.to_radians().cos()) / 2.0 * 100.0;

    let canvas_title = if test_mode {
        " Sky View (horizon circle, N=bottom) [ORION ONLY] "
    } else {
        " Sky View (horizon circle, N=bottom) "
    };

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(canvas_title),
        )
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

            // Cardinal labels (N=bottom due to canvas_orient convention)
            ctx.print(0.0, -2.15, "N");
            ctx.print(0.0, 2.15, "S");
            ctx.print(2.15, 0.0, "E");
            ctx.print(-2.15, 0.0, "W");

            if let Some((sx, sy)) = sun_pos {
                ctx.print(
                    sx,
                    sy,
                    Line::from(Span::styled("☀", Style::default().fg(Color::Yellow))),
                );
            }

            if let Some((mx, my)) = moon_pos {
                ctx.print(
                    mx,
                    my,
                    Line::from(Span::styled("●", Style::default().fg(Color::White))),
                );
                ctx.print(mx + 0.12, my + 0.12, format!("{phase_pct:.0}%"));
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
    let phase_pct =
        (1.0 - app.sun_moon.moon_phase_angle.to_radians().cos()) / 2.0 * 100.0;

    let text = vec![
        Line::from(Span::styled(" Sky Info", Style::default().add_modifier(Modifier::BOLD))),
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
        Line::from(format!("  Angle: {:.1}°", app.sun_moon.moon_phase_angle)),
    ];

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

    let chunks = Layout::vertical([Constraint::Min(0), Constraint::Length(5)]).split(area);

    // Table
    let header = Row::new(vec!["Time", "Cloud%", "Humid%", "Precip%", "Vis(km)", "Temp°C", "Seeing"])
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
                f.time.clone(),
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
        Constraint::Length(17),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(10),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Weather Forecast (↑/↓ scroll) "),
        );

    let mut state = TableState::default();
    state.select(Some(app.weather_scroll));
    f.render_stateful_widget(table, chunks[0], &mut state);

    // Sparkline of cloud cover for next 24h
    let cloud_data: Vec<u64> = forecasts
        .iter()
        .take(24)
        .map(|f| f.cloud_cover as u64)
        .collect();

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Cloud Cover - Next 24h (%) "),
        )
        .data(&cloud_data)
        .max(100)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(sparkline, chunks[1]);
}

fn render_status(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let dt_str = app.datetime.format("%Y-%m-%d %H:%M UTC").to_string();
    let live_str = if app.live_mode { " [LIVE]" } else { "" };

    let editing_hint = match app.input_mode {
        InputMode::Normal => String::new(),
        InputMode::EditingLat => format!(" Editing lat: {}_", app.input_buf),
        InputMode::EditingLon => format!(" Editing lon: {}_", app.input_buf),
        InputMode::EditingDatetime => format!(" Editing time: {}_", app.input_buf),
    };

    let line1 = if editing_hint.is_empty() {
        format!(
            " Lat:{:.4} Lon:{:.4} {}{}",
            app.lat, app.lon, dt_str, live_str
        )
    } else {
        editing_hint
    };

    let line2 = " [L]lat [O]lon [T]time [Space]live [+/-]mag [D]orion [R]weather [P/W]tab [Q]quit";

    let text = vec![Line::from(line1), Line::from(line2)];
    let para =
        Paragraph::new(text).block(Block::default().borders(Borders::TOP).title(" Controls "));
    f.render_widget(para, area);
}
