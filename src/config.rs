use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub adapter: String,
    pub database: String,
}

pub fn load_config() -> anyhow::Result<Config> {
    let content = std::fs::read_to_string("kairo.config")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
