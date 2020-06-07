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
use crate::task::{Task, Todo};
use diesel::SqliteConnection;
use libchordr::models::catalog::Catalog;
use libchordr::prelude::{CatalogBuilder, FileType};
use rocket::fairing::AdHoc;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, NamedFile, Redirect};
use rocket::{Rocket, State};
use rocket_contrib::{json::Json, serve::StaticFiles, templates::Template};
use std::io;
use std::path::Path;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

pub type ConnectionType = SqliteConnection;

#[database("main_database")]
pub struct DbConn(ConnectionType);

#[derive(Debug, Serialize)]
struct Context<'a, 'b> {
    msg: Option<(&'a str, &'b str)>,
    tasks: Vec<Task>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &DbConn, msg: &'a str) -> Context<'static, 'a> {
        Context {
            msg: Some(("error", msg)),
            tasks: Task::all(&conn.0),
        }
    }

    pub fn raw(conn: &DbConn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context {
            msg: msg,
            tasks: Task::all(&conn.0),
        }
    }
}

#[post("/", data = "<todo_form>")]
fn new(todo_form: Form<Todo>, conn: DbConn) -> Flash<Redirect> {
    let todo = todo_form.into_inner();
    if todo.description.is_empty() {
        Flash::error(Redirect::to("/"), "Description cannot be empty.")
    } else if Task::insert(todo, &conn.0) {
        Flash::success(Redirect::to("/"), "Todo successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Whoops! The server failed.")
    }
}

#[post("/", data = "<todo_form>")]
fn ne2w(todo_form: Form<Todo>, conn: DbConn) -> Flash<Redirect> {
    let todo = todo_form.into_inner();
    if todo.description.is_empty() {
        Flash::error(Redirect::to("/"), "Description cannot be empty.")
    } else if Task::insert(todo, &conn.0) {
        Flash::success(Redirect::to("/"), "Todo successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Whoops! The server failed.")
    }
}

#[put("/<id>")]
fn toggle(id: i32, conn: DbConn) -> Result<Redirect, Template> {
    if Task::toggle_with_id(id, &conn.0) {
        Ok(Redirect::to("/"))
    } else {
        Err(Template::render(
            "index",
            &Context::err(&conn, "Couldn't toggle task."),
        ))
    }
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConn) -> Result<Flash<Redirect>, Template> {
    if Task::delete_with_id(id, &conn.0) {
        Ok(Flash::success(Redirect::to("/"), "Todo was deleted."))
    } else {
        Err(Template::render(
            "index",
            &Context::err(&conn, "Couldn't delete task."),
        ))
    }
}

#[get("/")]
fn index(config: State<Config>) -> io::Result<NamedFile> {
    NamedFile::open(Path::new(&config.static_files_dir).join("index.html"))
}

#[get("/template")]
fn index_template(msg: Option<FlashMessage<'_, '_>>, conn: DbConn) -> Template {
    Template::render(
        "index",
        &match msg {
            Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
            None => Context::raw(&conn, None),
        },
    )
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
        // .mount("/todo", routes![new, toggle, delete])
        .mount("/setlist", crate::routes::setlist::get_routes())
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

fn main() {
    rocket().launch();
}
