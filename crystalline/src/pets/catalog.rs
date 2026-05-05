use std::collections::HashMap;
use std::path::Path;

use rusqlite::Connection;

use super::{
    NrcRepository, PetCatalogService, PetQueryService, PetRepository, PetSpecies, PetSystemError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetCatalog {
    species: Vec<PetSpecies>,
    species_by_id: HashMap<String, usize>,
    species_by_name: HashMap<String, usize>,
}

impl PetCatalog {
    pub(crate) fn new(species: Vec<PetSpecies>) -> Self {
        let species_by_id = species
            .iter()
            .enumerate()
            .map(|(index, species)| (species.species_id.clone(), index))
            .collect();
        let species_by_name = species
            .iter()
            .enumerate()
            .map(|(index, species)| (species.name.clone(), index))
            .collect();

        Self {
            species,
            species_by_id,
            species_by_name,
        }
    }

    pub fn from_nrc_bundle(bundle_root: impl AsRef<Path>) -> Result<Self, PetSystemError> {
        NrcRepository::load_catalog_from_bundle(bundle_root)
    }

    pub fn from_connection(
        connection: &Connection,
        icons_dir: Option<&Path>,
    ) -> Result<Self, PetSystemError> {
        NrcRepository::load_catalog_from_connection(connection, icons_dir)
    }

    pub fn species(&self) -> &[PetSpecies] {
        &self.species
    }

    pub fn species_by_id(&self, species_id: &str) -> Option<&PetSpecies> {
        self.species_by_id
            .get(species_id)
            .and_then(|index| self.species.get(*index))
    }

    pub fn species_by_name(&self, name: &str) -> Option<&PetSpecies> {
        self.species_by_name
            .get(name)
            .and_then(|index| self.species.get(*index))
    }

    pub fn query(&self) -> PetQueryService<'_, Self> {
        PetQueryService::new(self)
    }

    pub fn service(&self) -> impl PetCatalogService + '_ {
        self.query()
    }
}

impl PetRepository for PetCatalog {
    fn list_species(&self) -> Result<Vec<PetSpecies>, PetSystemError> {
        Ok(self.species.clone())
    }

    fn get_species(&self, species_id: &str) -> Result<Option<PetSpecies>, PetSystemError> {
        Ok(self.species_by_id(species_id).cloned())
    }

    fn find_species_by_name(&self, name: &str) -> Result<Option<PetSpecies>, PetSystemError> {
        Ok(self.species_by_name(name).cloned())
    }
}
