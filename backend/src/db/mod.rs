use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mongodb::{Database, Client, options::{ClientOptions}};
use std::env;

lazy_static! {
    pub static ref CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
        return get_client_handle().await;
    });
}

async fn get_client_handle() -> Client {
    let mongo_url = env::var("MONGO_URI").expect("MONGO_URI is not defined")
    let mongo_db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not defined");
}

pub async fn get_db_handle() -> Database {
    let test_mode = env::var("TEST_MODE");
    return match test_mode {
        Ok(_) => get_client_handle().await.default_database().expect(""),
        Err(_) => Client.get().await()
    }
}

pub mod address_db;

