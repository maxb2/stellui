use astronomy_engine_bindings::{
    Astronomy_Ecliptic, Astronomy_Equator, Astronomy_HelioVector, Astronomy_Horizon,
    Astronomy_Illumination, Astronomy_SearchRiseSetEx, astro_aberration_t_ABERRATION,
    astro_body_t_BODY_EARTH, astro_body_t_BODY_JUPITER, astro_body_t_BODY_MARS,
    astro_body_t_BODY_MERCURY, astro_body_t_BODY_MOON, astro_body_t_BODY_NEPTUNE,
    astro_body_t_BODY_SATURN, astro_body_t_BODY_SUN, astro_body_t_BODY_URANUS,
    astro_body_t_BODY_VENUS, astro_direction_t_DIRECTION_RISE, astro_direction_t_DIRECTION_SET,
    astro_equator_date_t_EQUATOR_OF_DATE, astro_observer_t, astro_refraction_t_REFRACTION_NORMAL,
    astro_status_t_ASTRO_SUCCESS, astro_time_t,
};
use chrono::{DateTime, Duration, TimeZone, Utc};
use stellui::astro::{
    CartesianCoordinates, PolarCoordinates, SunMoonProjection, astro_time_from_datetime,
    hor_to_stereo, star_stereo,
};
use stellui::catalog;
use stellui::dso::{self, DsoKind};

pub struct RenderedStar {
    pub x: f64,
    pub y: f64,
    pub mag: f64,
    pub alt: f64, // degrees, altitude above horizon (guaranteed >= 0)
    pub az: f64,  // degrees, 0=N 90=E
}

pub struct RenderedPlanet {
    pub name: &'static str,
    pub symbol: &'static str,
    pub x: f64,
    pub y: f64,
    pub mag: f64,
    pub alt: f64,
    pub az: f64,
}

pub struct SunMoonInfo {
    pub sun_stereo: Option<PolarCoordinates>,
    pub moon_stereo: Option<PolarCoordinates>,
    pub moon_cycle_degrees: f64,
    pub sun_alt: f64,
    pub sun_az: f64,
    pub moon_alt: f64,
    pub moon_az: f64,
}

pub fn compute_stars(
    lat: f64,
    lon: f64,
    height: f64,
    datetime: DateTime<Utc>,
    max_mag: f64,
    orion_only: bool,
) -> Vec<RenderedStar> {
    let observer = astro_observer_t {
        latitude: lat,
        longitude: lon,
        height,
    };
    let mut time = astro_time_from_datetime(datetime);

    catalog::J2000_CATALOG
        .iter()
        .filter(|s| {
            if orion_only {
                s.ra >= 4.7 && s.ra <= 6.3 && s.dec >= -11.0 && s.dec <= 23.0
            } else {
                true
            }
        })
        .filter(|s| s.mag <= max_mag)
        .filter_map(|star| {
            let polar = star_stereo(star, &mut time, &observer, true, true).ok()?;
            if polar.rad > 2.0 {
                return None;
            }
            let alt = 90.0 - 2.0 * (polar.rad / 2.0).atan().to_degrees();
            let az = polar.phi;
            let oriented = polar.canvas_orient_for(lat < 0.0);
            let cart = CartesianCoordinates::from(oriented);
            Some(RenderedStar {
                x: cart.x,
                y: cart.y,
                mag: star.mag,
                alt,
                az,
            })
        })
        .collect()
}

pub fn compute_planets(
    lat: f64,
    lon: f64,
    height: f64,
    datetime: DateTime<Utc>,
) -> Vec<RenderedPlanet> {
    const PLANETS: &[(&str, &str, i32)] = &[
        ("Mercury", "⚪☿", astro_body_t_BODY_MERCURY),
        ("Venus", "🟡♀", astro_body_t_BODY_VENUS),
        ("Mars", "🔴♂", astro_body_t_BODY_MARS),
        ("Jupiter", "🟠♃", astro_body_t_BODY_JUPITER),
        ("Saturn", "🪐♄", astro_body_t_BODY_SATURN),
        ("Uranus", "🔵⛢", astro_body_t_BODY_URANUS),
        ("Neptune", "🔵♆", astro_body_t_BODY_NEPTUNE),
    ];

    let observer = astro_observer_t {
        latitude: lat,
        longitude: lon,
        height,
    };
    let mut time = astro_time_from_datetime(datetime);

    PLANETS
        .iter()
        .filter_map(|&(name, symbol, body)| unsafe {
            let eq = Astronomy_Equator(
                body,
                &mut time as *mut _,
                observer,
                astro_equator_date_t_EQUATOR_OF_DATE,
                astro_aberration_t_ABERRATION,
            );
            if eq.status != astro_status_t_ASTRO_SUCCESS {
                return None;
            }

            let hor = Astronomy_Horizon(
                &mut time as *mut _,
                observer,
                eq.ra,
                eq.dec,
                astro_refraction_t_REFRACTION_NORMAL,
            );

            let polar = hor_to_stereo(&hor);
            if polar.rad > 2.0 {
                return None;
            }

            let oriented = polar.canvas_orient_for(lat < 0.0);
            let cart = CartesianCoordinates::from(oriented);

            let illum = Astronomy_Illumination(body, time);
            let mag = if illum.status == astro_status_t_ASTRO_SUCCESS {
                illum.mag
            } else {
                99.0
            };

            Some(RenderedPlanet {
                name,
                symbol,
                x: cart.x,
                y: cart.y,
                mag,
                alt: hor.altitude,
                az: hor.azimuth,
            })
        })
        .collect()
}

pub struct OrreryPlanet {
    pub name: &'static str,
    pub symbol: &'static str,
    pub cx: f64,
    pub cy: f64,
    pub dist_au: f64,
}

pub struct OrreryInfo {
    pub planets: Vec<OrreryPlanet>,
}

pub const PLANET_SEMI_MAJOR_AXES: &[(&str, f64)] = &[
    ("Mercury", 0.387),
    ("Venus", 0.723),
    ("Earth", 1.000),
    ("Mars", 1.524),
    ("Jupiter", 5.203),
    ("Saturn", 9.537),
    ("Uranus", 19.191),
    ("Neptune", 30.069),
];

pub fn orrery_scale(au: f64) -> f64 {
    au.sqrt()
}

pub fn compute_orrery(datetime: DateTime<Utc>) -> OrreryInfo {
    const BODIES: &[(&str, &str, i32)] = &[
        ("Mercury", "⚪☿", astro_body_t_BODY_MERCURY),
        ("Venus", "🟡♀", astro_body_t_BODY_VENUS),
        ("Earth", "🌍♁", astro_body_t_BODY_EARTH),
        ("Mars", "🔴♂", astro_body_t_BODY_MARS),
        ("Jupiter", "🟠♃", astro_body_t_BODY_JUPITER),
        ("Saturn", "🪐♄", astro_body_t_BODY_SATURN),
        ("Uranus", "🔵⛢", astro_body_t_BODY_URANUS),
        ("Neptune", "🔵♆", astro_body_t_BODY_NEPTUNE),
    ];
    let time = astro_time_from_datetime(datetime);
    let planets = BODIES
        .iter()
        .filter_map(|&(name, symbol, body)| unsafe {
            let helio = Astronomy_HelioVector(body, time);
            if helio.status != astro_status_t_ASTRO_SUCCESS {
                return None;
            }
            let ecl = Astronomy_Ecliptic(helio);
            if ecl.status != astro_status_t_ASTRO_SUCCESS {
                return None;
            }
            let raw_x = ecl.vec.x;
            let raw_y = ecl.vec.y;
            let r_au = (raw_x * raw_x + raw_y * raw_y).sqrt();
            let scale = if r_au > 1e-9 {
                orrery_scale(r_au) / r_au
            } else {
                0.0
            };
            Some(OrreryPlanet {
                name,
                symbol,
                cx: raw_x * scale,
                cy: raw_y * scale,
                dist_au: r_au,
            })
        })
        .collect();
    OrreryInfo { planets }
}

pub const ALMANAC_STEPS: usize = 96; // 15-min intervals over 24h

pub struct AlmanacTrack {
    pub name: &'static str,
    pub symbol: &'static str,
    pub color_rgb: (u8, u8, u8),
    /// altitude in degrees (-90..90) for each step; index 0 = local midnight
    pub altitudes: [f64; ALMANAC_STEPS],
    pub rise: Option<DateTime<Utc>>,
    pub transit: Option<DateTime<Utc>>,
    pub transit_alt: Option<f64>,
    pub set: Option<DateTime<Utc>>,
}

fn astro_time_to_utc(t: astro_time_t) -> DateTime<Utc> {
    // t.ut = days since J2000.0 (2000-01-01 12:00:00 UTC)
    use chrono::TimeZone;
    let j2000 = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let micros = (t.ut * 86_400.0 * 1_000_000.0) as i64;
    j2000 + Duration::microseconds(micros)
}

#[allow(clippy::type_complexity)]
fn compute_rise_set_transit(
    body: i32,
    observer: astro_observer_t,
    day_start: DateTime<Utc>,
    altitudes: &[f64; ALMANAC_STEPS],
    height: f64,
) -> (Option<DateTime<Utc>>, Option<DateTime<Utc>>, Option<DateTime<Utc>>, Option<f64>) {
    let all_up = altitudes.iter().all(|&a| a > 0.0);
    let all_down = altitudes.iter().all(|&a| a <= 0.0);

    // Transit: peak of altitude array with parabolic interpolation
    let peak_idx = altitudes
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0);

    let transit_alt = altitudes[peak_idx];

    let offset = if peak_idx > 0 && peak_idx < ALMANAC_STEPS - 1 {
        let y0 = altitudes[peak_idx - 1];
        let y1 = altitudes[peak_idx];
        let y2 = altitudes[peak_idx + 1];
        let denom = 2.0 * (2.0 * y1 - y0 - y2);
        if denom.abs() > 1e-9 { (y0 - y2) / denom } else { 0.0 }
    } else {
        0.0
    };

    let transit_mins = (peak_idx as f64 + offset) * 15.0;
    let transit = Some(day_start + Duration::seconds((transit_mins * 60.0) as i64));

    if all_up || all_down {
        return (None, transit, None, Some(transit_alt));
    }

    let start_time = astro_time_from_datetime(day_start);

    let rise = unsafe {
        let result = Astronomy_SearchRiseSetEx(
            body,
            observer,
            astro_direction_t_DIRECTION_RISE,
            start_time,
            1.0,
            height,
        );
        if result.status == astro_status_t_ASTRO_SUCCESS {
            Some(astro_time_to_utc(result.time))
        } else {
            None
        }
    };

    let set = unsafe {
        let result = Astronomy_SearchRiseSetEx(
            body,
            observer,
            astro_direction_t_DIRECTION_SET,
            start_time,
            1.0,
            height,
        );
        if result.status == astro_status_t_ASTRO_SUCCESS {
            Some(astro_time_to_utc(result.time))
        } else {
            None
        }
    };

    (rise, transit, set, Some(transit_alt))
}

pub struct AlmanacInfo {
    pub tracks: Vec<AlmanacTrack>,
    /// Step index corresponding to app.datetime (0..ALMANAC_STEPS)
    pub current_step: usize,
}

type BodySpec = (&'static str, &'static str, i32, (u8, u8, u8));

pub fn compute_almanac(lat: f64, lon: f64, height: f64, datetime: DateTime<Utc>, tz: Option<chrono_tz::Tz>) -> AlmanacInfo {
    const BODIES: &[BodySpec] = &[
        ("Sun",     "☀",  astro_body_t_BODY_SUN,     (255, 220,  50)),
        ("Moon",    "☽",  astro_body_t_BODY_MOON,    (200, 200, 200)),
        ("Mercury", "☿",  astro_body_t_BODY_MERCURY, (180, 180, 180)),
        ("Venus",   "♀",  astro_body_t_BODY_VENUS,   (230, 220, 100)),
        ("Mars",    "♂",  astro_body_t_BODY_MARS,    (220,  80,  60)),
        ("Jupiter", "♃",  astro_body_t_BODY_JUPITER, (240, 200, 150)),
        ("Saturn",  "♄",  astro_body_t_BODY_SATURN,  (200, 180, 100)),
        ("Uranus",  "⛢",  astro_body_t_BODY_URANUS,  (100, 220, 220)),
        ("Neptune", "♆",  astro_body_t_BODY_NEPTUNE, ( 80, 120, 220)),
    ];
    let observer = astro_observer_t { latitude: lat, longitude: lon, height };
    let (day_start, current_step) = match tz {
        Some(tz) => {
            let local_now = datetime.with_timezone(&tz);
            let midnight_naive = local_now.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let day_start = tz.from_local_datetime(&midnight_naive).unwrap().to_utc();
            let step = ((datetime - day_start).num_minutes().max(0) as usize / 15)
                .min(ALMANAC_STEPS - 1);
            (day_start, step)
        }
        None => {
            let day_start = datetime.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
            let step = ((datetime - day_start).num_minutes().max(0) as usize / 15)
                .min(ALMANAC_STEPS - 1);
            (day_start, step)
        }
    };

    let tracks = BODIES.iter().map(|&(name, symbol, body, color_rgb)| {
        let mut altitudes = [0f64; ALMANAC_STEPS];
        for (i, alt) in altitudes.iter_mut().enumerate() {
            let step_dt = day_start + Duration::minutes(i as i64 * 15);
            let mut time = astro_time_from_datetime(step_dt);
            *alt = unsafe {
                let eq = Astronomy_Equator(
                    body,
                    &mut time as *mut _,
                    observer,
                    astro_equator_date_t_EQUATOR_OF_DATE,
                    astro_aberration_t_ABERRATION,
                );
                if eq.status != astro_status_t_ASTRO_SUCCESS {
                    -90.0
                } else {
                    let hor = Astronomy_Horizon(
                        &mut time as *mut _,
                        observer,
                        eq.ra,
                        eq.dec,
                        astro_refraction_t_REFRACTION_NORMAL,
                    );
                    hor.altitude
                }
            };
        }
        let (rise, transit, set, transit_alt) =
            compute_rise_set_transit(body, observer, day_start, &altitudes, height);
        AlmanacTrack { name, symbol, color_rgb, altitudes, rise, transit, transit_alt, set }
    }).collect();

    AlmanacInfo { tracks, current_step }
}

#[allow(dead_code)]
pub struct RenderedDso {
    pub catalog: &'static str,
    pub name: &'static str,
    pub kind: DsoKind,
    pub x: f64,
    pub y: f64,
    pub alt: f64,
    pub az: f64,
    pub mag: f64,
}

pub fn compute_dsos(
    lat: f64,
    lon: f64,
    height: f64,
    datetime: DateTime<Utc>,
) -> Vec<RenderedDso> {
    let observer = astro_observer_t {
        latitude: lat,
        longitude: lon,
        height,
    };
    let mut time = astro_time_from_datetime(datetime);
    let southern = lat < 0.0;

    dso::MESSIER
        .iter()
        .filter_map(|d| {
            // Reuse star_stereo by constructing a temporary Star with the DSO's RA/Dec
            let fake_star = catalog::Star { id: 0, ra: d.ra, dec: d.dec, mag: d.mag };
            let polar = star_stereo(&fake_star, &mut time, &observer, true, true).ok()?;
            // Keep below-horizon DSOs (alt < 0) so search can find them; rendering filters by alt.
            let alt = 90.0 - 2.0 * (polar.rad / 2.0).atan().to_degrees();
            let az = polar.phi;
            let oriented = polar.canvas_orient_for(southern);
            let cart = CartesianCoordinates::from(oriented);
            Some(RenderedDso {
                catalog: d.catalog,
                name: d.name,
                kind: d.kind,
                x: cart.x,
                y: cart.y,
                alt,
                az,
                mag: d.mag,
            })
        })
        .collect()
}

pub fn compute_sun_moon(lat: f64, lon: f64, height: f64, datetime: DateTime<Utc>) -> SunMoonInfo {
    let observer = astro_observer_t {
        latitude: lat,
        longitude: lon,
        height,
    };
    let mut time = astro_time_from_datetime(datetime);
    let smp = SunMoonProjection::from_time_observer(&mut time, &observer);

    let to_opt = |hor: &astronomy_engine_bindings::astro_horizon_t| {
        let p = hor_to_stereo(hor).canvas_orient_for(lat < 0.0);
        if p.rad <= 2.0 { Some(p) } else { None }
    };

    SunMoonInfo {
        sun_stereo: to_opt(&smp.sun_hor),
        moon_stereo: to_opt(&smp.moon_hor),
        moon_cycle_degrees: smp.moon_cycle_degrees,
        sun_alt: smp.sun_hor.altitude,
        sun_az: smp.sun_hor.azimuth,
        moon_alt: smp.moon_hor.altitude,
        moon_az: smp.moon_hor.azimuth,
    }
}
