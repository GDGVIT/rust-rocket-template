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

pub mod api;
pub mod crud;
pub mod db;
pub mod diesel_schema;
pub mod models;
pub mod schemas;

#[cfg(test)]
mod tests;

embed_migrations!("migrations");

#[get("/")]
fn health_check() -> &'static str {
    "OK"
}

fn rocket() -> rocket::Rocket {
    embedded_migrations::run(&db::pool::pg_connection()).expect("expected successful migration");
    let mut rocket = rocket::ignite().mount("/api", routes![health_check]);
    rocket = api::endpoints::fuel(rocket);
    rocket = api::catchers::fuel(rocket);
    rocket.manage(db::pool::pool())
}

fn main() {
    // Load env variables
    dotenv::dotenv().ok();

    // Launch rocket instance
    rocket().launch();
}
