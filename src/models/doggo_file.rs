use chrono::{DateTime, Local};
use diesel::{deserialize::Queryable, sql_types::Uuid};

use crate::schema::doggo_files;

#[derive(Debug, Queryable)]
#[diesel(table_name = doggo_files)]
pub struct DoggoFile {
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    id: Uuid,
    local_path: String,
    net_path: String,
    file_name: Option<String>
}