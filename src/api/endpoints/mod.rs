use rocket::Rocket;
use rocket_cors::CorsOptions;

pub mod users;

pub fn fuel(rocket: Rocket) -> Rocket {
    let mut rocket = rocket;
    let cors = CorsOptions::default().to_cors().unwrap();

    rocket = users::fuel(rocket);
    rocket.attach(cors)
}
