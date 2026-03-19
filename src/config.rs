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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scope {
    pub name: String,
    /// Aperture in millimetres.
    pub aperture_mm: f64,
    /// Focal length in millimetres.
    pub focal_length_mm: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Eyepiece {
    pub name: String,
    /// Focal length in millimetres.
    pub focal_length_mm: f64,
    /// Apparent field of view in degrees (typically 40–100°).
    pub afov_deg: f64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub locations: Vec<Location>,
    pub max_mag: Option<f64>,
    #[serde(default)]
    pub scopes: Vec<Scope>,
    #[serde(default)]
    pub eyepieces: Vec<Eyepiece>,
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

    pub fn effective_scopes(&self) -> Vec<Scope> {
        if !self.scopes.is_empty() {
            self.scopes.clone()
        } else {
            vec![
                Scope { name: "4\" Refractor".to_string(),  aperture_mm: 102.0, focal_length_mm:  900.0 },
                Scope { name: "6\" Dobsonian".to_string(),  aperture_mm: 152.0, focal_length_mm: 1200.0 },
                Scope { name: "8\" Dobsonian".to_string(),  aperture_mm: 203.0, focal_length_mm: 1200.0 },
            ]
        }
    }

    pub fn effective_eyepieces(&self) -> Vec<Eyepiece> {
        if !self.eyepieces.is_empty() {
            self.eyepieces.clone()
        } else {
            vec![
                Eyepiece { name: "6mm".to_string(),  focal_length_mm:  6.0, afov_deg: 60.0 },
                Eyepiece { name: "9mm".to_string(),  focal_length_mm:  9.0, afov_deg: 52.0 },
                Eyepiece { name: "17mm".to_string(), focal_length_mm: 17.0, afov_deg: 68.0 },
                Eyepiece { name: "25mm".to_string(), focal_length_mm: 25.0, afov_deg: 52.0 },
                Eyepiece { name: "32mm".to_string(), focal_length_mm: 32.0, afov_deg: 52.0 },
            ]
        }
    }

    pub fn save(locations: &[Location], scopes: &[Scope], eyepieces: &[Eyepiece], max_mag: Option<f64>) {
        let Some(path) = config_path() else { return };
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let cfg = Config {
            locations: locations.to_vec(),
            max_mag,
            scopes: scopes.to_vec(),
            eyepieces: eyepieces.to_vec(),
        };
        if let Ok(text) = toml::to_string(&cfg) {
            let _ = std::fs::write(&path, text);
        }
    }
}
