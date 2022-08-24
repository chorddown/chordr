//noinspection RsMainFunctionNotFound
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use std::io;
use std::path::Path;

use diesel::SqliteConnection;
use rocket::fairing::AdHoc;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{http, Build, Rocket, State};
use rocket_sync_db_pools::database;

use libchordr::models::catalog::Catalog;
use libchordr::prelude::{CatalogBuilder, FileType};

use crate::config::Config;

mod authentication;
mod config;
mod cors;
mod domain;
mod error;
mod routes;
mod schema;
#[cfg(test)]
mod test_helpers;
mod traits;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

pub type ConnectionType = SqliteConnection;

#[database("main_database")]
pub struct DbConn(ConnectionType);

#[get("/")]
async fn index(config: &State<Config>) -> io::Result<NamedFile> {
    NamedFile::open(Path::new(&config.static_files_dir).join("index.html")).await
}

#[get("/<_..>", rank = 20)]
async fn html_fallback(config: &State<Config>) -> io::Result<NamedFile> {
    NamedFile::open(Path::new(&config.static_files_dir).join("index.html")).await
}

#[get("/catalog.json")]
fn catalog(config: &State<Config>) -> Result<Json<Catalog>, status::Custom<String>> {
    match CatalogBuilder::new().build_catalog_for_directory(
        &config.song_dir,
        FileType::Chorddown,
        true,
    ) {
        Err(e) => Err(status::Custom(
            http::Status::InternalServerError,
            e.to_string(),
        )),
        Ok(catalog_result) => {
            if !catalog_result.errors.is_empty() {
                for error in catalog_result.errors {
                    log::error!("{}", error);
                }
            }

            Ok(Json(catalog_result.catalog))
        }
    }
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    let conn = DbConn::get_one(&rocket)
        .await
        .expect("Database connection could not be established");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("Diesel migrations failed");

    rocket
}

fn rocket_build() -> Rocket<Build> {
    rocket::build()
        .attach(self::cors::Cors)
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Database Migrations", run_db_migrations))
        .attach(AdHoc::on_ignite(
            "Build application configuration",
            |rocket| async {
                let config = build_application_config(&rocket);
                rocket.manage(config)
            },
        ))
        .attach(AdHoc::on_ignite("Static Files config", |rocket| async {
            let config = build_application_config(&rocket);
            rocket.mount("/", FileServer::from(config.static_files_dir).rank(1))
        }))
        .mount("/", routes![index, catalog])
        .mount("/api/status", routes::status::get_routes())
        .mount("/api/setlist", routes::setlist::get_routes())
        .mount("/api/user", routes::user::get_routes())
        .mount("/", routes![html_fallback])
}

fn build_application_config(rocket: &Rocket<Build>) -> Config {
    rocket
        .figment()
        .extract()
        .expect("Could not deserialize the configuration")
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket_build()
}
