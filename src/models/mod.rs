use rocket::{routes, Build};

pub mod meeting;
pub mod meeting_config;
pub mod team;
pub mod user;
pub mod user_time;

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    rocket
        .mount("/api/user", routes![user::login, user::signup])
        .mount("/api/team", routes![team::new, team::get])
}
