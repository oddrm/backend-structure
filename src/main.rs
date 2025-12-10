use std::time::Duration;

use crate::storage::storage_instance::{Event, StorageInstance};
use rocket::routes;
use routes::queries::{
    add_sequence, get_entries, get_metadata, get_sequences, remove_sequence, update_metadata,
    update_sequence, update_tags,
};
use std::path::PathBuf;
use tokio::sync::mpsc::Sender;
use tracing_subscriber::FmtSubscriber;

pub mod error;
pub mod plugins;
pub mod routes;
pub mod storage;

pub struct AppState {
    pub event_transmitter: Sender<Event>,
}

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
    // TODO: process events in new thread

    let event_transmitter = storage_instance.get_event_transmitter();

    // web server
    rocket::build()
        .mount(
            "/",
            routes![
                get_entries,
                get_sequences,
                get_metadata,
                update_metadata,
                update_tags,
                add_sequence,
                remove_sequence,
                update_sequence,
            ],
        )
        .manage(AppState { event_transmitter })
        .launch()
        .await
        .unwrap();
}
