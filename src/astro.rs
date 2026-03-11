//! Coordinate math and astronomy engine wrappers.
//!
//! This module bridges the raw C astronomy-engine FFI to ergonomic Rust types.
//! All angle inputs/outputs are in **degrees** unless noted otherwise (RA is in
//! hours, matching the catalog convention).

use crate::catalog;
use astronomy_engine_bindings::{
    Astronomy_DefineStar, Astronomy_Equator, Astronomy_Horizon, Astronomy_MakeTime,
    Astronomy_MoonPhase, astro_aberration_t_ABERRATION, astro_aberration_t_NO_ABERRATION,
    astro_body_t_BODY_MOON, astro_body_t_BODY_STAR1, astro_body_t_BODY_SUN,
    astro_equator_date_t_EQUATOR_OF_DATE, astro_equatorial_t, astro_horizon_t, astro_observer_t,
    astro_refraction_t_REFRACTION_NONE, astro_refraction_t_REFRACTION_NORMAL,
    astro_status_t_ASTRO_SUCCESS, astro_time_t,
};
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::ops::{Add, Mul, Sub};

/// Errors that can occur while projecting a catalog star onto the horizon.
#[derive(Debug)]
pub enum StarHorizonError {
    /// `Astronomy_DefineStar` returned a non-success status code.
    DefineStar(u32),
    /// `Astronomy_Equator` (equator-of-date conversion) returned a non-success status code.
    EquatorOfDate(u32),
}

/// Project a catalog star onto the sky as stereographic polar coordinates.
///
/// # Parameters
/// - `star` — catalog entry with `ra` in **hours** and `dec` in **degrees**
/// - `time` — mutable astronomy-engine time (mutated internally by the C library)
/// - `observer` — geographic observer position
/// - `aberration` — whether to apply stellar aberration correction
/// - `refraction` — whether to apply atmospheric refraction correction
///
/// # Returns
/// [`PolarCoordinates`] where `rad` is the stereographic radius
/// (`0` = zenith, `2.0` = horizon, `>2.0` = below horizon) and
/// `phi` is the azimuth in degrees (North = 0°, East = 90°).
///
/// # Safety
/// Calls unsafe FFI functions from `astronomy_engine_bindings`. The caller must
/// ensure `time` and `observer` are valid values produced by the same library.
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

/// 2-D canvas coordinates used by the ratatui [`Canvas`](ratatui::widgets::canvas::Canvas).
///
/// The `z` field is always `0.0`; it exists only for potential future use.
/// Both `x` and `y` range over approximately `[-2.2, 2.2]`, matching the
/// canvas bounds used by the sky renderer.
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

/// Stereographic polar coordinates used for all-sky projection.
///
/// - `rad` — stereographic radius: `0.0` = zenith, `2.0` = horizon, `>2.0` = below horizon
/// - `phi` — angle in **degrees**; raw value is azimuth (North=0°, East=90°) before
///   [`canvas_orient`](PolarCoordinates::canvas_orient) is applied
#[derive(Debug, Clone)]
pub struct PolarCoordinates {
    pub rad: f64,
    /// Angle in degrees.
    pub phi: f64,
}

impl Mul<f64> for PolarCoordinates {
    type Output = Self;

    /// Scale the radius by `other`.
    ///
    /// If `other` is negative the radius is kept positive and `phi` is flipped
    /// by 180°, preserving the geometric direction of the point.
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
    /// Rotate `phi` in-place by −90° so that the canvas orientation matches the
    /// all-sky convention: **North = bottom, South = top, East = left, West = right**.
    pub fn mut_canvas_orient(&mut self) {
        self.phi -= 90.0;
    }

    /// Return a new [`PolarCoordinates`] rotated by −90° for canvas orientation.
    ///
    /// Equivalent to [`mut_canvas_orient`](Self::mut_canvas_orient) but consumes
    /// `self` and returns a new value.
    pub fn canvas_orient(self) -> Self {
        Self {
            rad: self.rad,
            phi: self.phi - 90.0,
        }
    }

    /// Rotate `phi` in-place by `phi` degrees (positive = counter-clockwise).
    pub fn mut_rot(&mut self, phi: f64) {
        self.phi += phi;
    }

    /// Return a new [`PolarCoordinates`] with `phi` increased by `phi` degrees.
    pub fn rot(self, phi: f64) -> Self {
        let mut out = self.clone();
        out.mut_rot(phi);
        out
    }
}

/// Convert an astronomy-engine horizon position to stereographic polar coordinates.
///
/// Uses the formula `r = 2·tan(45° − alt/2)`:
/// - altitude 90° (zenith) → `rad ≈ 0.0`
/// - altitude 0° (horizon) → `rad = 2.0`
/// - altitude < 0° (below horizon) → `rad > 2.0`
///
/// `phi` is set to `hz.azimuth` unchanged (North = 0°, East = 90°).
pub fn hor_to_stereo(hz: &astro_horizon_t) -> PolarCoordinates {
    let radius = 2f64 * (45f64 - hz.altitude / 2f64).to_radians().tan();

    PolarCoordinates {
        rad: radius,
        phi: hz.azimuth,
    }
}

/// Thin wrapper around `Astronomy_MakeTime` that accepts a [`chrono::DateTime<Utc>`].
///
/// Returns an `astro_time_t` suitable for passing to other astronomy-engine functions.
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

/// Horizon positions of the Sun and Moon, plus the Moon's cycle position.
#[derive(Debug)]
pub struct SunMoonProjection {
    /// Horizon coordinates (altitude, azimuth in degrees) of the Sun.
    pub sun_hor: astro_horizon_t,
    /// Horizon coordinates (altitude, azimuth in degrees) of the Moon.
    pub moon_hor: astro_horizon_t,
    /// Moon cycle position in degrees (0° = new moon, 90° = first quarter, 180° = full moon, 270° = last quarter).
    pub moon_cycle_degrees: f64,
}

impl SunMoonProjection {
    /// Compute Sun and Moon horizon positions and Moon phase for a given time and observer.
    ///
    /// # Panics
    /// Panics if the astronomy engine fails to compute the Moon's illumination.
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

        let moon_cycle_degrees: f64;
        unsafe {
            let phase = Astronomy_MoonPhase(*time);
            if phase.status != astro_status_t_ASTRO_SUCCESS {
                panic!("Error {} trying to calculate Moon phase.\n", phase.status);
            }
            moon_cycle_degrees = phase.angle;
        }

        Self {
            sun_hor,
            moon_hor,
            moon_cycle_degrees,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use astronomy_engine_bindings::astro_horizon_t;
    use chrono::TimeZone;

    fn make_hz(altitude: f64, azimuth: f64) -> astro_horizon_t {
        astro_horizon_t {
            altitude,
            azimuth,
            ra: 0.0,
            dec: 0.0,
        }
    }

    // --- hor_to_stereo ---

    #[test]
    fn hor_to_stereo_zenith() {
        let hz = make_hz(90.0, 0.0);
        let p = hor_to_stereo(&hz);
        assert!(
            p.rad.abs() < 1e-10,
            "zenith rad should be ~0, got {}",
            p.rad
        );
    }

    #[test]
    fn hor_to_stereo_horizon() {
        let hz = make_hz(0.0, 0.0);
        let p = hor_to_stereo(&hz);
        assert!(
            (p.rad - 2.0).abs() < 1e-10,
            "horizon rad should be 2.0, got {}",
            p.rad
        );
    }

    #[test]
    fn hor_to_stereo_below_horizon() {
        let hz = make_hz(-10.0, 0.0);
        let p = hor_to_stereo(&hz);
        assert!(
            p.rad > 2.0,
            "below-horizon rad should be >2.0, got {}",
            p.rad
        );
    }

    #[test]
    fn hor_to_stereo_azimuth_passthrough() {
        let hz = make_hz(45.0, 137.5);
        let p = hor_to_stereo(&hz);
        assert_eq!(p.phi, 137.5);
    }

    // --- canvas_orient / mut_canvas_orient ---

    #[test]
    fn canvas_orient_shifts_phi() {
        let p = PolarCoordinates {
            rad: 1.0,
            phi: 100.0,
        };
        let oriented = p.canvas_orient();
        assert_eq!(oriented.phi, 10.0);
    }

    #[test]
    fn mut_canvas_orient_shifts_phi() {
        let mut p = PolarCoordinates {
            rad: 1.0,
            phi: 100.0,
        };
        p.mut_canvas_orient();
        assert_eq!(p.phi, 10.0);
    }

    // --- rot / mut_rot ---

    #[test]
    fn rot_adds_angle() {
        let p = PolarCoordinates {
            rad: 1.0,
            phi: 10.0,
        };
        let rotated = p.rot(45.0);
        assert_eq!(rotated.phi, 55.0);
    }

    // --- Mul<f64> ---

    #[test]
    fn polar_mul_positive() {
        let p = PolarCoordinates {
            rad: 2.0,
            phi: 30.0,
        };
        let scaled = p * 3.0;
        assert_eq!(scaled.rad, 6.0);
        assert_eq!(scaled.phi, 30.0);
    }

    #[test]
    fn polar_mul_negative() {
        let p = PolarCoordinates {
            rad: 2.0,
            phi: 30.0,
        };
        let scaled = p * -1.0;
        assert_eq!(scaled.rad, 2.0);
        assert_eq!(scaled.phi, 210.0);
    }

    // --- CartesianCoordinates::from ---

    #[test]
    fn cartesian_from_polar_zero() {
        let p = PolarCoordinates { rad: 0.0, phi: 0.0 };
        let c = CartesianCoordinates::from(p);
        assert!(c.x.abs() < 1e-10);
        assert!(c.y.abs() < 1e-10);
    }

    #[test]
    fn cartesian_from_polar_north() {
        // phi=0° → x=rad, y=0
        let p = PolarCoordinates { rad: 1.0, phi: 0.0 };
        let c = CartesianCoordinates::from(p);
        assert!((c.x - 1.0).abs() < 1e-10, "x={}", c.x);
        assert!(c.y.abs() < 1e-10, "y={}", c.y);
    }

    #[test]
    fn cartesian_from_polar_east() {
        // phi=90° → x≈0, y=rad
        let p = PolarCoordinates {
            rad: 1.0,
            phi: 90.0,
        };
        let c = CartesianCoordinates::from(p);
        assert!(c.x.abs() < 1e-10, "x={}", c.x);
        assert!((c.y - 1.0).abs() < 1e-10, "y={}", c.y);
    }

    #[test]
    fn cartesian_add() {
        let a = CartesianCoordinates {
            x: 1.0,
            y: 2.0,
            z: 0.0,
        };
        let b = CartesianCoordinates {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        let sum = a + b;
        assert_eq!(sum.x, 4.0);
        assert_eq!(sum.y, 6.0);
    }

    #[test]
    fn cartesian_sub() {
        let a = CartesianCoordinates {
            x: 5.0,
            y: 7.0,
            z: 0.0,
        };
        let b = CartesianCoordinates {
            x: 2.0,
            y: 3.0,
            z: 0.0,
        };
        let diff = a - b;
        assert_eq!(diff.x, 3.0);
        assert_eq!(diff.y, 4.0);
    }

    // --- FFI smoke tests ---

    #[test]
    fn astro_time_from_datetime_does_not_panic() {
        let dt = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
        let _ = astro_time_from_datetime(dt);
    }

    #[test]
    fn star_stereo_known_star() {
        use astronomy_engine_bindings::astro_observer_t;

        // Polaris: RA ≈ 2.530 h, Dec ≈ 89.264°
        let polaris = crate::catalog::Star {
            id: 99999,
            ra: 2.530_111,
            dec: 89.264_108,
            mag: 1.98,
        };

        // NYC observer
        let observer = astro_observer_t {
            latitude: 40.71,
            longitude: -74.01,
            height: 0.0,
        };

        // Fixed UTC moment: 2024-06-21 12:00:00 UTC
        let dt = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
        let mut time = astro_time_from_datetime(dt);

        let result = star_stereo(&polaris, &mut time, &observer, false, false);
        let polar = result.expect("Polaris projection should succeed");

        // Polaris is always above the horizon from NYC
        assert!(
            polar.rad < 2.0,
            "Polaris rad={} should be <2.0 (above horizon)",
            polar.rad
        );
        // phi should be a valid azimuth
        assert!(
            polar.phi >= 0.0 && polar.phi < 360.0,
            "phi={} out of range",
            polar.phi
        );
    }
}
