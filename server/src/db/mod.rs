pub mod dbuser;
use std::env;

pub use dbuser::*;
pub mod dbmessage;
pub use dbmessage::*;
pub mod dbvisit;
pub use dbvisit::*;
use mongodb::{options::ClientOptions, Client, Database};
pub struct DB {
    pub client: Database,
}

impl DB {
    pub async fn new() -> Self {
        let env_conn_string = std::env::var("MONGODB_CONN_STRING").expect("mongo db connection string is not set");
        println!("Connection string is {}", env_conn_string);
        let client_options = ClientOptions::parse(env_conn_string)
            .await
            .expect("Error occured with mongodb client options");
        let client = Client::with_options(client_options).expect("Can't connect to mongodb client");
        Self {
            client: client.database("LFC"),
        }
    }
}
