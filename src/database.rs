use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::PgConnection;
use serenity::client::Context;
use std::env;

use crate::handler::DBConnection;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}

pub async fn get_db_connection(ctx: &Context) -> PgPooledConnection {
    let data = ctx.data.read().await;
    let pool = data.get::<DBConnection>().unwrap();
    pool.get().unwrap()
}
