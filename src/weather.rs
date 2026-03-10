use serde::Deserialize;

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
}

pub struct HourlyForecast {
    pub time: String,
    pub cloud_cover: f64,
    pub relative_humidity: f64,
    pub precip_probability: f64,
    pub visibility_km: f64,
    pub temperature_c: f64,
    pub seeing: SeeingQuality,
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

fn compute_seeing(cloud: f64, humidity: f64, precip: f64, visibility_km: f64) -> SeeingQuality {
    let cloud_score = if cloud < 10.0 {
        3
    } else if cloud < 30.0 {
        2
    } else if cloud < 60.0 {
        1
    } else {
        0
    };

    let humidity_score = if humidity < 60.0 {
        2
    } else if humidity < 75.0 {
        1
    } else {
        0
    };

    let precip_score = if precip == 0.0 {
        2
    } else if precip < 20.0 {
        1
    } else {
        0
    };

    let vis_score = if visibility_km > 20.0 { 1 } else { 0 };

    let total = cloud_score + humidity_score + precip_score + vis_score;

    match total {
        7..=8 => SeeingQuality::Excellent,
        5..=6 => SeeingQuality::Good,
        3..=4 => SeeingQuality::Fair,
        1..=2 => SeeingQuality::Poor,
        _ => SeeingQuality::Bad,
    }
}

pub fn fetch_forecast(lat: f64, lon: f64) -> anyhow::Result<Vec<HourlyForecast>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast\
?latitude={lat}&longitude={lon}\
&hourly=cloud_cover,relative_humidity_2m,precipitation_probability,visibility,temperature_2m\
&forecast_days=3&timezone=auto"
    );

    let response: WeatherResponse = ureq::get(&url).call()?.into_json()?;
    let h = response.hourly;

    let n = h.time.len();
    let forecasts = (0..n)
        .map(|i| {
            let cloud = h.cloud_cover.get(i).and_then(|x| *x).unwrap_or(0.0);
            let humidity = h.relative_humidity_2m.get(i).and_then(|x| *x).unwrap_or(0.0);
            let precip = h.precipitation_probability.get(i).and_then(|x| *x).unwrap_or(0.0);
            let visibility_m = h.visibility.get(i).and_then(|x| *x).unwrap_or(0.0);
            let visibility_km = visibility_m / 1000.0;
            let temperature_c = h.temperature_2m.get(i).and_then(|x| *x).unwrap_or(0.0);
            let seeing = compute_seeing(cloud, humidity, precip, visibility_km);
            HourlyForecast {
                time: h.time[i].clone(),
                cloud_cover: cloud,
                relative_humidity: humidity,
                precip_probability: precip,
                visibility_km,
                temperature_c,
                seeing,
            }
        })
        .collect();

    Ok(forecasts)
}
