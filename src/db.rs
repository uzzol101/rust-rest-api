use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;


pub fn connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("database url not set");
    PgConnection::establish(&database_url).expect("failed to connect to database")
}