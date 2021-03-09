use diesel::*;
use serde_json::json;
use std::collections::HashMap;
use crate::schema::messages;
use diesel::sql_types::{BigInt, Jsonb};

#[derive(Queryable, QueryableByName)]
pub struct SavedMessage {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "Jsonb"]
    pub embed_ids: serde_json::Value,
    #[sql_type = "Jsonb"]
    pub msg_ids: serde_json::Value,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage {
    pub embed_ids: serde_json::Value,
    pub msg_ids: serde_json::Value,
}

pub fn create_message(conn: &PgConnection, embed_ids: HashMap<i64, i64>, msg_ids: HashMap<i64,i64>) -> SavedMessage {
    let new_message = NewMessage {
        embed_ids: json!(embed_ids),
        msg_ids: json!(msg_ids),
    };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result(conn)
        .expect("This is fine")
}

pub fn find_message(conn: &PgConnection, id: i64, guild_id: i64) -> SavedMessage {
    diesel::sql_query(
        format!("SELECT * FROM messages WHERE messages.embed_ids->'{}' @> '{}' OR messages.msg_ids->'{}' @> '{}'", guild_id, id, guild_id, id))
        .get_results::<SavedMessage>(conn)
        .expect("...")
        .remove(0)
}
