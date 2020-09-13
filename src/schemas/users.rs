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

// #[derive(Serialize, Deserialize)]
// pub struct UserInDb {
//     pub id: Uuid,
//     pub username: String,
//     pub is_active: bool,
// }
//
// impl UserInDb {
//     pub fn to_model(&self) -> User {
//         User {
//             id: self.id,
//             username: self.username.clone(),
//             is_active: self.is_active,
//         }
//     }
//
//     pub fn from_model(user: &User) -> UserInDb {
//         UserInDb {
//             id: user.id,
//             is_active: user.is_active,
//             username: user.username.clone(),
//         }
//     }
// }