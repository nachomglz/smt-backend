mod models;

pub async fn run_api() -> Result<(), rocket::Error> {
    let mut rocket = rocket::build();
    rocket = models::mount(rocket);
    rocket.launch().await?;
    Ok(())
}
