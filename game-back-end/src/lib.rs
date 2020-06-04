#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

extern crate serde_json;

extern crate serde;

//datetime
extern crate chrono;

extern crate rand;

extern crate userInfo;

#[macro_use]
extern crate diesel;


use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub mod qa;
pub mod user;
pub mod models;
pub mod schema;
pub mod datetime;