pub enum Error {
    StorageError(StorageError),
    ParsingError(String),
    PollingError(notify::Error),
    CustomError(String),
}

pub enum StorageError {
    IoError(std::io::Error),
    NotFound,
    AlreadyExists,
    DecodingError(String),
    CustomError(String),
}
