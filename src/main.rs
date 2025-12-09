use std::time::Duration;

use crate::storage::storage_instance::StorageInstance;
use rocket::routes;
use std::path::PathBuf;
use tracing_subscriber::FmtSubscriber;
pub mod error;
pub mod routes;
pub mod storage;

/// Main entry point.
/// Launch web server, start db threads etc.
#[rocket::main]
async fn main() {
    // logging
    let log_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(log_subscriber).unwrap();
    // db
    let mut storage_instance = StorageInstance::new(&PathBuf::from("storage_path")).unwrap();
    storage_instance
        .start_scanning(&Duration::from_secs(2))
        .unwrap();
    // todo process events in thread
    // storage_instance.process_events();

    // web server
    rocket::build()
        .mount("/", routes![])
        .launch()
        .await
        .unwrap();
}
