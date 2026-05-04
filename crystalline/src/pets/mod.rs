mod model;
mod nrc;

use std::fmt;
use std::path::PathBuf;

pub use model::{Move, MoveEffect, Stats};
pub use nrc::PetCatalog;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evolution {
    pub to_name: String,
    pub evo_level: Option<i32>,
    pub condition: Option<String>,
    pub chain_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetSpecies {
    pub pokemon_id: i64,
    pub species_id: String,
    pub name: String,
    pub element: String,
    pub evo_stage: String,
    pub ability: String,
    pub spirit_no: Option<String>,
    pub icon_path: Option<PathBuf>,
    pub stats: Stats,
    pub learnset: Vec<Move>,
    pub evolutions: Vec<Evolution>,
}

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

pub fn bundled_nrc_bundle_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("data")
        .join("nrc_pokemon_data_bundle")
}
