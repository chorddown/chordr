#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

mod authentication;
#[deprecated(note = "Use cqrs::prelude::* instead")]
mod command;
mod config;
mod domain;
mod error;
mod routes;
mod schema;
mod task;
#[cfg(test)]
mod test_helpers;
mod traits;

use crate::config::Config;
use diesel::SqliteConnection;
use libchordr::models::catalog::Catalog;
use libchordr::prelude::{CatalogBuilder, FileType};
use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket::response::NamedFile;
use rocket::{Rocket, State};
use rocket_contrib::{json::Json, serve::StaticFiles, templates::Template};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};
use std::io;
use std::path::Path;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

pub type ConnectionType = SqliteConnection;

#[database("main_database")]
pub struct DbConn(ConnectionType);

#[get("/")]
fn index(config: State<Config>) -> io::Result<NamedFile> {
    NamedFile::open(Path::new(&config.static_files_dir).join("index.html"))
}

#[get("/catalog.json")]
fn catalog(config: State<Config>) -> Result<Json<Catalog>, libchordr::prelude::Error> {
    match CatalogBuilder::new().build_catalog_for_directory(
        &config.song_dir,
        FileType::Chorddown,
        true,
    ) {
        Err(e) => Err(e),
        Ok(catalog_result) => {
            if catalog_result.errors.len() > 0 {
                for error in catalog_result.errors {
                    log::error!("{}", error);
                }
            }

            Ok(Json(catalog_result.catalog))
        }
    }
}

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConn::get_one(&rocket).expect("Database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(build_cors_fairing())
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .attach(AdHoc::on_attach(
            "Build application configuration",
            |rocket| {
                let config = build_application_config(&rocket);
                Ok(rocket.manage(config))
            },
        ))
        .attach(AdHoc::on_attach("Static Files config", |rocket| {
            let config = build_application_config(&rocket);
            Ok(rocket.mount("/", StaticFiles::from(config.static_files_dir)))
        }))
        // .mount("/", StaticFiles::from("static/"))
        // .mount("/", StaticFiles::from("../target/deploy/"))
        .mount("/", routes![index, catalog])
        .mount("/status", crate::routes::status::get_routes())
        // .mount("/todo", routes![new, toggle, delete])
        .mount("/setlist", crate::routes::setlist::get_routes())
        .mount("/user", crate::routes::user::get_routes())
        .attach(Template::fairing())
}

fn build_application_config(rocket: &Rocket) -> Config {
    let static_files_dir = rocket
        .config()
        .get_string("static_files_dir")
        .expect("Static files directory is not specified");
    let song_dir = rocket
        .config()
        .get_str("song_dir")
        .expect("Song directory is not specified")
        .to_string();

    Config {
        song_dir,
        static_files_dir,
    }
}

fn build_cors_fairing() -> Cors {
    let allowed_origins =
        AllowedOrigins::some_exact(&["http://localhost:8000", "http://localhost:9000"]);

    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Invalid CORS configuration")
}

fn main() {
    rocket().launch();
}
