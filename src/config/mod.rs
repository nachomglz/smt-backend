use async_trait::async_trait;
use deadpool::managed;
use mongodb::options::ClientOptions;

pub struct PoolManager {}

impl PoolManager {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl managed::Manager for PoolManager {
    type Type = mongodb::Client;
    type Error = mongodb::error::Error;

    async fn create(&self) -> Result<mongodb::Client, mongodb::error::Error> {
        let CONN_STR = std::env::var("CONN_STR").unwrap();
        let mut client_options = ClientOptions::parse(CONN_STR).await?;
        client_options.app_name = Some("smt-backend".to_owned());
        client_options.default_database = Some("smt-backend".to_owned());
        Ok(mongodb::Client::with_options(client_options)?)
    }

    async fn recycle(
        &self,
        _: &mut mongodb::Client,
    ) -> managed::RecycleResult<mongodb::error::Error> {
        Ok(())
    }
}

pub type Pool = managed::Pool<PoolManager>;
