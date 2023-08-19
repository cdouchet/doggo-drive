use lazy_static::lazy_static;
use std::env::var;

lazy_static! {
    pub static ref DATABASE_URL: String = var("DATABASE_URL").expect("DATABASE_URL env var must be set");
}