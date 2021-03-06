use super::schema::guilds;

#[derive(Queryable, Insertable)]
#[table_name="guilds"]
pub struct Guild {
    pub id: i64,
    pub channel_id: i64,
}
