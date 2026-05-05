use std::path::PathBuf;

use super::{BestiaryError, Evolution, Move, PetRepository, PetSpecies};

pub trait PetCatalogService {
    fn species_by_name(&self, name: &str) -> Result<PetSpecies, BestiaryError>;
    fn legal_moves_for_species(&self, species_id: &str) -> Result<Vec<Move>, BestiaryError>;
}

#[derive(Debug, Clone, Copy)]
pub struct PetQueryService<'a, R> {
    repository: &'a R,
}

impl<'a, R> PetQueryService<'a, R> {
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }
}

impl<R> PetCatalogService for PetQueryService<'_, R>
where
    R: PetRepository,
{
    fn species_by_name(&self, name: &str) -> Result<PetSpecies, BestiaryError> {
        self.repository
            .find_species_by_name(name)?
            .ok_or_else(|| BestiaryError::MissingSpecies(name.to_string()))
    }

    fn legal_moves_for_species(&self, species_id: &str) -> Result<Vec<Move>, BestiaryError> {
        self.repository
            .get_species(species_id)?
            .map(|species| species.learnset)
            .ok_or_else(|| BestiaryError::MissingSpecies(species_id.to_string()))
    }
}

impl<R> PetQueryService<'_, R>
where
    R: PetRepository,
{
    pub fn species_by_element(&self, element: &str) -> Result<Vec<PetSpecies>, BestiaryError> {
        Ok(self
            .repository
            .list_species()?
            .into_iter()
            .filter(|species| species.element == element)
            .collect())
    }

    pub fn species_by_stage(&self, evo_stage: &str) -> Result<Vec<PetSpecies>, BestiaryError> {
        Ok(self
            .repository
            .list_species()?
            .into_iter()
            .filter(|species| species.evo_stage == evo_stage)
            .collect())
    }

    pub fn evolution_chain_for_species(
        &self,
        species_id: &str,
    ) -> Result<Vec<Evolution>, BestiaryError> {
        self.repository
            .get_species(species_id)?
            .map(|species| species.evolutions)
            .ok_or_else(|| BestiaryError::MissingSpecies(species_id.to_string()))
    }

    pub fn icon_path_for_species(
        &self,
        species_id: &str,
    ) -> Result<Option<PathBuf>, BestiaryError> {
        self.repository
            .get_species(species_id)?
            .map(|species| species.icon_path)
            .ok_or_else(|| BestiaryError::MissingSpecies(species_id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MoveEffect, PetCatalog, Stats};

    fn catalog() -> PetCatalog {
        let alpha = PetSpecies {
            pokemon_id: 1,
            species_id: "alpha".to_string(),
            name: "Alpha".to_string(),
            element: "Fire".to_string(),
            evo_stage: "Base".to_string(),
            ability: "Warm Up".to_string(),
            spirit_no: None,
            icon_path: None,
            stats: Stats {
                max_hp: 20,
                attack: 8,
                defense: 4,
                speed: 6,
                special_attack: 7,
                special_defense: 5,
            },
            learnset: vec![Move {
                id: "flare".to_string(),
                name: "Flare".to_string(),
                element: "Fire".to_string(),
                category: "Special".to_string(),
                priority: 0,
                energy_cost: 0,
                description: String::new(),
                effect: MoveEffect::Damage { power: 20 },
            }],
            evolutions: vec![Evolution {
                to_name: "Beta".to_string(),
                evo_level: Some(16),
                condition: None,
                chain_text: None,
            }],
        };
        let beta = PetSpecies {
            pokemon_id: 2,
            species_id: "beta".to_string(),
            name: "Beta".to_string(),
            element: "Water".to_string(),
            evo_stage: "Final".to_string(),
            ability: "Splash".to_string(),
            spirit_no: None,
            icon_path: Some(PathBuf::from("/tmp/beta.png")),
            stats: alpha.stats.clone(),
            learnset: alpha.learnset.clone(),
            evolutions: Vec::new(),
        };

        PetCatalog::new(vec![alpha, beta])
    }

    #[test]
    fn queries_species_by_filters() {
        let catalog = catalog();
        let query = catalog.query();

        assert_eq!(query.species_by_element("Fire").unwrap().len(), 1);
        assert_eq!(query.species_by_stage("Final").unwrap().len(), 1);
        assert_eq!(
            query.evolution_chain_for_species("alpha").unwrap()[0].to_name,
            "Beta"
        );
        assert_eq!(
            query.icon_path_for_species("beta").unwrap(),
            Some(PathBuf::from("/tmp/beta.png"))
        );
    }
}
