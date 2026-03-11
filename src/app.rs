use std::time::Instant;

use chrono::{DateTime, Utc};

use crate::sky::{self, OrreryInfo, RenderedPlanet, RenderedStar, SunMoonInfo};
use crate::weather::HourlyForecast;

/// Sky tab: seconds → days. Default index 6 = 1x real-time.
pub const SKY_SPEED_PRESETS: &[(i64, &str)] = &[
    (-86400,  "-1d/s"),
    (-3600,   "-1h/s"),
    (-600,    "-10m/s"),
    (-60,     "-1m/s"),
    (-10,     "-10x"),
    (-1,      "-1x"),
    (1,       "1x"),     // index 6 — default
    (10,      "10x"),
    (60,      "1m/s"),
    (600,     "10m/s"),
    (3600,    "1h/s"),
    (86400,   "1d/s"),
];

/// Orrery tab: days → decades. Default index 5 = 1d/s.
pub const ORRERY_SPEED_PRESETS: &[(i64, &str)] = &[
    (-86400 * 365 * 10, "-10y/s"),
    (-86400 * 365,      "-1y/s"),
    (-86400 * 30,       "-1mo/s"),
    (-86400 * 7,        "-1w/s"),
    (-86400,            "-1d/s"),
    (86400,             "1d/s"),   // index 5 — default
    (86400 * 7,         "1w/s"),
    (86400 * 30,        "1mo/s"),
    (86400 * 365,       "1y/s"),
    (86400 * 365 * 10,  "10y/s"),
];

pub enum Tab {
    Sky,
    Weather,
    SolarSystem,
}

pub enum InputMode {
    Normal,
    EditingLat,
    EditingLon,
    EditingDatetime,
}

pub struct App {
    pub tab: Tab,
    pub input_mode: InputMode,
    pub lat: f64,
    pub lon: f64,
    pub height: f64,
    pub input_buf: String,
    pub datetime: DateTime<Utc>,
    pub live_mode: bool,
    pub sky_speed_index: usize,
    pub orrery_speed_index: usize,
    pub time_paused: bool,
    pub last_tick: Instant,
    pub max_mag: f64,

    pub test_mode: bool,

    pub stars: Vec<RenderedStar>,
    pub sun_moon: SunMoonInfo,
    pub planets: Vec<RenderedPlanet>,
    pub orrery: OrreryInfo,

    pub forecasts: Option<Vec<HourlyForecast>>,
    pub weather_loading: bool,
    pub weather_error: Option<String>,
    pub weather_scroll: usize,
}

impl App {
    pub fn new(lat: f64, lon: f64, height: f64) -> Self {
        let mut app = Self {
            tab: Tab::Sky,
            input_mode: InputMode::Normal,
            lat,
            lon,
            height,
            input_buf: String::new(),
            datetime: Utc::now(),
            live_mode: false,
            sky_speed_index: 6,
            orrery_speed_index: 5,
            time_paused: false,
            last_tick: Instant::now(),
            max_mag: 5.5,
            test_mode: false,
            stars: Vec::new(),
            planets: Vec::new(),
            orrery: OrreryInfo { planets: Vec::new() },
            sun_moon: SunMoonInfo {
                sun_stereo: None,
                moon_stereo: None,
                moon_cycle_degrees: 0.0,
            },
            forecasts: None,
            weather_loading: false,
            weather_error: None,
            weather_scroll: 0,
        };
        app.recompute();
        app
    }

    pub fn recompute(&mut self) {
        self.stars = sky::compute_stars(
            self.lat,
            self.lon,
            self.height,
            self.datetime,
            self.max_mag,
            self.test_mode,
        );
        self.sun_moon = sky::compute_sun_moon(self.lat, self.lon, self.height, self.datetime);
        self.planets = sky::compute_planets(self.lat, self.lon, self.height, self.datetime);
        self.orrery = sky::compute_orrery(self.datetime);
    }
}
