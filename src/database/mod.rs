pub mod models;

use diesel::prelude::*;
use diesel::PgConnection;
use dotenv;
use std::env;

pub fn connect() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("No database URL");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to database"))
}