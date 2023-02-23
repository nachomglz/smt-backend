use dotenv::dotenv;

pub mod config;
pub mod models;
pub mod utils;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    smt_backend::run_api().await?;
    Ok(())
}
