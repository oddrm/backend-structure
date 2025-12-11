#![allow(unused)]

use std::{path::PathBuf, time::Duration};

use crate::{
    error::{Error, StorageError},
    routes::queries::Entry,
    storage::{
        metadata::Metadata,
        sequence::{Sequence, SequenceID},
    },
};
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub type EntryID = u64;
pub type Map<K, V> = std::collections::HashMap<K, V>;

#[derive(Clone)]
pub struct StorageInstance {}

pub enum Event {
    NewEntry(PathBuf),
    UpdateEntry(PathBuf),
    DeleteEntry(PathBuf),
    UpdateMetadata(PathBuf, Metadata),
    GetMetadata(PathBuf, oneshot::Sender<Option<Metadata>>),
    GetPath(EntryID, oneshot::Sender<Option<PathBuf>>),
    GetSequences(EntryID, oneshot::Sender<Map<SequenceID, Sequence>>),
    GetIterClone(oneshot::Sender<Iter>),
}
// TODO think about importing yaml metadata files
// TODO parallel read access, use multiversion concurrency control, as much as possible sled inbuilt functionality
// TODO make reads and writes always transactional
// TODO rename to records?
// find name for non-file data
// TODO differentiate between file and non-file read/write
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

    pub fn get_db_path(&self, id: EntryID) -> Result<Option<PathBuf>, StorageError> {
        todo!()
    }

    pub fn get_metadata(&self, id: EntryID) -> Result<Option<Metadata>, StorageError> {
        todo!()
    }

    pub fn update_metadata(
        &self,
        id: EntryID,
        metadata: &Metadata,
    ) -> Result<EntryID, StorageError> {
        todo!()
    }

    pub fn get_entries(
        &self,
        search_string: Option<String>,
        sort_by: Option<String>,
        ascending: Option<bool>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<(EntryID, Metadata)>, StorageError> {
        todo!()
    }

    pub fn get_entry(&self, id: EntryID) -> Result<Option<Entry>, StorageError> {
        todo!()
    }

    pub fn get_entry_by_path(&self, path: &str) -> Result<Option<Entry>, StorageError> {
        todo!()
    }

    pub fn get_sequences(&self, id: EntryID) -> Result<Map<SequenceID, Sequence>, StorageError> {
        todo!()
    }

    pub fn add_sequence(
        &self,
        entry_id: EntryID,
        sequence: Sequence,
    ) -> Result<SequenceID, StorageError> {
        todo!()
    }
    pub fn update_sequence(
        &self,
        entry_id: EntryID,
        sequence_id: SequenceID,
        sequence: Sequence,
    ) -> Result<(), StorageError> {
        todo!()
    }

    pub fn remove_sequence(
        &self,
        entry_id: EntryID,
        sequence_id: SequenceID,
    ) -> Result<(), StorageError> {
        todo!()
    }

    pub fn add_tag(&self, id: EntryID, tag: String) -> Result<(), StorageError> {
        todo!()
    }

    pub fn remove_tag(&self, id: EntryID, tag: String) -> Result<(), StorageError> {
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

    // only sends fs events into queue, events still have to be processed somewhere else
    pub fn start_scanning(&mut self, interval: &Duration) -> Result<(), Error> {
        todo!()
    }

    pub fn stop_scanning(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn get_event_transmitter(&self) -> mpsc::Sender<Event> {
        todo!()
    }

    pub fn iter(&self) -> Iter {
        todo!()
    }
}

#[derive(Clone)]
pub struct Iter {}

impl Iterator for Iter {
    type Item = EntryID;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
