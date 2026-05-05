use std::fmt;

#[derive(Debug)]
pub enum PetSystemError {
    Io(std::io::Error),
    Sqlite(rusqlite::Error),
    MissingSpecies(String),
    InvalidPet(String),
}

impl fmt::Display for PetSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "{error}"),
            Self::Sqlite(error) => write!(f, "{error}"),
            Self::MissingSpecies(name) => write!(f, "missing species data for {name}"),
            Self::InvalidPet(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for PetSystemError {}

impl From<std::io::Error> for PetSystemError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<rusqlite::Error> for PetSystemError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}
