use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpawnAppErr {
    #[error("Error listening on TCP port: {source}")]
    Listen { source: std::io::Error },

    #[error("Error getting the listen port: {source}")]
    GetListenPort { source: std::io::Error },

    #[error("Error connecting to Postgres: {source}")]
    PostgresConnection { source: sqlx::Error },
}
