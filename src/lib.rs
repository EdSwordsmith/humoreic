#[macro_use]
extern crate diesel;

use crate::models::Guild;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
use dotenv::dotenv;
use std::env;
use crate::diesel::*;

pub mod schema;
pub mod models;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
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

    guilds.filter(id.eq(guild_id)).first(conn).expect("Bruh 2.0")
}

pub fn get_guilds(conn: &PgConnection) -> Vec<Guild> {
    use schema::guilds::dsl::*;

    guilds.load::<Guild>(conn)
        .expect("Error loading guilds")
}
