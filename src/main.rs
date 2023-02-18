pub mod models;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    smt_backend::run_api().await?;
    Ok(())
}
