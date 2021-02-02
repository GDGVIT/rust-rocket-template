use uuid::Uuid;

use crate::diesel_schema::users;

#[derive(Serialize, Deserialize)]
pub struct UserCreate {
    pub id: Option<Uuid>,
    pub username: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, AsChangeset, Identifiable)]
#[table_name = "users"]
pub struct UserUpdate {
    pub id: Uuid,
    pub username: Option<String>,
    pub is_active: Option<bool>,
}
