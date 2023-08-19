use actix_web::web::Data;

use crate::handlers::error::DoggoError;

use super::Pool;

pub fn check_database_created(pool: Data<Pool>) -> Result<(), DoggoError> {
    let conn = &mut pool.get().unwrap();
    Ok(())
}