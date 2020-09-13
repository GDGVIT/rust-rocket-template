use anyhow::Result;
use rocket::Rocket;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid as RocketUuid;
use uuid::Uuid;

use crate::crud::users;
use crate::db::guard::DbConn;
use crate::models::users::User;
use crate::schemas::users::UserCreate;

#[post("/", format = "json", data = "<user>")]
fn create(user: Json<UserCreate>, db: DbConn) -> Result<Json<User>> {
    let inserted_user = users::create(&db, user.0)?;
    Ok(Json(inserted_user))
}

#[get("/<id>")]
fn fetch_by_id(id: RocketUuid, db: DbConn) -> Result<Json<User>> {
    let uuid = Uuid::from_bytes(*id.as_bytes());
    let found_user = users::find(&db, uuid)?;
    Ok(Json(found_user))
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/api/users", routes![create, fetch_by_id])
}