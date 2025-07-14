mod error;

use color_eyre::Result;
use error::SpawnAppErr;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app(db_pool: PgPool) -> Result<TestApp, SpawnAppErr> {
    let listener =
        TcpListener::bind("127.0.0.1:0").map_err(|source| SpawnAppErr::Listen { source })?;
    let port = listener
        .local_addr()
        .map_err(|source| SpawnAppErr::GetListenPort { source })?
        .port();
    // let mut configuration =
    //     get_configuration().map_err(|source| SpawnAppErr::GetConfiguration { source })?;

    // configuration.database.database_name = Uuid::new_v4().to_string();

    // let db_pool = configure_database(&configuration.database)
    //     .await
    //     .map_err(|source| SpawnAppErr::ConfigureDatabase { source })?;

    // *TODO* Fix expect
    let server =
        zero2prod::startup::run(listener, db_pool.clone()).expect("Failed to bind address");
    let server_task = tokio::spawn(server);
    std::mem::drop(server_task);

    let address = format!("http://127.0.0.1:{port}");

    Ok(TestApp { address, db_pool })
}

#[sqlx::test]
async fn health_check_works(pool: PgPool) -> Result<()> {
    let test_app = spawn_app(pool.clone()).await?;
    let address = test_app.address;

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

#[sqlx::test]
async fn subscribe_returns_a_200_for_valid_form_data(pool: PgPool) -> Result<()> {
    let test_app = spawn_app(pool.clone()).await?;
    let address = test_app.address;

    let client = reqwest::Client::new();
    let body = "name=Alison%20Jenkins&email=not_my_email%40nomail.com";
    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&pool)
        .await?;

    assert_eq!(saved.email, "not_my_email@nomail.com");
    assert_eq!(saved.name, "Alison Jenkins");
    Ok(())
}

#[sqlx::test]
async fn subscribe_returns_400_when_data_is_missing(pool: PgPool) -> Result<()> {
    let test_app = spawn_app(pool.clone()).await?;
    let address = test_app.address;
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
            "The API did not fail with 400 Bad Request when the payload was {error_message}.",
        );
    }
    Ok(())
}
