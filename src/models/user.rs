pub struct User {
    name: String,
}

#[rocket::post("/login")]
pub fn login() -> &'static str {
    "loggin in..."
}

#[rocket::post("/signup")]
pub fn signup() -> &'static str {
    "signin up..."
}
