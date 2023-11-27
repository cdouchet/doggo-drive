use chrono::{DateTime, Local};
use diesel::deserialize::Queryable;
use uuid::Uuid;
use crate::schema::users;


#[derive(Debug, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    id: Uuid,
    username: String,
    password: String
}