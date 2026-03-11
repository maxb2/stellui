use chrono::{DateTime, Utc};

use crate::sky::{self, RenderedPlanet, RenderedStar, SunMoonInfo};
use crate::weather::HourlyForecast;

pub enum Tab {
    Sky,
    Weather,
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
    pub max_mag: f64,

    pub test_mode: bool,

    pub stars: Vec<RenderedStar>,
    pub sun_moon: SunMoonInfo,
    pub planets: Vec<RenderedPlanet>,

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
            max_mag: 5.5,
            test_mode: false,
            stars: Vec::new(),
            planets: Vec::new(),
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
    }
}
