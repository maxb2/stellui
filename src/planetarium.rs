use astronomy_engine_bindings::astro_observer_t;
use chrono::{DateTime, Utc};
use planetuium::astro::{
    astro_time_from_datetime, hor_to_stereo, star_stereo, CartesianCoordinates, PolarCoordinates,
    SunMoonProjection,
};
use planetuium::catalog;

pub struct RenderedStar {
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
