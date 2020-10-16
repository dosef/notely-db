#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

use rocket_cors::{AllowedOrigins, Cors};

mod database;
mod models;
mod routes;
mod schema;
use routes::*;

embed_migrations!();

fn main() {
    let connection = database::establish_connection();
    embedded_migrations::run(&connection).unwrap();

    let cors = configure_cors().unwrap();
    rocket::ignite()
        .mount(
            "/",
            routes![
                add_list,
                add_item,
                get_lists,
                get_list,
                delete_list,
                delete_item
            ],
        )
        .attach(cors)
        .launch();
}

fn configure_cors() -> Result<Cors, rocket_cors::Error> {
    let allowed_origins = AllowedOrigins::All;
    rocket_cors::CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
}
