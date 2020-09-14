use anyhow::Result;
use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::diesel_schema::users;
use crate::models::users::User;
use crate::schemas::users::{UserCreate, UserUpdate};

pub fn create(db: &PgConnection, user: UserCreate) -> Result<User> {
    use crate::diesel_schema::users::dsl::*;
    let mut new_user = User {
        id: Uuid::new_v4(),
        username: user.username,
        is_active: user.is_active,
    };
    new_user = insert_into(users)
        .values(&new_user)
        .on_conflict_do_nothing()
        .get_result(db)?;
    Ok(new_user)
}

pub fn find(db: &PgConnection, id: Uuid) -> Result<User> {
    let user = users::table.find(id).get_result::<User>(db)?;
    Ok(user)
}

pub fn find_by_name(db: &PgConnection, find_username: String) -> Result<User> {
    use crate::diesel_schema::users::dsl::*;
    let user = users.filter(username.eq(find_username.as_str())).first(db)?;
    Ok(user)
}

pub fn update(db: &PgConnection, obj_in: &UserUpdate) -> Result<User> {
    let updated_user = diesel::update(obj_in).set(obj_in).get_result(db)?;
    Ok(updated_user)
}

pub fn delete(db: &PgConnection, obj_id: Uuid) -> Result<User> {
    use crate::diesel_schema::users::dsl::*;
    let deleted_user = diesel::delete(users.filter(id.eq(obj_id))).get_result(db)?;
    Ok(deleted_user)
}