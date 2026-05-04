mod model;
mod nrc;

use std::fmt;
use std::path::PathBuf;

pub use model::{Move, MoveEffect, Pet, Stats};
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

impl PetSpecies {
    pub fn instantiate(&self) -> Result<Pet, PetSystemError> {
        let moves = default_battle_moves(&self.learnset);
        if moves.is_empty() {
            return Err(PetSystemError::InvalidPet(format!(
                "pokemon {} does not have any usable moves",
                self.name
            )));
        }

        Pet::new(
            self.species_id.clone(),
            self.name.clone(),
            self.element.clone(),
            self.stats.clone(),
            moves,
        )
    }
}

impl Pet {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        element: impl Into<String>,
        stats: Stats,
        moves: Vec<Move>,
    ) -> Result<Self, PetSystemError> {
        if moves.is_empty() || moves.len() > 4 {
            return Err(PetSystemError::InvalidPet(format!(
                "pet must have between 1 and 4 moves, got {}",
                moves.len()
            )));
        }

        Ok(Self {
            id: id.into(),
            name: name.into(),
            element: element.into(),
            current_hp: stats.max_hp,
            stats,
            moves,
        })
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp <= 0
    }
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

fn default_battle_moves(learnset: &[Move]) -> Vec<Move> {
    let mut damaging: Vec<Move> = learnset
        .iter()
        .filter(|battle_move| matches!(battle_move.effect, MoveEffect::Damage { .. }))
        .take(4)
        .cloned()
        .collect();

    if damaging.is_empty() {
        damaging = learnset.iter().take(4).cloned().collect();
    }

    damaging
}
