pub use config::{ConfigError, File, FileFormat};

pub fn get_config() -> config::Config {
    let cfg = config::Config::builder()
        .add_source(File::new("config/settings.prod.json", FileFormat::Json))
        .add_source(config::Environment::default())
        .build();

    cfg.ok().unwrap()
}
