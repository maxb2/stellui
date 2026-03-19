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

use crate::sky::{self, AlmanacInfo, BestTargetsInfo, ConjunctionsInfo, OrreryInfo, RenderedDso, RenderedJupiterMoon, RenderedPlanet, RenderedStar, SunMoonInfo};
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
    Targets,
    Conjunctions,
}

pub enum InputMode {
    Normal,
    LocationPicker,
    AddingLocation,
    EditingDatetime,
    EditingTimezone,
    AlmanacBodyPicker,
    FovInput,
    ObjectSearch,
    EyepieceCalc,
}

pub struct NewLocationDraft {
    pub bufs: [String; 4], // [name, lat, lon, height]
    pub field: usize,
    pub error: Option<String>,
}

pub struct FovDraft {
    pub bufs: [String; 3], // [alt_deg, az_deg, fov_deg]
    pub field: usize,
    pub error: Option<String>,
}

/// State for the add-scope / add-eyepiece sub-form inside the eyepiece calculator.
pub struct CalcDraft {
    /// `false` = adding a scope, `true` = adding an eyepiece.
    pub adding_eyepiece: bool,
    /// Field buffers: [name, value1, value2]
    ///   scope:    [name, aperture_mm, focal_length_mm]
    ///   eyepiece: [name, focal_length_mm, afov_deg]
    pub bufs: [String; 3],
    pub field: usize,
    pub error: Option<String>,
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

    pub locations: Vec<crate::config::Location>,
    pub location_index: usize,
    pub picker_sel: usize,
    pub new_loc_draft: Option<NewLocationDraft>,

    pub stars: Vec<RenderedStar>,
    pub sun_moon: SunMoonInfo,
    pub planets: Vec<RenderedPlanet>,
    pub jupiter_moons: Vec<RenderedJupiterMoon>,
    pub dsos: Vec<RenderedDso>,
    pub show_dsos: bool,
    pub orrery: OrreryInfo,
    pub almanac: AlmanacInfo,

    pub selected_bodies: Vec<bool>,
    pub almanac_picker_sel: usize,
    pub almanac_show_times: bool,

    pub best_targets: BestTargetsInfo,
    pub best_targets_scroll: usize,
    pub best_targets_valid: bool,

    pub conjunctions: ConjunctionsInfo,
    pub conjunctions_scroll: usize,
    pub conjunctions_valid: bool,
    pub conjunctions_ref_time: DateTime<Utc>,

    pub forecasts: Option<Vec<HourlyForecast>>,
    pub weather_loading: bool,
    pub weather_error: Option<String>,
    pub weather_scroll: usize,

    pub fov_active: bool,
    pub fov_alt: f64,
    pub fov_az: f64,
    pub fov_deg: f64,
    pub fov_draft: Option<FovDraft>,

    pub search_query: String,
    pub search_sel: usize,

    pub scopes: Vec<crate::config::Scope>,
    pub scope_sel: usize,
    pub eyepieces: Vec<crate::config::Eyepiece>,
    pub ep_sel: usize,
    /// Which row is focused in the calc view: 0 = scope, 1 = eyepiece.
    pub calc_row: usize,
    pub calc_draft: Option<CalcDraft>,
}

impl App {
    pub fn new(
        locations: Vec<crate::config::Location>,
        scopes: Vec<crate::config::Scope>,
        eyepieces: Vec<crate::config::Eyepiece>,
        initial_index: usize,
        max_mag_override: Option<f64>,
    ) -> Self {
        let loc = &locations[initial_index];
        let timezone_override = loc.timezone.as_deref().and_then(|s| s.parse().ok());
        let lat = loc.lat;
        let lon = loc.lon;
        let height = loc.height;
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
            locations,
            location_index: initial_index,
            picker_sel: initial_index,
            new_loc_draft: None,
            stars: Vec::new(),
            planets: Vec::new(),
            jupiter_moons: Vec::new(),
            dsos: Vec::new(),
            show_dsos: true,
            orrery: OrreryInfo { planets: Vec::new() },
            almanac: AlmanacInfo { tracks: Vec::new(), current_step: 0 },
            selected_bodies: Vec::new(),
            almanac_picker_sel: 0,
            almanac_show_times: false,
            sun_moon: SunMoonInfo {
                sun_stereo: None,
                moon_stereo: None,
                moon_cycle_degrees: 0.0,
                sun_alt: 0.0,
                sun_az: 0.0,
                moon_alt: 0.0,
                moon_az: 0.0,
            },
            best_targets: BestTargetsInfo::default(),
            best_targets_scroll: 0,
            best_targets_valid: false,
            conjunctions: ConjunctionsInfo::default(),
            conjunctions_scroll: 0,
            conjunctions_valid: false,
            conjunctions_ref_time: Utc::now(),
            forecasts: None,
            weather_loading: false,
            weather_error: None,
            weather_scroll: 0,
            fov_active: false,
            fov_alt: 45.0,
            fov_az: 180.0,
            fov_deg: 30.0,
            fov_draft: None,
            search_query: String::new(),
            search_sel: 0,
            scopes,
            scope_sel: 0,
            eyepieces,
            ep_sel: 0,
            calc_row: 0,
            calc_draft: None,
        };
        app.recompute();
        app
    }

    pub fn switch_location(&mut self, index: usize) {
        if index >= self.locations.len() { return; }
        let loc = &self.locations[index];
        let timezone_override = loc.timezone.as_deref().and_then(|s| s.parse().ok());
        self.lat = loc.lat;
        self.lon = loc.lon;
        self.height = loc.height;
        self.timezone = timezone_override.or_else(|| resolve_tz(self.lat, self.lon));
        self.location_index = index;
        self.best_targets_valid = false;
        self.conjunctions_valid = false;
        self.recompute();
    }

    pub fn recompute(&mut self) {
        let drift = (self.datetime - self.conjunctions_ref_time).num_hours().abs();
        if drift >= 6 {
            self.conjunctions_valid = false;
        }

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
        self.jupiter_moons = sky::compute_jupiter_moons(self.lat, self.lon, self.height, self.datetime);
        self.dsos = sky::compute_dsos(self.lat, self.lon, self.height, self.datetime);
        self.orrery = sky::compute_orrery(self.datetime);
        if matches!(self.tab, Tab::Almanac) {
            self.almanac = sky::compute_almanac(self.lat, self.lon, self.height, self.datetime, self.timezone);
            while self.selected_bodies.len() < self.almanac.tracks.len() {
                self.selected_bodies.push(true);
            }
        }
        if matches!(self.tab, Tab::Targets) && !self.best_targets_valid {
            self.best_targets = sky::compute_best_targets(self.lat, self.lon, self.height, self.datetime, self.timezone, self.max_mag);
            self.best_targets_valid = true;
        }
        if matches!(self.tab, Tab::Conjunctions) && !self.conjunctions_valid {
            self.conjunctions = sky::compute_conjunctions(self.lat, self.lon, self.height, self.datetime);
            self.conjunctions_valid = true;
            self.conjunctions_ref_time = self.datetime;
        }
    }
}

/// Search result: (symbol, label, alt_deg, az_deg).
pub type SearchHit = (&'static str, String, f64, f64);

/// Collect objects matching `query` from DSOs, planets, Sun, and Moon.
/// Returns results sorted: above-horizon first, then alphabetically.
pub fn search_hits(app: &App, query: &str) -> Vec<SearchHit> {
    let q = query.to_lowercase();
    let mut hits: Vec<SearchHit> = Vec::new();

    // DSOs
    for dso in &app.dsos {
        let matches = q.is_empty()
            || dso.catalog.to_lowercase().contains(&q)
            || dso.name.to_lowercase().contains(&q);
        if matches {
            let label = if dso.name.is_empty() {
                dso.catalog.to_string()
            } else {
                format!("{}  {}", dso.catalog, dso.name)
            };
            use stellui::dso::DsoKind;
            let symbol = match dso.kind {
                DsoKind::Galaxy          => "⊙",
                DsoKind::OpenCluster     => "○",
                DsoKind::GlobularCluster => "⊕",
                DsoKind::Nebula          => "☁",
                DsoKind::PlanetaryNebula => "◎",
                DsoKind::SupernovaRemnant | DsoKind::Other => "✦",
            };
            hits.push((symbol, label, dso.alt, dso.az));
        }
    }

    // Planets
    for planet in &app.planets {
        if q.is_empty() || planet.name.to_lowercase().contains(&q) {
            hits.push((planet.symbol, planet.name.to_string(), planet.alt, planet.az));
        }
    }

    // Sun
    if q.is_empty() || "sun".contains(&q) {
        hits.push(("☀", "Sun".to_string(), app.sun_moon.sun_alt, app.sun_moon.sun_az));
    }

    // Moon
    if q.is_empty() || "moon".contains(&q) {
        hits.push(("☽", "Moon".to_string(), app.sun_moon.moon_alt, app.sun_moon.moon_az));
    }

    // Sort: above-horizon first, then alphabetically by label
    hits.sort_by(|a, b| {
        let a_up = a.2 >= 0.0;
        let b_up = b.2 >= 0.0;
        b_up.cmp(&a_up).then_with(|| a.1.cmp(&b.1))
    });

    hits
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
