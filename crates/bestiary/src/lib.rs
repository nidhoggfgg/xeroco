pub mod bundle;
mod catalog;
mod error;
mod model;
mod nrc;
mod query;
mod repository;

pub use bundle::bundled_nrc_bundle_dir;
pub use catalog::PetCatalog;
pub use error::BestiaryError;
pub use model::{Evolution, Move, MoveEffect, PetSpecies, Stats};
pub use nrc::NrcRepository;
pub use query::{PetCatalogService, PetQueryService};
pub use repository::PetRepository;
