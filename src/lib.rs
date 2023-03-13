mod config;
mod models;
mod utils;

#[allow(unused)]
pub async fn run_api() -> Result<(), rocket::Error> {
    let mut rocket = rocket::build();
    rocket = models::mount(rocket);

    utils::db::check_db_working().await;
    let pool_manager = config::PoolManager::new();
    let pool = config::Pool::builder(pool_manager)
        .max_size(16)
        .build()
        .unwrap();

    rocket.manage(pool).launch().await?;
    Ok(())
}
