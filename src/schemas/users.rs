use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserCreate {
    pub id: Option<Uuid>,
    pub username: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub is_active: Option<bool>,
}