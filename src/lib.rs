#[macro_use]
extern crate diesel;

use crate::diesel::*;
use crate::models::*;
use diesel::pg::PgConnection;
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

pub fn create_reaction(conn: &PgConnection, message_id: i64, reaction: &String, 
    user_id: i64) -> SavedReaction {
    use schema::reactions;

    let new_reaction = NewReaction {
        message_id,
        reaction: (*reaction).clone(),
        user_id
    };

    diesel::insert_into(reactions::table)
        .values(new_reaction)
        .get_result(conn)
        .expect("pls dont kill me")
}

pub fn delete_reaction(conn: &PgConnection, reaction_id: i64) {
    use schema::reactions::dsl::*;

    diesel::delete(reactions.filter(id.eq(reaction_id))).execute(conn);
}

pub fn get_reactions(conn: &PgConnection, message_id: i64) -> HashMap::<String, Vec<SavedReaction>> {
    use schema::messages;
    use schema::reactions;

    let reactions: Vec<SavedReaction> = reactions::table
        .inner_join(messages::table
        .on(reactions::message_id.eq(messages::id)
        .and(reactions::message_id.eq(message_id))))
        .select((reactions::id, reactions::reaction, 
            reactions::message_id, reactions::user_id))
        .load(conn)
        .expect("lul");

    
    let mut reactions_group = HashMap::<String, Vec<SavedReaction>>::new();
    for r in reactions.iter() {
        if !reactions_group.contains_key(&r.reaction) {
            reactions_group.insert(r.reaction.clone(), Vec::new());
        }

        reactions_group.get_mut(&r.reaction).expect("get me out")
            .push((*r).clone());
    }

    return reactions_group;
}

pub fn has_reaction(reactions: HashMap::<String, Vec<SavedReaction>>, reaction: &String, user_id: i64) -> bool {
    let reactions: &Vec<SavedReaction> = reactions.get(reaction).expect("aiaiai");
    for r in reactions.iter(){
        if r.user_id == user_id {
            return true;
        }
    }

    false
}

/*pub fn update_message(conn: &PgConnection, message_id: i64, reactions_map: Map<String, serde_json::Value>) -> SavedMessage {
    use schema::messages::dsl::*;

    diesel::update(messages.find(message_id))
        .set(reactions.eq(json!(reactions_map)))
        .get_result::<SavedMessage>(conn)
        .expect("...")
}*/
