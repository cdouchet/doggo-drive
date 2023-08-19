use utils::static_vars::DATABASE_URL;

pub mod utils;
pub mod db;
pub mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = DATABASE_URL.to_string();

    Ok(())
}