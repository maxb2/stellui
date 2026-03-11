use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub height: Option<f64>,
    pub timezone: Option<String>,
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

    pub fn save(&self) -> anyhow::Result<()> {
        let path = config_path().ok_or_else(|| anyhow::anyhow!("could not determine config path"))?;
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let text = toml::to_string(self)?;
        std::fs::write(&path, text)?;
        Ok(())
    }
}
