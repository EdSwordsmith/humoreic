use diesel::*;
use crate::schema::guilds;

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
        .expect("This is fine")
}

pub fn get_guild(conn: &PgConnection, guild_id: i64) -> Guild {
    use crate::schema::guilds::dsl::*;

    guilds.find(guild_id).first(conn).expect("Bruh 2.0")
}

pub fn get_guilds(conn: &PgConnection) -> Vec<Guild> {
    use crate::schema::guilds::dsl::*;

    guilds.load::<Guild>(conn).expect("Error loading guilds")
}
