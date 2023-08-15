use actix_web::{App, HttpServer, web::{self, Data, JsonConfig}};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use dotenv::dotenv;

use models;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("Failed to create pool");

	HttpServer::new(move || {
		App::new()
			.app_data(Data::new(pool.clone()))
	})
	.bind("localhost:8080")?
	.run()
	.await
}