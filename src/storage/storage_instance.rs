use std::{path::PathBuf, time::Duration};

use crate::{
    error::{Error, StorageError},
    storage::{
        metadata::Metadata,
        sequence::{Sequence, SequenceID},
    },
};
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub type EntryID = u64;
pub type Map<K, V> = std::collections::HashMap<K, V>;

pub struct StorageInstance {}

pub enum Event {
    NewEntry(PathBuf),
    UpdateEntry(PathBuf),
    DeleteEntry(PathBuf),
    UpdateMetadata(PathBuf, Metadata),
    GetMetadata(PathBuf, oneshot::Sender<Option<Metadata>>),
    GetPath(EntryID, oneshot::Sender<Option<PathBuf>>),
    GetSequences(EntryID, oneshot::Sender<Map<SequenceID, Sequence>>),
    GetIterClone(oneshot::Sender<StorageInstance>),
}

impl StorageInstance {
    pub fn new(path: &PathBuf) -> Result<Self, StorageError> {
        todo!()
    }

    pub fn path(&self) -> &PathBuf {
        todo!()
    }

    pub fn close(self) -> Result<(), StorageError> {
        todo!()
    }

    pub fn get_metadata(&self, id: EntryID) -> Result<Option<Metadata>, StorageError> {
        todo!()
    }

    pub fn get_path(&self, id: EntryID) -> Result<Option<PathBuf>, StorageError> {
        todo!()
    }

    pub fn update_metadata_for_entry(
        &mut self,
        id: EntryID,
        metadata: &Metadata,
    ) -> Result<EntryID, StorageError> {
        todo!()
    }

    pub fn add_path(&mut self, path: &PathBuf) -> Result<EntryID, StorageError> {
        todo!()
    }

    pub fn remove_entry(&mut self, id: EntryID) -> Result<(), StorageError> {
        todo!()
    }

    pub fn get_sequences(&self, id: EntryID) -> Result<Map<SequenceID, Sequence>, StorageError> {
        todo!()
    }

    pub fn process_event(&mut self, event: &Event) -> Result<(), StorageError> {
        todo!()
    }

    pub async fn process_events(&mut self) -> Result<(), StorageError> {
        todo!()
    }

    pub fn scan_once(&mut self) -> Result<(), Error> {
        todo!()
    }

    // only sends events into queue, events still have to be processed somewhere else
    pub fn start_scanning(&mut self, interval: &Duration) -> Result<(), Error> {
        todo!()
    }

    pub fn stop_scanning(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn get_event_transmitter(&self) -> mpsc::Sender<Event> {
        todo!()
    }
}

impl Iterator for StorageInstance {
    type Item = Result<EntryID, StorageError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
