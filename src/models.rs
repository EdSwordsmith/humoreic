extern crate serde_json;

use super::schema::{admins, bans, guilds, messages};

#[derive(Queryable, Insertable)]
#[table_name = "guilds"]
pub struct Guild {
    pub id: i64,
    pub channel_id: i64,
}

#[derive(Queryable, Insertable)]
#[table_name = "messages"]
pub struct SavedMessage {
    pub id: i64,
    pub others: serde_json::Value,
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
