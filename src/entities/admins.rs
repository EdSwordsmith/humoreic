use diesel::*;
use crate::schema::admins;
use crate::schema::admins::dsl::*;

#[derive(Queryable, Insertable)]
#[table_name = "admins"]
pub struct Admin {
    pub id: i64,
}

pub fn create_admin(conn: &PgConnection, user_id: i64) -> Admin {
    let new_admin = Admin { id: user_id };

    diesel::insert_into(admins::table)
        .values(&new_admin)
        .get_result(conn)
        .expect("This is fine")
}

pub fn is_admin(conn: &PgConnection, user_id: i64) -> bool {
    match admins.find(user_id).first::<Admin>(conn) {
        Ok(_) => true,
        _ => false,
    }
}
