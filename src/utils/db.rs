use mongodb::{options::ClientOptions, Client};

pub async fn check_db_working() {
    println!("[DB] ~ Checking database status...");
    let mut tries: u8 = 1;
    let mut connected: bool = false;
    let mut opts = ClientOptions::parse(std::env::var("CONN_STR").unwrap())
        .await
        .unwrap();
    opts.connect_timeout = Some(std::time::Duration::from_secs(5));
    let client = Client::with_options(opts).unwrap();
    let db = client.database("smt-backend");

    while !connected {
        let connection_result = db.list_collection_names(None).await;
        if let Err(_) = connection_result {
            eprintln!("[DB] ~ Connection failed after {} tries", tries);
            tries += 1;
        } else {
            connected = true;
            println!("[DB] ~ Working!");
        }
    }
}
