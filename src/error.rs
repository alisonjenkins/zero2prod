use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErr {
    #[error("Error while installing color_eyre handler: {source}")]
    ColorEyreInstall { source: color_eyre::eyre::Error },

    #[error("Error while getting the configuration: {source}")]
    GetConfiguration { source: config::ConfigError },

    #[error("Error while connecting to Postgres: {source}")]
    PostgresConnection { source: sqlx::Error },

    #[error("Error while listening on TCP port: {source}")]
    Listen { source: std::io::Error },

    #[error("Error while running the application server: {source}")]
    RunServer { source: std::io::Error },
}
