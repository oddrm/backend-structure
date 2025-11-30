use std::{path::PathBuf, time::Duration};

use crate::{error::Error, storage::storage_instance::StorageInstance};
use tokio::sync::mpsc::{self, Receiver};

pub struct StorageEventProcessor<'a> {
    storage_instance: &'a mut StorageInstance,
}

pub enum Event {
    NewEntry(PathBuf),
    UpdateEntry(PathBuf),
    DeleteEntry(PathBuf),
}

impl StorageEventProcessor<'_> {
    pub fn new(storage_instance: &mut StorageInstance) -> Self {
        todo!()
    }

    pub fn process_event(&mut self, event: &Event) -> Result<(), Error> {
        todo!()
    }

    pub fn scan_initial(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn start_periodic_scan(&mut self, interval: &Duration) -> Result<(), Error> {
        todo!()
    }

    pub fn stop_scanning(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn get_event_receiver(&self) -> Receiver<Event> {
        todo!()
    }
}
