use actix_web::{web::Data, App, HttpServer};
use db::Pool;
use diesel::{r2d2::ConnectionManager, PgConnection};
use utils::static_vars::{API_PORT, DATABASE_URL};

pub mod db;
pub mod handlers;
pub mod models;
pub mod utils;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = DATABASE_URL.to_string();

    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::new(connection_manager);
    let pool_data = Data::new(pool);

    let server = HttpServer::new(move || App::new().app_data(pool_data.clone()))
        .bind(("0.0.0.0", *API_PORT))
        .expect(&format!(
            "Could not bind 0.0.0.0 to {}. Is port already taken ?",
            *API_PORT
        ));

    println!("Starting Doggo-Drive on port {}", *API_PORT);

    server.run().await?;

    Ok(())
}
