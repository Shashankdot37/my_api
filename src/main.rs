use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;
#[macro_use] extern crate rocket;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use serde_json::json;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
mod schema;
mod models;
mod db;
mod static_files;

pub fn rocket() -> Rocket<Build>{
dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(&database_url);

    rocket::build().manage(pool).mount("/", routes![static_files::all, static_files::index])
}

fn main(){
    
}