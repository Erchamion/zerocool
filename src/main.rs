use std::{net::TcpListener};
use secrecy::ExposeSecret;
use sqlx::PgPool;

use zerocool::{startup::run, configuration::get_configuration, telemetry::{get_subscriber, init_subscriber}};

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let subscriber = get_subscriber(
		"zerocool".into(),
		"info".into(),
		std::io::stdout
	);
	init_subscriber(subscriber);

	let settings = get_configuration().expect("Failed to read configuration");
	let conn_pool = PgPool::connect_lazy(
		&settings.database.connection_string().expose_secret()
	)
	.expect("Failed to connect to Postgres");

	let host = format!("{}:{}", settings.application.host, settings.application.port);
	let listener = TcpListener::bind(host).expect("Failed to start tcp listener");

	run(listener, conn_pool)?.await
}
