#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use actix_web::{HttpServer, App};
use dotenv::dotenv;
use std::env;

mod user;
mod schema;
mod db;

use user::model::{User, Newuser};
use user::routes::init_routes;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").expect("host not set");
    let port = env::var("PORT").expect("port not set");


    HttpServer::new(|| {
        App::new().configure(init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}