use std::net::TcpListener;

use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use tracing_actix_web::TracingLogger;

use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

pub fn run(
	listener: TcpListener,
	conn_pool: PgPool
) -> Result<Server, std::io::Error> {

	let conn_pool = web::Data::new(conn_pool);
	let server = HttpServer::new(move || {
		App::new()
			.wrap(TracingLogger::default())
			.route("/health_check", web::get().to(health_check))
			.route("/subscriptions", web::post().to(subscribe))
			.app_data(conn_pool.clone())
	})
	.listen(listener)?
	.run();

	Ok(server)
}
