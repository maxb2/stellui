use astronomy_engine_bindings::{
    Astronomy_Equator, Astronomy_Horizon, astro_aberration_t_ABERRATION,
    astro_body_t_BODY_SUN, astro_equator_date_t_EQUATOR_OF_DATE, astro_observer_t,
    astro_refraction_t_REFRACTION_NORMAL, astro_status_t_ASTRO_SUCCESS,
};
use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::Deserialize;
use stellui::astro::astro_time_from_datetime;

#[derive(Deserialize)]
struct WeatherResponse {
    hourly: HourlyData,
}

#[derive(Deserialize)]
struct HourlyData {
    time: Vec<String>,
    cloud_cover: Vec<Option<f64>>,
    relative_humidity_2m: Vec<Option<f64>>,
    precipitation_probability: Vec<Option<f64>>,
    visibility: Vec<Option<f64>>,
    temperature_2m: Vec<Option<f64>>,
    windspeed_10m: Vec<Option<f64>>,
}

pub enum DayPeriod {
    Day,
    CivilTwilight,
    NauticalTwilight,
    AstronomicalTwilight,
    Night,
}

impl DayPeriod {
    pub fn symbol(&self) -> &'static str {
        match self {
            DayPeriod::Day => "☀",
            DayPeriod::CivilTwilight => "⊙",
            DayPeriod::NauticalTwilight => "◑",
            DayPeriod::AstronomicalTwilight => "○",
            DayPeriod::Night => "★",
        }
    }
}

pub struct HourlyForecast {
    pub time: String,
    pub cloud_cover: f64,
    pub relative_humidity: f64,
    pub precip_probability: f64,
    pub visibility_km: f64,
    pub temperature_c: f64,
    pub wind_speed_kmh: f64,
    pub seeing: SeeingQuality,
    pub day_period: DayPeriod,
}

pub enum SeeingQuality {
    Excellent,
    Good,
    Fair,
    Poor,
    Bad,
}

impl SeeingQuality {
    pub fn label(&self) -> &'static str {
        match self {
            SeeingQuality::Excellent => "Excellent",
            SeeingQuality::Good => "Good",
            SeeingQuality::Fair => "Fair",
            SeeingQuality::Poor => "Poor",
            SeeingQuality::Bad => "Bad",
        }
    }
}

fn compute_seeing(cloud: f64, humidity: f64, precip: f64, visibility_km: f64, wind_kmh: f64) -> SeeingQuality {
    // Hard gates: heavy cloud cover overrides all other factors
    if cloud >= 90.0 {
        return SeeingQuality::Bad;
    }
    if cloud >= 70.0 {
        return SeeingQuality::Poor;
    }

    let cloud_score = if cloud < 10.0 { 3 } else if cloud < 30.0 { 2 } else { 1 }; // <70% max

    let humidity_score = if humidity < 60.0 { 2 } else if humidity < 75.0 { 1 } else { 0 };

    let precip_score = if precip == 0.0 { 2 } else if precip < 20.0 { 1 } else { 0 };

    let vis_score = if visibility_km > 20.0 { 1 } else { 0 };

    // Wind: calm = good seeing, strong = turbulence
    let wind_score = if wind_kmh < 10.0 { 2 } else if wind_kmh < 25.0 { 1 } else { 0 };

    let total = cloud_score + humidity_score + precip_score + vis_score + wind_score;

    match total {
        8..=10 => SeeingQuality::Excellent,
        6..=7 => SeeingQuality::Good,
        4..=5 => SeeingQuality::Fair,
        2..=3 => SeeingQuality::Poor,
        _ => SeeingQuality::Bad,
    }
}

fn sun_altitude_at(lat: f64, lon: f64, time_str: &str) -> f64 {
    let Ok(ndt) = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M") else {
        return -90.0;
    };
    let dt = Utc.from_utc_datetime(&ndt);
    let observer = astro_observer_t {
        latitude: lat,
        longitude: lon,
        height: 0.0,
    };
    let mut time = astro_time_from_datetime(dt);
    unsafe {
        let eq = Astronomy_Equator(
            astro_body_t_BODY_SUN,
            &mut time as *mut _,
            observer,
            astro_equator_date_t_EQUATOR_OF_DATE,
            astro_aberration_t_ABERRATION,
        );
        if eq.status != astro_status_t_ASTRO_SUCCESS {
            return -90.0;
        }
        let hor = Astronomy_Horizon(
            &mut time as *mut _,
            observer,
            eq.ra,
            eq.dec,
            astro_refraction_t_REFRACTION_NORMAL,
        );
        hor.altitude
    }
}

fn classify_day_period(alt: f64) -> DayPeriod {
    if alt > 0.0 {
        DayPeriod::Day
    } else if alt > -6.0 {
        DayPeriod::CivilTwilight
    } else if alt > -12.0 {
        DayPeriod::NauticalTwilight
    } else if alt > -18.0 {
        DayPeriod::AstronomicalTwilight
    } else {
        DayPeriod::Night
    }
}

pub fn fetch_forecast(lat: f64, lon: f64) -> anyhow::Result<Vec<HourlyForecast>> {
    let now = Utc::now();
    let start = now.format("%Y-%m-%dT%H:00").to_string();
    let end = (now + chrono::Duration::hours(72)).format("%Y-%m-%dT%H:00").to_string();
    let url = format!(
        "https://api.open-meteo.com/v1/forecast\
?latitude={lat}&longitude={lon}\
&hourly=cloud_cover,relative_humidity_2m,precipitation_probability,visibility,temperature_2m,windspeed_10m\
&start_hour={start}\
&end_hour={end}\
&timezone=GMT"
    );

    let response: WeatherResponse = ureq::get(&url).call()?.into_json()?;
    let h = response.hourly;

    let n = h.time.len();
    let forecasts = (0..n)
        .map(|i| {
            let cloud = h.cloud_cover.get(i).and_then(|x| *x).unwrap_or(0.0);
            let humidity = h
                .relative_humidity_2m
                .get(i)
                .and_then(|x| *x)
                .unwrap_or(0.0);
            let precip = h
                .precipitation_probability
                .get(i)
                .and_then(|x| *x)
                .unwrap_or(0.0);
            let visibility_m = h.visibility.get(i).and_then(|x| *x).unwrap_or(0.0);
            let visibility_km = visibility_m / 1000.0;
            let temperature_c = h.temperature_2m.get(i).and_then(|x| *x).unwrap_or(0.0);
            let wind_speed_kmh = h.windspeed_10m.get(i).and_then(|x| *x).unwrap_or(0.0);
            let seeing = compute_seeing(cloud, humidity, precip, visibility_km, wind_speed_kmh);
            let alt = sun_altitude_at(lat, lon, &h.time[i]);
            let day_period = classify_day_period(alt);
            HourlyForecast {
                time: h.time[i].clone(),
                cloud_cover: cloud,
                relative_humidity: humidity,
                precip_probability: precip,
                visibility_km,
                temperature_c,
                wind_speed_kmh,
                seeing,
                day_period,
            }
        })
        .collect();

    Ok(forecasts)
}
