use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpawnAppErr {
    #[error("Error listening on TCP port: {source}")]
    Listen { source: std::io::Error },

    #[error("Error getting configuration: {source}")]
    GetConfiguration { source: config::ConfigError },

    #[error("Error getting the listen port: {source}")]
    GetListenPort { source: std::io::Error },

    #[error("Error configuring the database: {source}")]
    ConfigureDatabase { source: ConfigureDatabaseError },
}

#[derive(Error, Debug)]
pub enum ConfigureDatabaseError {
    #[error("Error connecting to Postgres: {source}")]
    PostgresConnection { source: sqlx::Error },

    #[error("Error creating the database '{db_name}': {source}")]
    CreateDatabase {
        source: sqlx::Error,
        db_name: String,
    },

    #[error("Postgres Pool connection error: {source}")]
    PostgresPool { source: sqlx::Error },

    #[error("Error while running database migrations: {source}")]
    Migrations { source: sqlx::migrate::MigrateError },
}
