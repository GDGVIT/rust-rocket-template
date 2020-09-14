#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate slog;

use slog::{Drain, Logger};
use slog_async;
use slog_term;

pub mod db;
pub mod api;
pub mod schemas;
pub mod diesel_schema;
pub mod models;
pub mod crud;


embed_migrations!("migrations");

fn run_migrations(logger: &Logger) {
    let result = embedded_migrations::run(&db::pool::pg_connection());
    if let Err(e) = result {
        error!(logger, "migration error: {}", e.to_string());
    }
}

#[get("/")]
fn health_check() -> &'static str {
    "OK"
}

fn main() {
    // Load env variables
    dotenv::dotenv().ok();

    // Set up logger
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());

    run_migrations(&logger);

    let mut rocket = rocket::ignite()
        .mount("/api", routes![health_check]);
    rocket = api::endpoints::fuel(rocket);
    rocket = api::catchers::fuel(rocket);
    rocket.manage(db::pool::pool()).manage(logger).launch();
}
