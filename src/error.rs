pub enum Error {
    StorageError(StorageError),
    ParsingError(String),
    CustomError(String),
}

pub enum StorageError {
    IoError(std::io::Error),
    NotFound,
    AlreadyExists,
    CustomError(String),
}
