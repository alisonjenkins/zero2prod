use color_eyre::eyre::Result;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, error::AppErr, startup::run};

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    color_eyre::install().map_err(|source| AppErr::ColorEyreInstall { source })?;
    let configuration =
        get_configuration().map_err(|source| AppErr::GetConfiguration { source })?;
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .map_err(|source| AppErr::PostgresConnection { source })?;
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).map_err(|source| AppErr::Listen { source })?;
    let _server = run(listener, connection_pool).map_err(|source| AppErr::RunServer { source })?;
    Ok(())
}
