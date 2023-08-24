use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer, web::{self, Data, JsonConfig}, http::header};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use dotenv::dotenv;
use routes::*;

mod schema;

mod models;
mod routes;
mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("Failed to create pool");

	HttpServer::new(move || {
		let cors = Cors::default()
			.allowed_header(header::CONTENT_TYPE)
			.allowed_origin("http://localhost:3000")
			.allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
			.allow_any_origin();

		App::new()
			.wrap(cors)
			.wrap(Logger::default())
			.app_data(Data::new(pool.clone()))
			.app_data(JsonConfig::default())
			.service(web::scope("/cable_type").configure(cable_type_routes::config))
			.service(web::scope("/cable").configure(cable_routes::config))
	})
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}