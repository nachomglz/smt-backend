use dotenv::dotenv;

pub mod config;
pub mod models;
pub mod utils;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    std::env::var("MONGODB_ROOT_USERNAME").expect("[DB] ~ Username variable hasn't been provided");
    std::env::var("MONGODB_ROOT_PASSWORD").expect("[DB] ~ Password variable hasn't been provided");

    smt_backend::run_api().await?;
    Ok(())
}
