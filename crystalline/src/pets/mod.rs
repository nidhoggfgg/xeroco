mod catalog;
mod error;
mod model;
mod nrc;
mod query;
mod repository;

use std::path::PathBuf;

pub use catalog::PetCatalog;
pub use error::PetSystemError;
pub use model::{Evolution, Move, MoveEffect, PetSpecies, Stats};
pub use nrc::NrcRepository;
pub use query::{PetCatalogService, PetQueryService};
pub use repository::PetRepository;

pub fn bundled_nrc_bundle_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("data")
        .join("nrc_pokemon_data_bundle")
}
