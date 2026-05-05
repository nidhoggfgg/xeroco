use super::{BestiaryError, PetSpecies};

pub trait PetRepository {
    fn list_species(&self) -> Result<Vec<PetSpecies>, BestiaryError>;
    fn get_species(&self, species_id: &str) -> Result<Option<PetSpecies>, BestiaryError>;
    fn find_species_by_name(&self, name: &str) -> Result<Option<PetSpecies>, BestiaryError>;
}
