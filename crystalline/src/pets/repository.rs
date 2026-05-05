use super::{PetSpecies, PetSystemError};

pub trait PetRepository {
    fn list_species(&self) -> Result<Vec<PetSpecies>, PetSystemError>;
    fn get_species(&self, species_id: &str) -> Result<Option<PetSpecies>, PetSystemError>;
    fn find_species_by_name(&self, name: &str) -> Result<Option<PetSpecies>, PetSystemError>;
}
