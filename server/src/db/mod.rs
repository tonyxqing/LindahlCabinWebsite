pub mod dbuser;
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
        let client_options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .expect("Error occured with mongodb client options");
        let client = Client::with_options(client_options).expect("Can't connect to mongodb client");
        Self {
            client: client.database("LFC"),
        }
    }
}
