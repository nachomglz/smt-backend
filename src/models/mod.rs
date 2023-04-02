use rocket::{routes, Build};

pub mod meeting;
pub mod meeting_config;
pub mod team;
pub mod user;
pub mod user_time;

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    rocket
        .mount(
            "/api/user",
            routes![
                user::login,
                user::signup,
                user::get,
                user::update,
                user::delete
            ],
        )
        .mount(
            "/api/team",
            routes![
                team::create,
                team::get,
                team::update,
                team::get_users,
                team::all
            ],
        )
        .mount("/api/meeting", routes![meeting::create, meeting::get])
        .mount(
            "/api/meeting_config",
            routes![
                meeting_config::create,
                meeting_config::get,
                meeting_config::update,
                meeting_config::delete,
                meeting_config::all
            ],
        )
        .mount(
            "/api/user_time",
            routes![
                user_time::create,
                user_time::get,
                user_time::update,
                user_time::delete
            ],
        )
}
