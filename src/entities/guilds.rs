use crate::schema::guilds;
use diesel::*;

#[derive(Queryable, Insertable)]
#[table_name = "guilds"]
pub struct Guild {
    pub id: i64,
    pub channel_id: i64,
}

pub fn create_guild(conn: &PgConnection, guild_id: i64, channel_id: i64) -> Guild {
    let new_guild = Guild {
        id: guild_id,
        channel_id,
    };

    diesel::insert_into(guilds::table)
        .values(&new_guild)
        .get_result(conn)
        .expect(&format!("Couldn't insert guild {} in table", guild_id))
}

pub fn get_guild(conn: &PgConnection, guild_id: i64) -> Guild {
    use crate::schema::guilds::dsl::*;

    guilds
        .find(guild_id)
        .first(conn)
        .expect(&format!("Couldn't get guild {}", guild_id))
}

pub fn get_guilds(conn: &PgConnection) -> Vec<Guild> {
    use crate::schema::guilds::dsl::*;

    guilds.load::<Guild>(conn).expect("Couldn't get guilds")
}
