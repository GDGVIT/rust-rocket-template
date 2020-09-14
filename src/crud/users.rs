use anyhow::Result;
use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::users::User;
use crate::diesel_schema::users;
use crate::schemas::users::UserCreate;

pub fn create(db: &PgConnection, user: UserCreate) -> Result<User> {
    use crate::diesel_schema::users::dsl::*;
    let new_user = User {
        id: Uuid::new_v4(),
        username: user.username,
        is_active: user.is_active,
    };
    insert_into(users)
        .values(&new_user)
        .on_conflict_do_nothing()
        .execute(db)?;
    Ok(find(db, new_user.id)?)
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