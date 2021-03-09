use diesel::*;
use crate::schema::bans;
use crate::schema::bans::dsl::*;

#[derive(Queryable, Insertable)]
#[table_name = "bans"]
pub struct Ban {
    pub id: i64,
}

pub fn create_ban(conn: &PgConnection, user_id: i64) -> Ban {
    let new_ban = Ban { id: user_id };

    diesel::insert_into(bans::table)
        .values(&new_ban)
        .get_result(conn)
        .expect("This is fine")
}

pub fn is_banned(conn: &PgConnection, user_id: i64) -> bool {
    match bans.find(user_id).first::<Ban>(conn) {
        Ok(_) => true,
        _ => false,
    }
}

pub fn rm_ban(conn: &PgConnection, user_id: i64) {
    diesel::delete(bans.filter(id.eq(user_id)))
        .execute(conn)
        .expect("This is fine");
}
