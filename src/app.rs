use std::sync::OnceLock;
use std::time::Instant;

use chrono::{DateTime, Utc};
use tzf_rs::DefaultFinder;

static TZ_FINDER: OnceLock<DefaultFinder> = OnceLock::new();

pub fn resolve_tz(lat: f64, lon: f64) -> Option<chrono_tz::Tz> {
    let finder = TZ_FINDER.get_or_init(DefaultFinder::new);
    let name = finder.get_tz_name(lon, lat);
    if name.is_empty() { None } else { name.parse().ok() }
}

use crate::sky::{self, AlmanacInfo, OrreryInfo, RenderedPlanet, RenderedStar, SunMoonInfo};
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
    Almanac,
}

pub enum InputMode {
    Normal,
    EditingLat,
    EditingLon,
    EditingDatetime,
    EditingTimezone,
}

pub struct App {
    pub tab: Tab,
    pub input_mode: InputMode,
    pub lat: f64,
    pub lon: f64,
    pub height: f64,
    pub timezone: Option<chrono_tz::Tz>,
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
    pub almanac: AlmanacInfo,

    pub forecasts: Option<Vec<HourlyForecast>>,
    pub weather_loading: bool,
    pub weather_error: Option<String>,
    pub weather_scroll: usize,
}

impl App {
    pub fn new(lat: f64, lon: f64, height: f64, timezone_override: Option<chrono_tz::Tz>, max_mag_override: Option<f64>) -> Self {
        let timezone = timezone_override.or_else(|| resolve_tz(lat, lon));
        let mut app = Self {
            tab: Tab::Sky,
            input_mode: InputMode::Normal,
            lat,
            lon,
            height,
            timezone,
            input_buf: String::new(),
            datetime: Utc::now(),
            live_mode: false,
            sky_speed_index: 6,
            orrery_speed_index: 5,
            time_paused: false,
            last_tick: Instant::now(),
            max_mag: max_mag_override.unwrap_or(5.5),
            test_mode: false,
            stars: Vec::new(),
            planets: Vec::new(),
            orrery: OrreryInfo { planets: Vec::new() },
            almanac: AlmanacInfo { tracks: Vec::new(), current_step: 0 },
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
        if matches!(self.tab, Tab::Almanac) {
            self.almanac = sky::compute_almanac(self.lat, self.lon, self.height, self.datetime, self.timezone);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDateTime, TimeZone, Utc};

    #[test]
    fn central_time_dst_after_spring_forward() {
        // March 11, 2026 is after US spring-forward (March 8, 2026)
        // 21:00 UTC should be 16:00 CDT (UTC-5), not 15:00 CST (UTC-6)
        let utc = Utc.from_utc_datetime(
            &NaiveDateTime::parse_from_str("2026-03-11 21:00", "%Y-%m-%d %H:%M").unwrap()
        );
        let tz = "America/Chicago".parse::<chrono_tz::Tz>().unwrap();
        let local = utc.with_timezone(&tz);
        assert_eq!(local.format("%H:%M %Z").to_string(), "16:00 CDT");
    }

    #[test]
    fn resolve_tz_chicago() {
        // Chicago coordinates should resolve to America/Chicago
        let tz = resolve_tz(41.88, -87.63);
        assert!(tz.is_some());
        assert_eq!(tz.unwrap().name(), "America/Chicago");
    }
}
