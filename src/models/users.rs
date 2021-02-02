use uuid::Uuid;

use crate::diesel_schema::users;

#[table_name = "users"]
#[derive(AsChangeset, Queryable, Insertable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub is_active: bool,
}
