use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, Connection};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn pg_connection() -> PgConnection {
    PgConnection::establish(database_url().as_str()).unwrap()
}
