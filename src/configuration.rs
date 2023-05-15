#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name("configuration"))
            .build()?
            .try_deserialize()
    }
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub database_name: String,
    pub host: String,
    pub password: String,
    pub port: u16,
    pub username: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Settings {
    match Settings::new() {
        Ok(settings) => settings,
        Err(error) => {
            eprintln!("Failed to load configuration: {}", error);
            std::process::exit(1);
        }
    }
}
