pub mod init;

pub type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
