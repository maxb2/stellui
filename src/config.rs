use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Location {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(default)]
    pub height: f64,
    pub timezone: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub locations: Vec<Location>,
    pub max_mag: Option<f64>,
}

fn config_path() -> Option<std::path::PathBuf> {
    let mut p = config_base_dir()?;
    p.push("stellui");
    p.push("config.toml");
    Some(p)
}

fn config_base_dir() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "windows")]
    return std::env::var_os("APPDATA").map(std::path::PathBuf::from);

    #[cfg(not(target_os = "windows"))]
    {
        let home = std::env::var_os("HOME")?;
        let mut p = std::path::PathBuf::from(home);
        p.push(".config");
        Some(p)
    }
}

impl Config {
    pub fn load() -> Self {
        let Some(path) = config_path() else { return Self::default() };
        let Ok(text) = std::fs::read_to_string(&path) else { return Self::default() };
        toml::from_str(&text).unwrap_or_default()
    }

    pub fn effective_locations(&self) -> Vec<Location> {
        if !self.locations.is_empty() {
            self.locations.clone()
        } else {
            vec![Location {
                name: "New York".to_string(),
                lat: 40.71,
                lon: -74.01,
                height: 0.0,
                timezone: None,
            }]
        }
    }

    pub fn save(locations: &[Location], max_mag: Option<f64>) {
        let Some(path) = config_path() else { return };
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let cfg = Config {
            locations: locations.to_vec(),
            max_mag,
        };
        if let Ok(text) = toml::to_string(&cfg) {
            let _ = std::fs::write(&path, text);
        }
    }
}
