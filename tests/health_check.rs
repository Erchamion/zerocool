use std::net::TcpListener;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use uuid::Uuid;
use once_cell::sync::Lazy;
use zerocool::{configuration::{get_configuration, DatabaseSettings}, telemetry::{get_subscriber, init_subscriber}};

static TRACING: Lazy<()> = Lazy::new(|| {
	let default_filter_level = "info".to_string();
	let subscriber_name = "test".to_string();

	if std::env::var("TEST_LOG").is_ok() {
		let subscriber = get_subscriber(
			subscriber_name,
			default_filter_level,
			std::io::stdout
		);
		init_subscriber(subscriber);
	} else {
		let subscriber = get_subscriber(
			subscriber_name,
			default_filter_level,
			std::io::sink
		);
		init_subscriber(subscriber);
	}

});

pub struct TestApp {
	pub address: String,
	pub conn_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
	let test_app = spawn_app().await;
	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/health_check", &test_app.address))
		.send()
		.await
		.expect("Failed to execute request");

	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
	let test_app = spawn_app().await;
	let client = reqwest::Client::new();

	let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
	let response = client
		.post(&format!("{}/subscriptions", &test_app.address))
		.header("Content-Type", "application/x-www-form-urlencoded")
		.body(body)
		.send()
		.await
		.expect("Failed to execute request");

	assert_eq!(200, response.status().as_u16());

	let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
		.fetch_one(&test_app.conn_pool)
		.await
		.expect("Failed to fetch saved subscription");

	assert_eq!(saved.email, "ursula_le_guin@gmail.com");
	assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
	// ARRANGE
	let app = spawn_app().await;
	let client = reqwest::Client::new();
	let test_cases = vec![
		("name=&email=ursula_le_quin%40gmail.com", "empty name"),
		("name=Ursula&email=", "empty email"),
		("name=Ursula&email=not-an-email", "invalid email"),
	];

	for (body, description) in test_cases {
		// ACT
		let response = client
			.post(&format!("{}/subscriptions", &app.address))
			.header("Content-Type", "application/x-www-form-urlencoded")
			.body(body)
			.send()
			.await
			.expect("Failed to execute request.");

		assert_eq!(
			400,
			response.status().as_u16(),
			"The API did not return a 400 Bad Rquest when the payload was {}.",
			description
		);
	}
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
	let test_app = spawn_app().await;
	let client = reqwest::Client::new();
	let test_cases = vec![
		("name=le%20guin", "missing email"),
		("email=ursurla_le_%40gmail.com", "missing name"),
		("", "missing both name and email")
	];

	for (invalid_body, error_message) in test_cases {
		let response = client
			.post(&format!("{}/subscriptions", &test_app.address))
			.header("Content-Type", "application/x-www-form-urlencoded")
			.body(invalid_body)
			.send()
			.await
			.expect("Failed to execute request");

		assert_eq!(
			400,
			response.status().as_u16(),
			"The API did not fail with 400 bad request when the payload was {}",
			error_message
		);
	}
}

async fn spawn_app() -> TestApp {
	Lazy::force(&TRACING);

	let listener = TcpListener::bind("127.0.0.1:0")
		.expect("Failed to start tcp listener");

	let port = listener.local_addr().unwrap().port();
	let host = format!("http://127.0.0.1:{}", port);

	let mut settings = get_configuration().expect("Failed to read configuration");
	settings.database.database_name = Uuid::new_v4().to_string();
	let conn_pool = configure_database(&settings.database).await;

	let server = zerocool::startup::run(listener, conn_pool.clone())
		.expect("Failed to bind address");

	let _ = tokio::spawn(server);

	TestApp { address: host, conn_pool }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
	let mut connection = PgConnection::connect_with(&config.without_db())
		.await
		.expect("Failed to connect to Postgres");

	connection
		.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
		.await
		.expect("Failed to create database");

	let conn_pool = PgPool::connect_with(config.with_db())
		.await
		.expect("Failed to connect to Postgres");

	sqlx::migrate!("./migrations")
		.run(&conn_pool)
		.await
		.expect("Failed to migrate the database");

	conn_pool
}
