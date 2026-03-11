use crate::catalog;
use astronomy_engine_bindings::{
    Astronomy_DefineStar, Astronomy_Equator, Astronomy_Horizon, Astronomy_Illumination,
    Astronomy_MakeTime, astro_aberration_t_ABERRATION, astro_aberration_t_NO_ABERRATION,
    astro_body_t_BODY_MOON, astro_body_t_BODY_STAR1, astro_body_t_BODY_SUN,
    astro_equator_date_t_EQUATOR_OF_DATE, astro_equatorial_t, astro_horizon_t, astro_illum_t,
    astro_observer_t, astro_refraction_t_REFRACTION_NONE, astro_refraction_t_REFRACTION_NORMAL,
    astro_status_t_ASTRO_SUCCESS, astro_time_t,
};
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::ops::{Add, Mul, Sub};

#[derive(Debug)]
pub enum StarHorizonError {
    DefineStar(u32),
    EquatorOfDate(u32),
}

pub fn star_stereo(
    star: &catalog::Star,
    time: &mut astro_time_t,
    observer: &astro_observer_t,
    aberration: bool,
    refraction: bool,
) -> Result<PolarCoordinates, StarHorizonError> {
    unsafe {
        let star_status = Astronomy_DefineStar(astro_body_t_BODY_STAR1, star.ra, star.dec, 1000.0);

        if star_status != astro_status_t_ASTRO_SUCCESS {
            return Err(StarHorizonError::DefineStar(star_status));
        }

        let eq_date = Astronomy_Equator(
            astro_body_t_BODY_STAR1,
            time as *mut _,
            *observer,
            astro_equator_date_t_EQUATOR_OF_DATE,
            if aberration {
                astro_aberration_t_ABERRATION
            } else {
                astro_aberration_t_NO_ABERRATION
            },
        );

        if eq_date.status != astro_status_t_ASTRO_SUCCESS {
            return Err(StarHorizonError::EquatorOfDate(eq_date.status));
        }

        let hor = Astronomy_Horizon(
            time as *mut _,
            *observer,
            eq_date.ra,
            eq_date.dec,
            if refraction {
                astro_refraction_t_REFRACTION_NORMAL
            } else {
                astro_refraction_t_REFRACTION_NONE
            },
        );

        let stereo = hor_to_stereo(&hor);

        Ok(stereo)
    }
}

#[derive(Debug, Clone)]
pub struct CartesianCoordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for CartesianCoordinates {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<&Self> for CartesianCoordinates {
    type Output = Self;
    fn add(self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for CartesianCoordinates {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl From<&PolarCoordinates> for CartesianCoordinates {
    fn from(polar_coord: &PolarCoordinates) -> Self {
        CartesianCoordinates {
            x: polar_coord.rad * polar_coord.phi.to_radians().cos(),
            y: polar_coord.rad * polar_coord.phi.to_radians().sin(),
            z: 0.0,
        }
    }
}

impl From<PolarCoordinates> for CartesianCoordinates {
    fn from(polar_coord: PolarCoordinates) -> Self {
        CartesianCoordinates {
            x: polar_coord.rad * polar_coord.phi.to_radians().cos(),
            y: polar_coord.rad * polar_coord.phi.to_radians().sin(),
            z: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PolarCoordinates {
    pub rad: f64,
    pub phi: f64, // degrees
}

impl Mul<f64> for PolarCoordinates {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        if other < 0.0 {
            Self {
                rad: -self.rad * other,
                phi: self.phi + 180.0,
            }
        } else {
            Self {
                rad: self.rad * other,
                phi: self.phi,
            }
        }
    }
}

impl PolarCoordinates {
    pub fn mut_canvas_orient(&mut self) {
        self.phi -= 90.0;
    }

    pub fn canvas_orient(self) -> Self {
        Self {
            rad: self.rad,
            phi: self.phi - 90.0,
        }
    }

    pub fn mut_rot(&mut self, phi: f64) {
        self.phi += phi;
    }

    pub fn rot(self, phi: f64) -> Self {
        let mut out = self.clone();
        out.mut_rot(phi);
        out
    }
}

pub fn hor_to_stereo(hz: &astro_horizon_t) -> PolarCoordinates {
    let radius = 2f64 * (45f64 - hz.altitude / 2f64).to_radians().tan();

    PolarCoordinates {
        rad: radius,
        phi: hz.azimuth,
    }
}

pub fn astro_time_from_datetime(datetime: DateTime<Utc>) -> astro_time_t {
    unsafe {
        Astronomy_MakeTime(
            datetime.year(),
            datetime.month() as i32,
            datetime.day() as i32,
            datetime.hour() as i32,
            datetime.minute() as i32,
            datetime.second() as f64,
        )
    }
}

#[derive(Debug)]
pub struct SunMoonProjection {
    pub sun_hor: astro_horizon_t,
    pub moon_hor: astro_horizon_t,
    pub moon_phase_angle: f64, // degrees
}

impl SunMoonProjection {
    pub fn from_time_observer(time: &mut astro_time_t, observer: &astro_observer_t) -> Self {
        let sun_hor: astro_horizon_t;
        unsafe {
            let sun_eq: astro_equatorial_t = Astronomy_Equator(
                astro_body_t_BODY_SUN,
                time as *mut _,
                *observer,
                astro_equator_date_t_EQUATOR_OF_DATE,
                astro_aberration_t_ABERRATION,
            );
            sun_hor = Astronomy_Horizon(
                time as *mut _,
                *observer,
                sun_eq.ra,
                sun_eq.dec,
                astro_refraction_t_REFRACTION_NORMAL,
            );
        }

        let moon_hor: astro_horizon_t;
        unsafe {
            let moon_eq: astro_equatorial_t = Astronomy_Equator(
                astro_body_t_BODY_MOON,
                time as *mut _,
                *observer,
                astro_equator_date_t_EQUATOR_OF_DATE,
                astro_aberration_t_ABERRATION,
            );
            moon_hor = Astronomy_Horizon(
                time as *mut _,
                *observer,
                moon_eq.ra,
                moon_eq.dec,
                astro_refraction_t_REFRACTION_NORMAL,
            );
        }

        let illum: astro_illum_t;
        unsafe {
            illum = Astronomy_Illumination(astro_body_t_BODY_MOON, *time);
            if illum.status != astro_status_t_ASTRO_SUCCESS {
                panic!(
                    "Error {} trying to calculate Moon illumination.\n",
                    illum.status
                );
            }
        }

        Self {
            sun_hor,
            moon_hor,
            moon_phase_angle: illum.phase_angle,
        }
    }
}
