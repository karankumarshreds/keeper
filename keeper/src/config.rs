use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RootConfig {
    pub port: usize,
    pub servers: Vec<Upstream>,
}

#[derive(Deserialize, Debug)]
pub struct Upstream {
    pub address: String,
    pub port: usize,
}

pub fn load() -> Result<RootConfig, Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
        .build()
        .expect("should load config");
    let port = config.get("port").expect("should have port");
    let servers = config.get("servers").expect("should have port");
    Ok(RootConfig { port, servers })
}
