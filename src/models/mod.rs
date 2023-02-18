use rocket::Build;

pub mod meeting;
pub mod meeting_config;
pub mod team;
pub mod user;
pub mod user_time;

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    rocket.mount("/api/user", rocket::routes![user::login, user::signup])
}
