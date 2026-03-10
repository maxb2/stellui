use crate::catalog;
use astronomy_engine_bindings::{
    Astronomy_CurrentTime, Astronomy_DefineStar, Astronomy_Equator, Astronomy_Horizon,
    Astronomy_HourAngle, Astronomy_Illumination, Astronomy_MakeObserver, Astronomy_MakeTime,
    Astronomy_Pivot, Astronomy_RotateVector, Astronomy_Rotation_EQD_HOR,
    astro_aberration_t_ABERRATION, astro_aberration_t_NO_ABERRATION, astro_body_t_BODY_MOON,
    astro_body_t_BODY_STAR1, astro_body_t_BODY_SUN, astro_equator_date_t_EQUATOR_OF_DATE,
    astro_equatorial_t, astro_horizon_t, astro_illum_t, astro_observer_t,
    astro_refraction_t_REFRACTION_NONE, astro_refraction_t_REFRACTION_NORMAL,
    astro_status_t_ASTRO_SUCCESS, astro_time_t,
};
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use lazy_static::lazy_static;
use std::ops::{Add, Mul, Sub};
use tzf_rs::DefaultFinder;
use zeno::{Command, PathBuilder, Transform};

lazy_static! {
    pub static ref FINDER: DefaultFinder = DefaultFinder::new();
}

pub fn solar_time() {
    unsafe {
        let mut time = Astronomy_CurrentTime();
        let observer = Astronomy_MakeObserver(38.933601, -92.362999, 0.0);
        let ha = Astronomy_HourAngle(astro_body_t_BODY_SUN, &mut time as *mut _, observer);

        if ha.status != astro_status_t_ASTRO_SUCCESS {
            panic!("ERROR {} in Astronomy_HourAngle().", ha.status);
        }

        println!("{:?}", ha);
    }
}

#[derive(Debug)]
pub struct StarJ2000 {
    pub ra: f64,   // hours
    pub dec: f64,  // degrees
    pub dist: f64, // light years
}

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

        return Ok(stereo);
    }
}

pub fn star_horizon(
    star: &StarJ2000,
    time: &mut astro_time_t,
    observer: &astro_observer_t,
    aberration: bool,
    refraction: bool,
) -> Result<astro_horizon_t, StarHorizonError> {
    unsafe {
        let star_status =
            Astronomy_DefineStar(astro_body_t_BODY_STAR1, star.ra, star.dec, star.dist);

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

        return Ok(hor);
    }
}

pub fn horiz_example0() {
    let ra_j2000 = 6.0 + 45.0 / 60.0 + 8.917 / 3600.0;
    let dec_j2000 = -(16.0 + 42.0 / 60.0 + 58.02 / 3600.0);
    let star = StarJ2000 {
        ra: ra_j2000,
        dec: dec_j2000,
        dist: 8.6,
    };

    println!("{:?}", star);

    let mut time: astro_time_t;
    let observer = astro_observer_t {
        latitude: 38.933601,
        longitude: -92.362999,
        height: 0.0,
    };
    unsafe {
        time = Astronomy_MakeTime(2024, 1, 1, 1, 1, 1.0);
    }
    let hor = star_horizon(&star, &mut time, &observer, true, true).unwrap();

    println!("{:?}", hor);

    let hz = HzCoordinates {
        alt: hor.altitude,
        az: hor.azimuth,
    };

    println!("{:?}", hz_to_stereo(&hz));

    for star in catalog::J2000_CATALOG.iter() {
        let stereo = star_stereo(star, &mut time, &observer, true, true).unwrap();
        println!("{:?}", star);
        println!("{:?}", stereo);
    }
}

pub fn horiz_example1() {
    let observer = astro_observer_t {
        latitude: 38.933601,
        longitude: -92.362999,
        height: 0.0,
    };

    let tz_name = FINDER.get_tz_name(observer.longitude, observer.latitude);

    let tz: Tz = tz_name.parse().unwrap();

    let dt = tz.with_ymd_and_hms(2016, 10, 22, 12, 0, 0);

    let mut time = astro_time_from_datetime(dt.unwrap().to_utc());

    catalog::J2000_CATALOG
        .iter()
        .filter(|&star| star.mag <= 5.0)
        .map(|star| star_stereo(star, &mut time, &observer, true, true).unwrap())
        .enumerate()
        .for_each(|(ix, stereo)| {
            println!("{:?} {:?}", ix, stereo);
        });

    unsafe {
        let sun_eq = Astronomy_Equator(
            astro_body_t_BODY_SUN,
            &mut time as *mut _,
            observer,
            astro_equator_date_t_EQUATOR_OF_DATE,
            astro_aberration_t_ABERRATION,
        );

        if sun_eq.status != astro_status_t_ASTRO_SUCCESS {
            panic!("eq: {}", sun_eq.status);
        }

        let sun_hor = Astronomy_Horizon(
            &mut time as *mut _,
            observer,
            sun_eq.ra,
            sun_eq.dec,
            astro_refraction_t_REFRACTION_NORMAL,
        );

        let sun_stereo = hor_to_stereo(&sun_hor);

        println!("Sun: {:?} {:?}", sun_hor, sun_stereo);

        let moon_eq = Astronomy_Equator(
            astro_body_t_BODY_MOON,
            &mut time as *mut _,
            observer,
            astro_equator_date_t_EQUATOR_OF_DATE,
            astro_aberration_t_ABERRATION,
        );

        if moon_eq.status != astro_status_t_ASTRO_SUCCESS {
            panic!("eq: {}", moon_eq.status);
        }

        let moon_hor = Astronomy_Horizon(
            &mut time as *mut _,
            observer,
            moon_eq.ra,
            moon_eq.dec,
            astro_refraction_t_REFRACTION_NORMAL,
        );

        let moon_stereo = hor_to_stereo(&moon_hor);

        println!("Moon: {:?} {:?}", moon_hor, moon_stereo);
    }

    println!("{:?}", observer);
}

#[derive(Debug, Clone)]
pub struct HzCoordinates {
    pub alt: f64,
    pub az: f64,
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
            z: 0.0.into(),
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
    pub fn mut_canvas_orient(self: &mut Self) {
        self.phi = -self.phi - 90.0;
    }

    pub fn canvas_orient(self: Self) -> Self {
        Self {
            rad: self.rad,
            phi: -self.phi - 90.0,
        }
    }

    pub fn mut_rot(self: &mut Self, phi: f64) {
        self.phi = self.phi + phi;
    }

    pub fn rot(self: Self, phi: f64) -> Self {
        let mut out = self.clone();
        out.mut_rot(phi);
        return out;
    }
}

pub fn hz_to_stereo(hz: &HzCoordinates) -> PolarCoordinates {
    let radius = 2f64 * (45f64 - hz.alt / 2f64).to_radians().tan();

    return PolarCoordinates {
        rad: radius,
        phi: hz.az,
    };
}

pub fn hor_to_stereo(hz: &astro_horizon_t) -> PolarCoordinates {
    let radius = 2f64 * (45f64 - hz.altitude / 2f64).to_radians().tan();

    return PolarCoordinates {
        rad: radius,
        phi: hz.azimuth,
    };
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
pub enum MoonFaceTiltError {
    RotationEqdHor(u32),
    PivotAzimuth(u32),
    PivotAltitude(u32),
    RotateVector(u32),
    UnitVectorTolerance(f64),
}

/// Calculates the tilt angle of the moon CCW from the unit vector perpendicular to the horizon.
/// See AstronomyEngine's [camera demo](https://github.com/cosinekitty/astronomy/blob/master/demo/c/README.md#camera).
pub fn _moon_face_tilt(
    sun_eq: astro_equatorial_t,
    moon_eq: astro_equatorial_t,
    moon_hor: astro_horizon_t,
    time: &mut astro_time_t,
    observer: astro_observer_t,
) -> Result<f64, MoonFaceTiltError> {
    // const TOLERANCE: f64 = 1.0e-8;

    // ccw angle from zenith
    let moon_tilt: f64;

    unsafe {
        /* Get the rotation matrix that converts equatorial to horizontal coordintes for this place and time. */
        let mut rot = Astronomy_Rotation_EQD_HOR(time as *mut _, observer);

        /*
            Modify the rotation matrix in two steps:
            First, rotate the orientation so we are facing the Moon's azimuth.
            We do this by pivoting around the zenith axis.
            Horizontal axes are: 0 = north, 1 = west, 2 = zenith.
            Tricky: because the pivot angle increases counterclockwise, and azimuth
            increases clockwise, we undo the azimuth by adding the positive value.
        */
        rot = Astronomy_Pivot(rot, 2, moon_hor.azimuth);
        if rot.status != astro_status_t_ASTRO_SUCCESS {
            return Err(MoonFaceTiltError::PivotAzimuth(rot.status));
        }

        /*
            Second, pivot around the leftward axis to bring the Moon to the camera's altitude level.
            From the point of view of the leftward axis, looking toward the camera,
            adding the angle is the correct sense for subtracting the altitude.
        */
        rot = Astronomy_Pivot(rot, 1, moon_hor.altitude);
        if rot.status != astro_status_t_ASTRO_SUCCESS {
            return Err(MoonFaceTiltError::PivotAltitude(rot.status));
        }

        /* As a sanity check, apply this rotation to the Moon's equatorial (EQD) coordinates and verify x=0, y=0. */
        let check_vec = Astronomy_RotateVector(rot, moon_eq.vec);
        if check_vec.status != astro_status_t_ASTRO_SUCCESS {
            return Err(MoonFaceTiltError::RotateVector(check_vec.status));
        }

        /* Convert to unit vector. */
        // let radius = Astronomy_VectorLength(check_vec);
        // check_vec.x /= radius;
        // check_vec.y /= radius;
        // check_vec.z /= radius;
        // println!(
        //     "Moon check: x = {:0>.6}, y = {:0>.6}, z = {:0>.6}\n",
        //     check_vec.x,
        //     check_vec.y.abs(),
        //     check_vec.z.abs()
        // );
        // let err = (check_vec.x - 1.0).abs();
        // if err > TOLERANCE {
        //     return Err(MoonFaceTiltError::UnitVectorTolerance(err));
        // }

        // if check_vec.y.abs() > tolerance {
        //     panic!("Excessive error in moon check (y)");
        // }

        // if check_vec.z.abs() > tolerance {
        //     panic!("Excessive error in moon check (z)");
        // }

        /* Apply the same rotation to the Sun's equatorial vector. */
        /* The x- and y-coordinates now tell us which side appears sunlit in the camera! */

        let vec = Astronomy_RotateVector(rot, sun_eq.vec);
        if vec.status != astro_status_t_ASTRO_SUCCESS {
            return Err(MoonFaceTiltError::RotateVector(vec.status));
        }

        /* Don't bother normalizing the Sun vector, because in AU it will be close to unit anyway. */
        // println!(
        //     "Sun vector: x = {:0>.6}, y = {:0>.6}, z = {:0>.6}\n",
        //     vec.x, vec.y, vec.z
        // );

        /* Calculate the tilt angle of the sunlit side, as seen by the camera. */
        /* The x-axis is now pointing directly at the object, z is up in the camera image, y is to the left. */
        moon_tilt = vec.y.atan2(vec.z);
        // println!(
        //     "Tilt angle of sunlit side of the Moon = {:0>.3} degrees counterclockwise from up.\n",
        //     moon_tilt
        // );
    }

    Ok(moon_tilt.to_degrees())
}

#[derive(Debug)]
pub struct SunMoonProjection {
    pub sun_eq: astro_equatorial_t,
    pub sun_hor: astro_horizon_t,
    pub moon_eq: astro_equatorial_t,
    pub moon_hor: astro_horizon_t,
    pub moon_path: Vec<Command>,
    pub moon_path_transform: zeno::Transform,
    pub moon_phase_angle: f64, // degrees
    pub moon_face_tilt: f64,   // degrees
}

impl SunMoonProjection {
    pub fn from_time_observer(time: &mut astro_time_t, observer: &astro_observer_t) -> Self {
        let sun_eq: astro_equatorial_t;
        let sun_hor: astro_horizon_t;
        unsafe {
            sun_eq = Astronomy_Equator(
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

        let moon_eq: astro_equatorial_t;
        let moon_hor: astro_horizon_t;
        unsafe {
            moon_eq = Astronomy_Equator(
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

        let moon_face_tilt = _moon_face_tilt(sun_eq, moon_eq, moon_hor, time, *observer).unwrap();

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

        let moon_path = moon_phase_path_unit(illum.phase_angle as f32);

        let moon_path_transform = Transform::rotation(zeno::Angle::from_degrees(
            (-moon_face_tilt + 180. - moon_hor.azimuth) as f32,
        ));

        Self {
            sun_eq,
            sun_hor,
            moon_eq,
            moon_hor,
            moon_path,
            moon_path_transform,
            moon_phase_angle: illum.phase_angle,
            moon_face_tilt,
        }
    }
}

pub fn moon_face_tilt(
    time: &mut astro_time_t,
    observer: &astro_observer_t,
) -> (Result<f64, MoonFaceTiltError>, astro_horizon_t) {
    let sun_eq: astro_equatorial_t;
    unsafe {
        sun_eq = Astronomy_Equator(
            astro_body_t_BODY_SUN,
            time as *mut _,
            *observer,
            astro_equator_date_t_EQUATOR_OF_DATE,
            astro_aberration_t_ABERRATION,
        );
    }

    let moon_eq: astro_equatorial_t;
    let moon_hor: astro_horizon_t;
    unsafe {
        moon_eq = Astronomy_Equator(
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

    (
        _moon_face_tilt(sun_eq, moon_eq, moon_hor, time, *observer),
        moon_hor,
    )
}

/// Constructs a path describing the illuminated part of the moon.
/// Note: this returns arcs with unit radius and the illuminated
/// part oriented upward. You should scale this by the final moon
/// radius and rotate to the final orientation.
pub fn moon_phase_path_unit(phase_angle: f32) -> Vec<Command> {
    let mut path: Vec<Command> = Vec::new();

    const LEFT: (f32, f32) = (-1f32, 0f32);

    const RIGHT: (f32, f32) = (1f32, 0f32);

    path.move_to(LEFT);

    let sweep_angle: zeno::Angle = zeno::Angle::from_degrees(180.);

    let sweep_outer: zeno::ArcSweep;
    let sweep_inner: zeno::ArcSweep;

    let mut _phase_angle = phase_angle;

    if 0. <= phase_angle && phase_angle < 90. {
        sweep_outer = zeno::ArcSweep::Positive;
        sweep_inner = zeno::ArcSweep::Positive;
    } else if 90. <= phase_angle && phase_angle < 180. {
        sweep_outer = zeno::ArcSweep::Positive;
        sweep_inner = zeno::ArcSweep::Negative;
    } else if 180. <= phase_angle && phase_angle < 270. {
        _phase_angle = 360. - phase_angle;
        sweep_outer = zeno::ArcSweep::Positive;
        sweep_inner = zeno::ArcSweep::Negative;
    } else {
        _phase_angle = 360.0 - phase_angle;
        sweep_outer = zeno::ArcSweep::Positive;
        sweep_inner = zeno::ArcSweep::Positive;
    }

    let ry = _phase_angle.to_radians().cos().abs().max(1e-16);

    path.arc_to(
        1f32,
        1f32,
        sweep_angle,
        zeno::ArcSize::Small,
        sweep_outer,
        RIGHT,
    )
    .arc_to(
        1f32,
        ry,
        sweep_angle,
        zeno::ArcSize::Small,
        sweep_inner,
        LEFT,
    );

    path
}
