#[macro_use]
extern crate diesel;

use crate::diesel::*;
use crate::models::*;
use diesel::{pg::PgConnection, sql_types::{BigInt, Text}, types::VarChar};
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;
use serde_json::json;
use std::collections::HashMap;

pub mod models;
pub mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}

pub fn create_guild(conn: &PgConnection, guild_id: i64, channel_id: i64) -> Guild {
    use schema::guilds;

    let new_guild = Guild {
        id: guild_id,
        channel_id: channel_id,
    };

    diesel::insert_into(guilds::table)
        .values(&new_guild)
        .get_result(conn)
        .expect("This is fine")
}

pub fn get_guild(conn: &PgConnection, guild_id: i64) -> Guild {
    use schema::guilds::dsl::*;

    guilds.find(guild_id).first(conn).expect("Bruh 2.0")
}

pub fn get_guilds(conn: &PgConnection) -> Vec<Guild> {
    use schema::guilds::dsl::*;

    guilds.load::<Guild>(conn).expect("Error loading guilds")
}

pub fn create_admin(conn: &PgConnection, user_id: i64) -> Admin {
    use schema::admins;

    let new_admin = Admin { id: user_id };

    diesel::insert_into(admins::table)
        .values(&new_admin)
        .get_result(conn)
        .expect("This is fine")
}

pub fn is_admin(conn: &PgConnection, user_id: i64) -> bool {
    use schema::admins::dsl::*;

    match admins.find(user_id).first::<Admin>(conn) {
        Ok(_) => true,
        _ => false,
    }
}

pub fn create_ban(conn: &PgConnection, user_id: i64) -> Ban {
    use schema::bans;

    let new_ban = Ban { id: user_id };

    diesel::insert_into(bans::table)
        .values(&new_ban)
        .get_result(conn)
        .expect("This is fine")
}

pub fn is_banned(conn: &PgConnection, user_id: i64) -> bool {
    use schema::bans::dsl::*;

    match bans.find(user_id).first::<Ban>(conn) {
        Ok(_) => true,
        _ => false,
    }
}

pub fn rm_ban(conn: &PgConnection, user_id: i64) {
    use schema::bans::dsl::*;

    diesel::delete(bans.filter(id.eq(user_id)))
        .execute(conn)
        .expect("This is fine");
}

pub fn create_message(conn: &PgConnection, embed_ids: HashMap<i64, i64>, msg_ids: HashMap<i64,i64>) -> SavedMessage {
    use schema::messages;

    let new_message = NewMessage {
        embed_ids: json!(embed_ids),
        msg_ids: json!(msg_ids),
        reactions: json!({}),
    };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result(conn)
        .expect("This is fine")
}

pub fn find_message(conn: &PgConnection, id: i64, guild_id: i64) -> Vec<SavedMessage> {
    diesel::sql_query(
        format!("SELECT * FROM messages WHERE messages.embed_ids->'{}' @> '{}' OR messages.msg_ids->'{}' @> '{}'", guild_id, id, guild_id, id))
        .get_results::<SavedMessage>(conn)
        .expect("...")
}
