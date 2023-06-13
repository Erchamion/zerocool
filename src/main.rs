use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zerocool::{
    configuration::get_configuration,
    email_client::EmailClient,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zerocool".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = get_configuration().expect("Failed to read configuration");

    let sender_email = settings
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        settings.email_client.base_url,
        sender_email,
        settings.email_client.authorization_token,
    );

    let conn_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(settings.database.with_db());

    let host = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );
    let listener = TcpListener::bind(host).expect("Failed to start tcp listener");

    run(listener, conn_pool, email_client)?.await
}
