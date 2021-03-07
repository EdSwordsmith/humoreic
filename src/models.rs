extern crate serde_json;

use super::schema::{admins, bans, guilds, messages};
use diesel::sql_types::BigInt;
use diesel::pg::types::sql_types::Jsonb;

#[derive(Queryable, Insertable)]
#[table_name = "guilds"]
pub struct Guild {
    pub id: i64,
    pub channel_id: i64,
}

#[derive(Queryable, QueryableByName)]
pub struct SavedMessage {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "Jsonb"]
    pub embed_ids: serde_json::Value,
    #[sql_type = "Jsonb"]
    pub msg_ids: serde_json::Value,
    #[sql_type = "Jsonb"]
    pub reactions: serde_json::Value,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage {
    pub embed_ids: serde_json::Value,
    pub msg_ids: serde_json::Value,
    pub reactions: serde_json::Value,
}

#[derive(Queryable, Insertable)]
#[table_name = "admins"]
pub struct Admin {
    pub id: i64,
}

#[derive(Queryable, Insertable)]
#[table_name = "bans"]
pub struct Ban {
    pub id: i64,
}
