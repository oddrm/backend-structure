use std::path::PathBuf;

use crate::{error::StorageError, storage::metadata::Metadata};

pub type EntryID = u64;

pub struct StorageInstance {}

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

    pub fn insert_metadata_for_entry(
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
}

impl Iterator for StorageInstance {
    type Item = Result<EntryID, StorageError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
