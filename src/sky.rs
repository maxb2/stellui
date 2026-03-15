use astronomy_engine_bindings::{
    Astronomy_Ecliptic, Astronomy_Equator, Astronomy_HelioVector, Astronomy_Horizon,
    Astronomy_Illumination, astro_aberration_t_ABERRATION, astro_body_t_BODY_EARTH,
    astro_body_t_BODY_JUPITER, astro_body_t_BODY_MARS, astro_body_t_BODY_MERCURY,
    astro_body_t_BODY_MOON, astro_body_t_BODY_NEPTUNE, astro_body_t_BODY_SATURN,
    astro_body_t_BODY_SUN, astro_body_t_BODY_URANUS, astro_body_t_BODY_VENUS,
    astro_equator_date_t_EQUATOR_OF_DATE, astro_observer_t, astro_refraction_t_REFRACTION_NORMAL,
    astro_status_t_ASTRO_SUCCESS,
};
use chrono::{DateTime, Duration, TimeZone, Utc};
use stellui::astro::{
    CartesianCoordinates, PolarCoordinates, SunMoonProjection, astro_time_from_datetime,
    hor_to_stereo, star_stereo,
};
use stellui::catalog;

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
    /// altitude in degrees (-90..90) for each step; index 0 = UTC midnight
    pub altitudes: [f64; ALMANAC_STEPS],
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
        AlmanacTrack { name, symbol, color_rgb, altitudes }
    }).collect();

    AlmanacInfo { tracks, current_step }
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
