use dotenvy::dotenv;
use std::env;
#[macro_use] extern crate rocket;
use rocket::{Build, Rocket};

use crate::routes::*;
mod schema;
mod models;
mod db;
mod static_files;
mod routes;

pub fn rocket() -> Rocket<Build>{
dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(&database_url);

    rocket::build().manage(pool).mount("/api/v1/", routes![index, new, show, update, delete,player]).mount("/", routes![static_files::all, static_files::index])
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket().launch().await.map(|_rocket| ())
}