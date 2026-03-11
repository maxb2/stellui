use astronomy_engine_bindings::{
    Astronomy_Equator, Astronomy_Horizon, Astronomy_Illumination, astro_aberration_t_ABERRATION,
    astro_body_t_BODY_JUPITER, astro_body_t_BODY_MARS, astro_body_t_BODY_MERCURY,
    astro_body_t_BODY_NEPTUNE, astro_body_t_BODY_SATURN, astro_body_t_BODY_URANUS,
    astro_body_t_BODY_VENUS, astro_equator_date_t_EQUATOR_OF_DATE, astro_observer_t,
    astro_refraction_t_REFRACTION_NORMAL, astro_status_t_ASTRO_SUCCESS,
};
use chrono::{DateTime, Utc};
use stellui::astro::{
    CartesianCoordinates, PolarCoordinates, SunMoonProjection, astro_time_from_datetime,
    hor_to_stereo, star_stereo,
};
use stellui::catalog;

pub struct RenderedStar {
    pub x: f64,
    pub y: f64,
    pub mag: f64,
}

pub struct RenderedPlanet {
    pub name: &'static str,
    pub symbol: &'static str,
    pub x: f64,
    pub y: f64,
    pub mag: f64,
}

pub struct SunMoonInfo {
    pub sun_stereo: Option<PolarCoordinates>,
    pub moon_stereo: Option<PolarCoordinates>,
    pub moon_phase_angle: f64,
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
            let oriented = polar.canvas_orient();
            let cart = CartesianCoordinates::from(oriented);
            Some(RenderedStar {
                x: cart.x,
                y: cart.y,
                mag: star.mag,
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
        ("Mercury", "☿", astro_body_t_BODY_MERCURY),
        ("Venus", "♀", astro_body_t_BODY_VENUS),
        ("Mars", "♂", astro_body_t_BODY_MARS),
        ("Jupiter", "♃", astro_body_t_BODY_JUPITER),
        ("Saturn", "♄", astro_body_t_BODY_SATURN),
        ("Uranus", "⛢", astro_body_t_BODY_URANUS),
        ("Neptune", "♆", astro_body_t_BODY_NEPTUNE),
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

            let oriented = polar.canvas_orient();
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
        let p = hor_to_stereo(hor).canvas_orient();
        if p.rad <= 2.0 { Some(p) } else { None }
    };

    SunMoonInfo {
        sun_stereo: to_opt(&smp.sun_hor),
        moon_stereo: to_opt(&smp.moon_hor),
        moon_phase_angle: smp.moon_phase_angle,
    }
}
