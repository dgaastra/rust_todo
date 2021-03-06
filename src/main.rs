#![feature(in_band_lifetimes)]
#![feature(backtrace)]

mod api;
mod database;
mod model;
mod tools;
mod webpage;

#[macro_use]
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate uuid;
extern crate actix_cors;
extern crate hex;

use std::env;
use actix_web::{web, App, HttpServer};
use crate::api::api::{regi, login, list, add, toggle, remove};
use crate::database::user_db::UserDB;
use crate::database::session_db::SessionDB;
use crate::database::todo_db::TodoDB;
use actix_web::middleware::Logger;
use actix_cors::Cors;

const DEFAULT_SERVER_ADDRESS: &str = "0.0.0.0:8001";
const USER_DB_PATH: &str = "todo_data/user_db.json";
const SESSION_DB_PATH: &str = "todo_data/session_db.json";
const TODO_DB_PATH: &str = "todo_data/todo_db.json";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug,actix_server=info");
    env_logger::init();

    let server_address = env::args().collect::<Vec<String>>().get(1)
        .map_or_else(|| String::from(DEFAULT_SERVER_ADDRESS), |arg| String::from(arg));

    let user_db = web::Data::new(UserDB::new(String::from(USER_DB_PATH)));
    let user_db_global = user_db.clone();
    let session_db = web::Data::new(SessionDB::new(String::from(SESSION_DB_PATH), &user_db));
    let session_db_global = session_db.clone();
    let todo_db = web::Data::new(TodoDB::new(String::from(TODO_DB_PATH), user_db.get_ref()));
    let todo_db_global = todo_db.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(user_db.clone())
            .app_data(session_db.clone())
            .app_data(todo_db.clone())
            .wrap(Cors::new()
                .send_wildcard()
                .finish())
            .wrap(Logger::default())
            .service(regi)
            .service(login)
            .service(list)
            .service(add)
            .service(toggle)
            .service(remove)
            .service(webpage::webpage)
    })
        .bind(server_address)?
        .run()
        .await?;

    user_db_global.save();
    session_db_global.save();
    todo_db_global.save();
    Ok(())
}
