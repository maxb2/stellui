use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Paragraph, Row, Sparkline, Table, TableState, Tabs,
        canvas::{Canvas, Circle, Line as CanvasLine, Points},
    },
};

use crate::app::{App, FovDraft, InputMode, NewLocationDraft, ORRERY_SPEED_PRESETS, SKY_SPEED_PRESETS, Tab, search_hits};
use crate::sky::{self, ALMANAC_STEPS};
use stellui::astro::CartesianCoordinates;
use stellui::dso::DsoKind;

/// Tangent-plane (gnomonic) projection.
/// xi > 0 = east; eta > 0 = up. Returns None if behind the projection plane.
fn gnomonic_project(obj_alt: f64, obj_az: f64, calt: f64, caz: f64) -> Option<(f64, f64)> {
    let (ar, zr) = (obj_alt.to_radians(), obj_az.to_radians());
    let (cr, czr) = (calt.to_radians(), caz.to_radians());
    let vx = ar.cos() * zr.sin();
    let vy = ar.cos() * zr.cos();
    let vz = ar.sin();
    let cx = cr.cos() * czr.sin();
    let cy = cr.cos() * czr.cos();
    let cz = cr.sin();
    let dot = cx * vx + cy * vy + cz * vz;
    if dot <= 0.0 { return None; }
    let xi  = (czr.cos() * vx - czr.sin() * vy) / dot;
    let eta = (-cr.sin() * czr.sin() * vx - cr.sin() * czr.cos() * vy + cr.cos() * vz) / dot;
    Some((xi, eta))
}

/// Project and scale to canvas [-1, 1] bounds; returns None if outside.
fn project_and_scale(alt: f64, az: f64, calt: f64, caz: f64, scale: f64) -> Option<(f64, f64)> {
    let (xi, eta) = gnomonic_project(alt, az, calt, caz)?;
    let (cx, cy) = (xi * scale, eta * scale);
    if cx.abs() > 1.0 || cy.abs() > 1.0 { return None; }
    Some((cx, cy))
}

fn fov_tick_spacing(fov: f64) -> usize {
    if fov > 60.0 { 30 } else if fov > 30.0 { 10 } else if fov > 10.0 { 5 }
    else if fov > 5.0 { 2 } else { 1 }
}

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

fn dso_color(kind: DsoKind) -> Color {
    match kind {
        DsoKind::Galaxy          => Color::Cyan,
        DsoKind::OpenCluster     => Color::White,
        DsoKind::GlobularCluster => Color::Yellow,
        DsoKind::Nebula          => Color::Magenta,
        DsoKind::PlanetaryNebula => Color::Cyan,
        DsoKind::SupernovaRemnant | DsoKind::Other => Color::Gray,
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
        Tab::Targets => render_best_targets(f, app, chunks[1]),
        Tab::Conjunctions => render_conjunctions(f, app, chunks[1]),
    }

    render_status(f, app, chunks[2]);

    // Overlays
    match app.input_mode {
        InputMode::LocationPicker => render_location_picker(f, app),
        InputMode::AddingLocation => render_add_location(f, app),
        InputMode::AlmanacBodyPicker => render_almanac_body_picker(f, app),
        InputMode::FovInput => render_fov_input(f, app),
        InputMode::ObjectSearch => render_object_search(f, app),
        InputMode::EyepieceCalc => render_eyepiece_calc(f, app),
        _ => {}
    }
}

fn render_tabs(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let selected = match app.tab {
        Tab::Sky => 0,
        Tab::Weather => 1,
        Tab::SolarSystem => 2,
        Tab::Almanac => 3,
        Tab::Targets => 4,
        Tab::Conjunctions => 5,
    };
    let tabs = Tabs::new(vec!["[S] Sky", "[W] Weather", "[P] Solar System", "[A] Almanac", "[B] Best Targets", "[C] Conjunctions"])
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

    if app.fov_active {
        render_fov_canvas(f, app, cols[0]);
    } else {
        render_canvas(f, app, cols[0]);
    }
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

    let dso_positions: Vec<(&str, &str, f64, f64, ratatui::style::Color)> = if app.show_dsos {
        app.dsos
            .iter()
            .filter(|d| d.alt >= 0.0)
            .map(|d| (d.catalog, d.kind.symbol(), d.x, d.y, dso_color(d.kind)))
            .collect()
    } else {
        Vec::new()
    };

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

            for (_catalog, symbol, x, y, color) in &dso_positions {
                ctx.print(
                    *x,
                    *y,
                    Line::from(Span::styled(*symbol, Style::default().fg(*color))),
                );
            }
        });

    f.render_widget(canvas, area);
}

fn render_fov_canvas(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let calt = app.fov_alt;
    let caz = app.fov_az;
    let fov_deg = app.fov_deg;
    let scale = 1.0 / (fov_deg / 2.0).to_radians().tan();

    // Project stars into brightness buckets
    let bright: Vec<(f64, f64)> = app.stars.iter()
        .filter(|s| s.mag <= 2.0)
        .filter_map(|s| project_and_scale(s.alt, s.az, calt, caz, scale))
        .collect();
    let medium: Vec<(f64, f64)> = app.stars.iter()
        .filter(|s| s.mag > 2.0 && s.mag <= 4.0)
        .filter_map(|s| project_and_scale(s.alt, s.az, calt, caz, scale))
        .collect();
    let dim: Vec<(f64, f64)> = app.stars.iter()
        .filter(|s| s.mag > 4.0)
        .filter_map(|s| project_and_scale(s.alt, s.az, calt, caz, scale))
        .collect();

    // Project planets
    let planet_positions: Vec<(&str, &str, f64, f64, ratatui::style::Color)> = app.planets.iter()
        .filter_map(|p| {
            let (cx, cy) = project_and_scale(p.alt, p.az, calt, caz, scale)?;
            Some((p.name, p.symbol, cx, cy, planet_color(p.name)))
        })
        .collect();

    // Project DSOs
    let dso_fov_positions: Vec<(&str, &str, f64, f64, ratatui::style::Color)> = if app.show_dsos {
        app.dsos
            .iter()
            .filter(|d| d.alt >= 0.0)
            .filter_map(|d| {
                let (cx, cy) = project_and_scale(d.alt, d.az, calt, caz, scale)?;
                Some((d.catalog, d.kind.symbol(), cx, cy, dso_color(d.kind)))
            })
            .collect()
    } else {
        Vec::new()
    };

    // Project sun and moon
    let sun_fov_pos = if app.sun_moon.sun_alt >= 0.0 {
        project_and_scale(app.sun_moon.sun_alt, app.sun_moon.sun_az, calt, caz, scale)
    } else {
        None
    };
    let moon_fov_pos = if app.sun_moon.moon_alt >= 0.0 {
        project_and_scale(app.sun_moon.moon_alt, app.sun_moon.moon_az, calt, caz, scale)
    } else {
        None
    };
    let phase_angle = app.sun_moon.moon_cycle_degrees;

    // Horizon line: all horizon points project to y = -tan(calt) * scale
    let y_horiz = -calt.to_radians().tan() * scale;
    let horizon_visible = y_horiz.abs() <= 1.0 && calt.abs() < 89.0;

    // Az ticks along the horizon line
    let az_ticks: Vec<(f64, Option<String>)> = if horizon_visible {
        let tick_step = fov_tick_spacing(fov_deg);
        (0u32..360).step_by(tick_step)
            .filter_map(|az_tick| {
                let (cx, _) = gnomonic_project(0.0, az_tick as f64, calt, caz)?;
                let cx = cx * scale;
                if cx.abs() > 1.0 { return None; }
                let label: Option<String> = match az_tick {
                    0 => Some("N".to_string()),
                    90 => Some("E".to_string()),
                    180 => Some("S".to_string()),
                    270 => Some("W".to_string()),
                    d => Some(format!("{d}°")),
                };
                Some((cx, label))
            })
            .collect()
    } else {
        Vec::new()
    };

    let title = format!(
        " FoV: alt={:.1}° az={:.1}° fov={:.1}° | [↑↓←→]pan [[]out []]in [f/Esc]exit ",
        calt, caz, fov_deg
    );

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(title))
        .x_bounds([-1.0, 1.0])
        .y_bounds([-1.0, 1.0])
        .background_color(Color::Black)
        .paint(move |ctx| {
            // Crosshair
            ctx.draw(&CanvasLine { x1: -0.05, y1: 0.0, x2: 0.05, y2: 0.0, color: Color::DarkGray });
            ctx.draw(&CanvasLine { x1: 0.0, y1: -0.05, x2: 0.0, y2: 0.05, color: Color::DarkGray });

            // Stars
            if !dim.is_empty() {
                ctx.draw(&Points { coords: &dim, color: Color::DarkGray });
            }
            if !medium.is_empty() {
                ctx.draw(&Points { coords: &medium, color: Color::Gray });
            }
            if !bright.is_empty() {
                ctx.draw(&Points { coords: &bright, color: Color::White });
            }

            // Planets
            for (_name, symbol, x, y, color) in &planet_positions {
                ctx.print(*x, *y, Line::from(Span::styled(*symbol, Style::default().fg(*color))));
            }

            // Sun and moon
            if let Some((sx, sy)) = sun_fov_pos {
                ctx.print(sx, sy, Line::from(Span::styled("🌞", Style::default().fg(Color::Yellow))));
            }
            if let Some((mx, my)) = moon_fov_pos {
                ctx.print(mx, my, Line::from(Span::styled(
                    moon_phase_char(phase_angle),
                    Style::default().fg(Color::White),
                )));
            }

            for (_catalog, symbol, x, y, color) in &dso_fov_positions {
                ctx.print(*x, *y, Line::from(Span::styled(*symbol, Style::default().fg(*color))));
            }

            // Horizon line
            if horizon_visible {
                ctx.draw(&CanvasLine {
                    x1: -1.0, y1: y_horiz, x2: 1.0, y2: y_horiz,
                    color: Color::DarkGray,
                });
                // Az ticks and labels
                for (cx, label) in &az_ticks {
                    ctx.draw(&CanvasLine {
                        x1: *cx, y1: y_horiz - 0.04,
                        x2: *cx, y2: y_horiz + 0.04,
                        color: Color::DarkGray,
                    });
                    if let Some(lbl) = label {
                        ctx.print(*cx, y_horiz + 0.06, lbl.to_owned());
                    }
                }
            }
        });

    f.render_widget(canvas, area);
}

fn render_fov_input(f: &mut Frame, app: &App) {
    let area = centered_popup(f, 60, 10);
    f.render_widget(Clear, area);

    let block = Block::default().borders(Borders::ALL).title(" FoV Settings ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let draft: &FovDraft = app.fov_draft.as_ref().unwrap();
    let field_names = ["Alt (°, -90..90)", "Az (°, 0..360)", "FoV (°, 1..90)"];
    let mut lines: Vec<Line> = field_names.iter().enumerate().map(|(i, label)| {
        let cursor = if i == draft.field { "_" } else { "" };
        let text = format!(" {}: {}{}", label, draft.bufs[i], cursor);
        if i == draft.field {
            Line::from(Span::styled(text, Style::default().fg(Color::Yellow)))
        } else {
            Line::from(text)
        }
    }).collect();

    lines.push(Line::from(""));
    if let Some(err) = &draft.error {
        lines.push(Line::from(Span::styled(
            format!(" Error: {}", err),
            Style::default().fg(Color::Red),
        )));
    } else {
        lines.push(Line::from(""));
    }
    lines.push(Line::from(Span::styled(
        " [Tab/↓]next  [↑]prev  [Enter]next/confirm  [Esc]cancel",
        Style::default().fg(Color::DarkGray),
    )));

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
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
        Line::from(format!(
            " DSOs: {} [o]",
            if app.show_dsos { format!("{}", app.dsos.len()) } else { "off".to_string() }
        )),
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

    if app.show_dsos {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            " DSO symbols",
            Style::default().add_modifier(Modifier::BOLD),
        )));
        const DSO_LEGEND: &[(DsoKind, &str)] = &[
            (DsoKind::Galaxy,          "Galaxy"),
            (DsoKind::GlobularCluster, "Globular"),
            (DsoKind::OpenCluster,     "Open cluster"),
            (DsoKind::Nebula,          "Nebula"),
            (DsoKind::PlanetaryNebula, "Planetary neb."),
            (DsoKind::SupernovaRemnant,"SNR / other"),
        ];
        for &(kind, label) in DSO_LEGEND {
            text.push(Line::from(vec![
                Span::styled(
                    format!("  {} ", kind.symbol()),
                    Style::default().fg(dso_color(kind)),
                ),
                Span::raw(label),
            ]));
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
        Constraint::Length(6),  // cloud cover
        Constraint::Length(6),  // humidity
        Constraint::Length(6),  // precip
        Constraint::Length(6),  // temp
        Constraint::Length(6),  // visibility
        Constraint::Length(6),  // wind
        Constraint::Length(1),  // day/night axis
        Constraint::Min(0),     // overflow
    ])
    .split(cols[0]);

    // Time formatter: API returns UTC; convert to observer local time via app.timezone
    use chrono::{NaiveDateTime, TimeZone, Utc};
    let format_time = |time_str: &str| -> String {
        let Ok(ndt) = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M") else {
            return time_str.get(11..16).unwrap_or(time_str).to_string();
        };
        if let Some(tz) = app.timezone {
            let local_dt = Utc.from_utc_datetime(&ndt).with_timezone(&tz);
            local_dt.format("%a %H:%M").to_string()
        } else {
            // No timezone info — show UTC
            ndt.format("%a %H:%M").to_string()
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

    let wind_data: Vec<u64> = forecasts.iter().map(|f| f.wind_speed_kmh as u64).collect();
    let wind_max = wind_data.iter().cloned().max().unwrap_or(1).max(1);
    let wind_sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title(" Wind Speed (km/h) "))
        .data(&wind_data)
        .max(wind_max)
        .style(Style::default().fg(Color::Magenta));

    f.render_widget(cloud_sparkline,    left_chunks[0]);
    f.render_widget(humidity_sparkline, left_chunks[1]);
    f.render_widget(precip_sparkline,   left_chunks[2]);
    f.render_widget(temp_sparkline,     left_chunks[3]);
    f.render_widget(vis_sparkline,      left_chunks[4]);
    f.render_widget(wind_sparkline,     left_chunks[5]);

    // Day/night axis aligned with sparkline inner content
    let inner_width = left_chunks[0].width.saturating_sub(2) as usize;
    use ratatui::text::{Line, Span};
    let mut axis_spans: Vec<Span> = vec![Span::raw(" ")]; // left-border offset
    axis_spans.extend(forecasts.iter().take(inner_width).map(|f| {
        use crate::weather::DayPeriod;
        let color = match f.day_period {
            DayPeriod::Day                  => Color::Yellow,
            DayPeriod::CivilTwilight        => Color::Rgb(255, 165, 0),
            DayPeriod::NauticalTwilight     => Color::Rgb(100, 140, 210),
            DayPeriod::AstronomicalTwilight => Color::Blue,
            DayPeriod::Night                => Color::Rgb(80, 80, 160),
        };
        Span::styled(f.day_period.symbol(), Style::default().fg(color))
    }));
    f.render_widget(Paragraph::new(Line::from(axis_spans)), left_chunks[6]);

    // Table (right panel)
    use ratatui::widgets::Cell;
    let header = Row::new(vec!["Sky", "Time", "Cld", "Hum", "Prc", "Vis", "Tmp", "Wnd", "Seeing"])
        .style(Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED));

    let rows: Vec<Row> = forecasts
        .iter()
        .map(|f| {
            use crate::weather::{DayPeriod, SeeingQuality};
            let seeing_color = match f.seeing {
                SeeingQuality::Excellent | SeeingQuality::Good => Color::Green,
                SeeingQuality::Fair => Color::Yellow,
                SeeingQuality::Poor | SeeingQuality::Bad => Color::Red,
            };
            let sky_color = match f.day_period {
                DayPeriod::Day => Color::Yellow,
                DayPeriod::CivilTwilight => Color::Rgb(255, 165, 0),
                DayPeriod::NauticalTwilight => Color::Rgb(100, 140, 210),
                DayPeriod::AstronomicalTwilight => Color::Blue,
                DayPeriod::Night => Color::Rgb(80, 80, 160),
            };
            Row::new(vec![
                Cell::new(f.day_period.symbol()).style(Style::default().fg(sky_color)),
                Cell::new(format_time(&f.time)).style(Style::default().fg(seeing_color)),
                Cell::new(format!("{:.0}", f.cloud_cover)).style(Style::default().fg(seeing_color)),
                Cell::new(format!("{:.0}", f.relative_humidity)).style(Style::default().fg(seeing_color)),
                Cell::new(format!("{:.0}", f.precip_probability)).style(Style::default().fg(seeing_color)),
                Cell::new(format!("{:.1}", f.visibility_km)).style(Style::default().fg(seeing_color)),
                Cell::new(format!("{:.1}", f.temperature_c)).style(Style::default().fg(seeing_color)),
                Cell::new(format!("{:.0}", f.wind_speed_kmh)).style(Style::default().fg(seeing_color)),
                Cell::new(f.seeing.label().to_string()).style(Style::default().fg(seeing_color)),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(3),
        Constraint::Length(9),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(6),
        Constraint::Length(6),
        Constraint::Length(4),
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
                Line::from(Span::styled("🌞", Style::default().fg(Color::Yellow))),
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
            Tab::Sky | Tab::Weather | Tab::Almanac | Tab::Targets | Tab::Conjunctions => SKY_SPEED_PRESETS[app.sky_speed_index].1,
            Tab::SolarSystem => ORRERY_SPEED_PRESETS[app.orrery_speed_index].1,
        };
        format!(" [{}]", label)
    };

    let local_str = if let Some(tz) = app.timezone {
        let local = app.datetime.with_timezone(&tz);
        format!("  {} [{}]", local.format("%H:%M %Z"), tz.name())
    } else {
        String::new()
    };

    let loc_name = app.locations.get(app.location_index).map(|l| l.name.as_str()).unwrap_or("");

    let editing_hint = match app.input_mode {
        InputMode::Normal | InputMode::LocationPicker | InputMode::AddingLocation | InputMode::AlmanacBodyPicker | InputMode::FovInput | InputMode::ObjectSearch | InputMode::EyepieceCalc => String::new(),
        InputMode::EditingDatetime => format!(" Editing time (local): {}_", app.input_buf),
        InputMode::EditingTimezone => format!(" Editing timezone: {}_", app.input_buf),
    };

    let line1 = if editing_hint.is_empty() {
        format!(
            " {} Lat:{:.4} Lon:{:.4}  {}{}{}",
            loc_name, app.lat, app.lon, dt_str, local_str, mode_str
        )
    } else {
        editing_hint
    };

    let line2 = match app.tab {
        Tab::Sky =>
            " [L]locations [T]time [Z]tz [N]now [Space]pause [,/.]speed [+/-]mag [D]orion [f]toggle FoV [/]search [S/W/P/A/B]tab [Q]quit",
        Tab::Weather =>
            " [L]locations [R]weather [↑/↓]scroll [S/W/P/A/B]tab [Q]quit",
        Tab::SolarSystem =>
            " [L]locations [T]time [Z]tz [N]now [Space]pause [,/.]speed [S/W/P/A/B]tab [Q]quit",
        Tab::Almanac =>
            " [L]locations [T]time [Z]tz [N]now [Space]pause [,/.]speed [v]bodies [t]times [S/W/P/A/B]tab [Q]quit",
        Tab::Targets =>
            " [↑/↓]scroll  [+/-]mag  [S/W/P/A/B/C]tab  [Q]quit",
        Tab::Conjunctions =>
            " [↑/↓]scroll  [C]conjunctions  [S/W/P/A/B]tab  [N]now  [Q]quit",
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
    for (idx, track) in almanac.tracks.iter().enumerate() {
        if !app.selected_bodies.get(idx).copied().unwrap_or(true) {
            continue;
        }
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

fn render_object_search(f: &mut Frame, app: &App) {
    let hits = search_hits(app, &app.search_query);
    let n = hits.len();

    let popup_lines: u16 = 14;
    let area = centered_popup(f, 70, popup_lines);
    f.render_widget(Clear, area);

    let block = Block::default().borders(Borders::ALL).title(" Object Search ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Query line
    let query_line = Line::from(vec![
        Span::styled("> ", Style::default().fg(Color::Yellow)),
        Span::raw(app.search_query.as_str()),
        Span::styled("_", Style::default().fg(Color::Yellow)),
    ]);

    // Results: up to (inner.height - 3) rows, scrolled around search_sel
    let list_rows = inner.height.saturating_sub(3) as usize;
    let sel = app.search_sel.min(if n == 0 { 0 } else { n - 1 });

    let start = if n == 0 {
        0
    } else {
        let half = list_rows / 2;
        if sel < half { 0 } else { (sel - half).min(n.saturating_sub(list_rows)) }
    };

    let mut result_lines: Vec<Line> = Vec::new();
    if n == 0 {
        result_lines.push(Line::from(Span::styled(
            "  no matches",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (i, (sym, label, alt, az)) in hits.iter().enumerate().skip(start).take(list_rows) {
            let is_sel = i == sel;
            if *alt >= 0.0 {
                let text = format!(" {} {:<30} alt {:>5.1}°  az {:>5.1}°", sym, label, alt, az);
                if is_sel {
                    result_lines.push(Line::from(Span::styled(
                        text,
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    )));
                } else {
                    result_lines.push(Line::from(text));
                }
            } else {
                let text = format!(" {} {:<30} below horizon", sym, label);
                if is_sel {
                    result_lines.push(Line::from(Span::styled(
                        text,
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    )));
                } else {
                    result_lines.push(Line::from(Span::styled(
                        text,
                        Style::default().fg(Color::DarkGray),
                    )));
                }
            }
        }
    }

    let hint = Line::from(Span::styled(
        " [↑↓] select  [Enter] go to  [Esc] cancel",
        Style::default().fg(Color::DarkGray),
    ));

    let mut all_lines = vec![query_line, Line::from("─".repeat(inner.width as usize))];
    all_lines.extend(result_lines);
    // Pad to push hint to bottom
    while all_lines.len() + 1 < inner.height as usize {
        all_lines.push(Line::from(""));
    }
    all_lines.push(hint);

    let para = Paragraph::new(all_lines);
    f.render_widget(para, inner);
}

fn render_eyepiece_calc(f: &mut Frame, app: &App) {
    let area = centered_popup(f, 65, 20);
    f.render_widget(Clear, area);

    let block = Block::default().borders(Borders::ALL).title(" Equipment Calculator [e] ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    if let Some(draft) = &app.calc_draft {
        // ── Add-scope / add-eyepiece sub-form ──
        let (title, labels) = if draft.adding_eyepiece {
            ("New Eyepiece", ["Name", "Focal length (mm)", "AFOV (°, e.g. 52)"])
        } else {
            ("New Scope", ["Name", "Aperture (mm)", "Focal length (mm)"])
        };
        lines.push(Line::from(Span::styled(
            format!(" {title}"),
            Style::default().add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
        for (i, label) in labels.iter().enumerate() {
            let cursor = if i == draft.field { "_" } else { "" };
            let text = format!("  {}: {}{}", label, draft.bufs[i], cursor);
            if i == draft.field {
                lines.push(Line::from(Span::styled(text, Style::default().fg(Color::Yellow))));
            } else {
                lines.push(Line::from(text));
            }
        }
        if let Some(err) = &draft.error {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!(" {err}"),
                Style::default().fg(Color::Red),
            )));
        }
        while lines.len() + 1 < inner.height as usize {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(Span::styled(
            " [Tab/↓] next  [↑] prev  [Enter] confirm  [Esc] back",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        // ── Main calc view ──
        let scope = app.scopes.get(app.scope_sel);
        let ep    = app.eyepieces.get(app.ep_sel);

        // Scope row
        let scope_label = scope.map(|s| s.name.as_str()).unwrap_or("—");
        let scope_line = format!(
            " Scope  [{}/{}]  {}",
            app.scope_sel + 1, app.scopes.len(), scope_label
        );
        if app.calc_row == 0 {
            lines.push(Line::from(Span::styled(
                scope_line,
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(scope_line));
        }
        if let Some(s) = scope {
            let fr = s.focal_length_mm / s.aperture_mm;
            lines.push(Line::from(Span::styled(
                format!("   aperture {:.0}mm  fl {:.0}mm  f/{:.1}", s.aperture_mm, s.focal_length_mm, fr),
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(""));

        // Eyepiece row
        let ep_label = ep.map(|e| e.name.as_str()).unwrap_or("—");
        let ep_line = format!(
            " Eyepiece  [{}/{}]  {}",
            app.ep_sel + 1, app.eyepieces.len(), ep_label
        );
        if app.calc_row == 1 {
            lines.push(Line::from(Span::styled(
                ep_line,
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(ep_line));
        }
        if let Some(e) = ep {
            lines.push(Line::from(Span::styled(
                format!("   fl {:.0}mm  AFOV {:.0}°", e.focal_length_mm, e.afov_deg),
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(""));

        // Computed results
        lines.push(Line::from(Span::styled(
            " ─── Results ───────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )));
        if let (Some(s), Some(e)) = (scope, ep) {
            let mag      = s.focal_length_mm / e.focal_length_mm;
            let true_fov = e.afov_deg / mag;
            let exit_pup = s.aperture_mm / mag;
            let lim_mag  = 2.1 + 5.0 * s.aperture_mm.log10();
            lines.push(Line::from(format!("  Magnification  {:.0}×", mag)));
            lines.push(Line::from(format!("  True FOV       {:.2}°", true_fov)));
            lines.push(Line::from(format!("  Exit pupil     {:.1}mm", exit_pup)));
            lines.push(Line::from(format!("  Limiting mag   ~{:.1}", lim_mag)));
        } else {
            lines.push(Line::from(Span::styled(
                "  (select scope and eyepiece)",
                Style::default().fg(Color::DarkGray),
            )));
        }

        while lines.len() + 2 < inner.height as usize {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(Span::styled(
            " [←→] cycle  [Tab] switch row  [Enter] apply FOV",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            " [n] new  [d] delete  [Esc] close",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
}

fn centered_popup(f: &Frame, width_pct: u16, height: u16) -> Rect {
    let area = f.area();
    let popup_w = (area.width * width_pct / 100).max(40);
    let popup_h = height.min(area.height.saturating_sub(4));
    let x = area.x + (area.width.saturating_sub(popup_w)) / 2;
    let y = area.y + (area.height.saturating_sub(popup_h)) / 2;
    Rect { x, y, width: popup_w, height: popup_h }
}

fn render_location_picker(f: &mut Frame, app: &App) {
    let height = (app.locations.len() as u16 + 6).min(20);
    let area = centered_popup(f, 60, height);
    f.render_widget(Clear, area);

    let block = Block::default().borders(Borders::ALL).title(" Locations ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let list_height = inner.height.saturating_sub(2) as usize; // reserve 2 rows for hints
    let _ = list_height;

    let mut lines: Vec<Line> = app.locations.iter().enumerate().map(|(i, loc)| {
        let marker = if i == app.picker_sel { "►" } else { " " };
        let text = format!(" {} {} ({:.4}, {:.4})", marker, loc.name, loc.lat, loc.lon);
        if i == app.picker_sel {
            Line::from(Span::styled(text, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
        } else {
            Line::from(text)
        }
    }).collect();

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " [↑/↓]select  [Enter]use  [n]add  [d]delete  [Esc]cancel",
        Style::default().fg(Color::DarkGray),
    )));

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
}

fn render_add_location(f: &mut Frame, app: &App) {
    let area = centered_popup(f, 60, 12);
    f.render_widget(Clear, area);

    let block = Block::default().borders(Borders::ALL).title(" Add Location ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let draft: &NewLocationDraft = app.new_loc_draft.as_ref().unwrap();
    let field_names = ["Name", "Lat", "Lon", "Height (m)"];
    let mut lines: Vec<Line> = field_names.iter().enumerate().map(|(i, label)| {
        let cursor = if i == draft.field { "_" } else { "" };
        let text = format!(" {}: {}{}", label, draft.bufs[i], cursor);
        if i == draft.field {
            Line::from(Span::styled(text, Style::default().fg(Color::Yellow)))
        } else {
            Line::from(text)
        }
    }).collect();

    lines.push(Line::from(""));
    if let Some(err) = &draft.error {
        lines.push(Line::from(Span::styled(
            format!(" Error: {}", err),
            Style::default().fg(Color::Red),
        )));
    } else {
        lines.push(Line::from(""));
    }
    lines.push(Line::from(Span::styled(
        " [Tab/↓]next  [↑]prev  [Enter]next/confirm  [Esc]cancel",
        Style::default().fg(Color::DarkGray),
    )));

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
}

fn render_almanac_body_picker(f: &mut Frame, app: &App) {
    let n = app.almanac.tracks.len() as u16;
    let height = n + 4;
    let area = centered_popup(f, 35, height);
    f.render_widget(Clear, area);

    let block = Block::default().borders(Borders::ALL).title(" Bodies [b] ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = app.almanac.tracks.iter().enumerate().map(|(i, track)| {
        let checked = app.selected_bodies.get(i).copied().unwrap_or(true);
        let check = if checked { "✓" } else { " " };
        let text = format!(" [{}] {} {}", check, track.symbol, track.name);
        let (r, g, b) = track.color_rgb;
        if i == app.almanac_picker_sel {
            Line::from(vec![
                Span::styled(text, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("  ■", Style::default().fg(Color::Rgb(r, g, b))),
            ])
        } else if checked {
            Line::from(Span::styled(text, Style::default().fg(Color::Rgb(r, g, b))))
        } else {
            Line::from(Span::styled(text, Style::default().fg(Color::DarkGray)))
        }
    }).collect();

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Space toggle · Esc close",
        Style::default().fg(Color::DarkGray),
    )));

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
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
    ];

    if app.almanac_show_times {
        text.push(Line::from(Span::styled(
            "  Rise  Trans  Set ",
            Style::default().add_modifier(Modifier::BOLD),
        )));

        let fmt = |dt: Option<chrono::DateTime<chrono::Utc>>| -> String {
            match dt {
                None => "--:--".to_string(),
                Some(utc) => {
                    if let Some(tz) = app.timezone {
                        utc.with_timezone(&tz).format("%H:%M").to_string()
                    } else {
                        utc.format("%H:%M").to_string()
                    }
                }
            }
        };

        for (i, track) in app.almanac.tracks.iter().enumerate() {
            let visible = app.selected_bodies.get(i).copied().unwrap_or(true);
            let (r, g, b) = track.color_rgb;

            let all_down = track.altitudes.iter().all(|&a| a <= 0.0);
            let all_up = track.altitudes.iter().all(|&a| a > 0.0);

            let label = if all_down {
                format!(" {} {} below horizon", track.symbol, track.name)
            } else if all_up {
                let max_alt = track.transit_alt.map(|a| format!(" ({:.0}°)", a)).unwrap_or_default();
                format!(" {} {} always up{}", track.symbol, track.name, max_alt)
            } else {
                format!(
                    " {} {} {} {}",
                    track.symbol,
                    fmt(track.rise),
                    fmt(track.transit),
                    fmt(track.set),
                )
            };
            let style = if visible {
                Style::default().fg(Color::Rgb(r, g, b))
            } else {
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM)
            };
            text.push(Line::from(Span::styled(label, style)));
        }
    } else {
        text.push(Line::from(Span::styled(
            " Body  Alt",
            Style::default().add_modifier(Modifier::BOLD),
        )));

        for (i, track) in app.almanac.tracks.iter().enumerate() {
            let visible = app.selected_bodies.get(i).copied().unwrap_or(true);
            let alt = track.altitudes[app.almanac.current_step];
            let (r, g, b) = track.color_rgb;
            let label = if alt > 0.0 {
                format!(" {} {} {:.1}°", track.symbol, track.name, alt)
            } else {
                format!(" {} {} below", track.symbol, track.name)
            };
            let style = if visible {
                Style::default().fg(Color::Rgb(r, g, b))
            } else {
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM)
            };
            text.push(Line::from(Span::styled(label, style)));
        }
    }

    let title = if app.almanac_show_times { " Times [t] " } else { " Legend [t] " };
    let para = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(para, area);
}

fn score_stars(score: f64) -> &'static str {
    match (score * 5.0) as u8 {
        5 => "★★★★★",
        4 => "★★★★☆",
        3 => "★★★☆☆",
        2 => "★★☆☆☆",
        _ => "★☆☆☆☆",
    }
}

fn score_color(score: f64) -> Color {
    if score >= 0.7 { Color::Green } else if score >= 0.4 { Color::Yellow } else { Color::Red }
}

fn render_best_targets(f: &mut Frame, app: &App, area: Rect) {
    let rows_area = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
    ]).split(area);

    let bt = &app.best_targets;

    // Header
    let night_str = if let (Some(ns), Some(ne)) = (bt.night_start, bt.night_end) {
        let fmt_dt = |dt: chrono::DateTime<chrono::Utc>| {
            if let Some(tz) = app.timezone {
                dt.with_timezone(&tz).format("%H:%M %Z").to_string()
            } else {
                dt.format("%H:%M UTC").to_string()
            }
        };
        format!("Astronomical Night: {}–{}", fmt_dt(ns), fmt_dt(ne))
    } else {
        "No astronomical night — showing best of 24h".to_string()
    };
    let header_text = format!(" {}   mag ≤ {:.1}  [+/-]", night_str, app.max_mag);
    let header_para = Paragraph::new(Line::from(Span::styled(
        header_text,
        Style::default().fg(Color::DarkGray),
    )));
    f.render_widget(header_para, rows_area[0]);

    if bt.targets.is_empty() {
        let para = Paragraph::new("No observable targets found tonight.")
            .block(Block::default().borders(Borders::ALL).title(" Best Targets "));
        f.render_widget(para, rows_area[1]);
        return;
    }

    let header_row = Row::new(vec!["#", "Sym", "Object", "Type", "Mag", "Peak Alt", "Time", "Moon Sep", "Score"])
        .style(Style::default().add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = bt.targets.iter().enumerate().map(|(i, t)| {
        let sc = score_color(t.score);
        let obj = if t.name.is_empty() {
            t.catalog.to_string()
        } else {
            format!("{} {}", t.catalog, t.name)
        };
        let time_str = if let Some(tz) = app.timezone {
            t.peak_time_utc.with_timezone(&tz).format("%H:%M").to_string()
        } else {
            t.peak_time_utc.format("%H:%M").to_string()
        };
        Row::new(vec![
            ratatui::text::Text::from(Span::styled(format!("{}", i + 1), Style::default().fg(Color::DarkGray))),
            ratatui::text::Text::from(Span::styled(t.symbol, Style::default().fg(sc))),
            ratatui::text::Text::from(Span::styled(obj, Style::default().fg(sc))),
            ratatui::text::Text::from(Span::styled(t.kind_label, Style::default().fg(Color::DarkGray))),
            ratatui::text::Text::from(Span::styled(format!("{:.1}", t.mag), Style::default().fg(Color::DarkGray))),
            ratatui::text::Text::from(Span::styled(format!("{:.1}°", t.peak_alt_deg), Style::default().fg(sc))),
            ratatui::text::Text::from(Span::styled(time_str, Style::default().fg(Color::DarkGray))),
            ratatui::text::Text::from(Span::styled(format!("{:.1}°", t.moon_sep_deg), Style::default().fg(Color::DarkGray))),
            ratatui::text::Text::from(Span::styled(score_stars(t.score), Style::default().fg(sc))),
        ])
    }).collect();

    let widths = [
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(22),
        Constraint::Length(14),
        Constraint::Length(5),
        Constraint::Length(9),
        Constraint::Length(6),
        Constraint::Length(9),
        Constraint::Length(11),
    ];

    let table = Table::new(rows, widths)
        .header(header_row)
        .block(Block::default().borders(Borders::ALL).title(" Best Targets "))
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    let mut state = TableState::default().with_selected(Some(app.best_targets_scroll));
    f.render_stateful_widget(table, rows_area[1], &mut state);
}

fn render_conjunctions(f: &mut Frame, app: &App, area: Rect) {
    let rows_area = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
    ]).split(area);

    let cj = &app.conjunctions;

    let fmt_dt = |dt: chrono::DateTime<chrono::Utc>| -> String {
        if let Some(tz) = app.timezone {
            dt.with_timezone(&tz).format("%b %d %H:%M %Z").to_string()
        } else {
            dt.format("%b %d %H:%M UTC").to_string()
        }
    };

    let header_text = format!(
        " Conjunctions ±7 days (sep < 5°) — {} to {}",
        fmt_dt(cj.scan_start),
        fmt_dt(cj.scan_end)
    );
    let header_para = Paragraph::new(Line::from(Span::styled(
        header_text,
        Style::default().fg(Color::DarkGray),
    )));
    f.render_widget(header_para, rows_area[0]);

    if cj.events.is_empty() {
        let para = Paragraph::new("No conjunctions within 5° in the ±7 day window.")
            .block(Block::default().borders(Borders::ALL).title(" Conjunctions "));
        f.render_widget(para, rows_area[1]);
        return;
    }

    // Find the event nearest to app.datetime for highlighting
    let nearest_idx = cj.events.iter().enumerate().min_by_key(|(_, ev)| {
        (ev.time_utc - app.datetime).num_seconds().abs()
    }).map(|(i, _)| i).unwrap_or(0);

    let header_row = Row::new(vec!["Time", "Bodies", "Sep°", "Visibility"])
        .style(Style::default().add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = cj.events.iter().enumerate().map(|(i, ev)| {
        let time_str = fmt_dt(ev.time_utc);

        let bodies_str = format!("{} {} – {} {}", ev.symbol_a, ev.body_a, ev.symbol_b, ev.body_b);

        let sep_str = format!("{:.2}°", ev.sep_deg);
        let sep_color = if ev.sep_deg < 1.0 {
            Color::Green
        } else if ev.sep_deg < 3.0 {
            Color::Yellow
        } else {
            Color::Reset
        };

        let (vis_str, vis_color) = if ev.alt_a >= 0.0 && ev.alt_b >= 0.0 {
            (format!("Both visible {:.0}°/{:.0}°", ev.alt_a, ev.alt_b), Color::Green)
        } else if ev.alt_a >= 0.0 {
            (format!("{} above {:.0}°", ev.body_a, ev.alt_a), Color::Yellow)
        } else if ev.alt_b >= 0.0 {
            (format!("{} above {:.0}°", ev.body_b, ev.alt_b), Color::Yellow)
        } else {
            ("Below horizon".to_string(), Color::DarkGray)
        };

        let row_style = if i == nearest_idx {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        };

        Row::new(vec![
            ratatui::text::Text::from(Span::styled(time_str, Style::default().fg(Color::Reset))),
            ratatui::text::Text::from(Span::styled(bodies_str, Style::default().fg(Color::White))),
            ratatui::text::Text::from(Span::styled(sep_str, Style::default().fg(sep_color))),
            ratatui::text::Text::from(Span::styled(vis_str, Style::default().fg(vis_color))),
        ]).style(row_style)
    }).collect();

    let widths = [
        Constraint::Length(22),
        Constraint::Min(28),
        Constraint::Length(7),
        Constraint::Min(24),
    ];

    let table = Table::new(rows, widths)
        .header(header_row)
        .block(Block::default().borders(Borders::ALL).title(" Conjunctions "))
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    let mut state = TableState::default().with_selected(Some(app.conjunctions_scroll));
    f.render_stateful_widget(table, rows_area[1], &mut state);
}
