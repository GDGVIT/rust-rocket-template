use rocket::http::{ContentType, Status};
use rocket::local::Client;

use crate::models::users::User;
use crate::rocket;
use crate::schemas::users::{UserCreate, UserUpdate};

#[test]
fn test_health() {
    dotenv::dotenv().ok();
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/api").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("OK".into()));
}

#[test]
fn test_crud_user() {
    dotenv::dotenv().ok();
    let client = Client::new(rocket()).expect("valid rocket instance");

    // Create user
    let mut response = client
        .post("/api/users")
        .body(
            serde_json::to_string(&UserCreate {
                id: None,
                username: "user1".to_owned(),
                is_active: false,
            })
            .unwrap(),
        )
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let created_body: User =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    // Read user
    let mut response = client
        .get(format!("/api/users/{}", created_body.id))
        .header(ContentType::JSON)
        .dispatch();

    let fetched_body: User =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(fetched_body.id, created_body.id);
    assert_eq!(fetched_body.username, created_body.username);
    assert_eq!(fetched_body.is_active, created_body.is_active);

    // Update user
    let mut response = client
        .patch("/api/users")
        .body(
            serde_json::to_string(&UserUpdate {
                id: fetched_body.id,
                username: Some("user1_modified".to_owned()),
                is_active: Some(true),
            })
            .unwrap(),
        )
        .header(ContentType::JSON)
        .dispatch();

    let updated_body: User =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(updated_body.id, fetched_body.id);
    assert_eq!(updated_body.username, "user1_modified".to_string());
    assert_eq!(updated_body.is_active, true);

    // Delete user
    let mut response = client
        .delete(format!("/api/users/{}", updated_body.id))
        .header(ContentType::JSON)
        .dispatch();

    let deleted_body: User =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(deleted_body.id, updated_body.id);
    assert_eq!(deleted_body.username, updated_body.username);
    assert_eq!(deleted_body.is_active, updated_body.is_active);
}
