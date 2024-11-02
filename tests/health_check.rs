mod error;

use color_eyre::Result;
use error::SpawnAppErr;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

async fn spawn_app() -> Result<String, SpawnAppErr> {
    let listener =
        TcpListener::bind("127.0.0.1:0").map_err(|source| SpawnAppErr::Listen { source })?;
    let port = listener
        .local_addr()
        .map_err(|source| SpawnAppErr::GetListenPort { source })?
        .port();
    let configuration = get_configuration();
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .map_err(|source| SpawnAppErr::PostgresConnection { source })?;
    let server = zero2prod::startup::run(listener, connection).expect("Failed to bind address");
    let server_task = tokio::spawn(server);
    std::mem::drop(server_task);
    Ok(format!("http://127.0.0.1:{port}"))
}

#[tokio::test]
async fn health_check_works() -> Result<()> {
    // color_eyre::install()?;
    let address = spawn_app().await?;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    Ok(())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() -> Result<()> {
    let address = spawn_app().await?;
    let configuration = get_configuration();

    let client = reqwest::Client::new();
    let body = "name=Alison%20Jenkins&email=not_my_email%40nomail.com";
    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;

    assert_eq!(200, response.status().as_u16());

    let connection_string = configuration.database.connection_string();
    let mut db_connection = PgConnection::connect(&connection_string).await?;

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut db_connection)
        .await?;

    assert_eq!(saved.email, "ursula_de_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    Ok(())
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() -> Result<()> {
    // color_eyre::install()?;

    let address = spawn_app().await?;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{address}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await?;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
    Ok(())
}
