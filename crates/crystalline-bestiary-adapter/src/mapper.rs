use bestiary::{PetSpecies, Stats};
use crystalline::battle::{BattleMove, BattlePet, BattleStats};

use super::AdapterError;

pub(crate) fn map_species_to_battle_pet(
    species: &PetSpecies,
    moves: Vec<BattleMove>,
) -> Result<BattlePet, AdapterError> {
    BattlePet::new(
        species.species_id.clone(),
        species.species_id.clone(),
        species.name.clone(),
        species.element.clone(),
        map_stats(&species.stats),
        moves,
    )
    .map_err(AdapterError::from)
}

fn map_stats(value: &Stats) -> BattleStats {
    BattleStats {
        max_hp: value.max_hp,
        attack: value.attack,
        defense: value.defense,
        speed: value.speed,
        special_attack: value.special_attack,
        special_defense: value.special_defense,
    }
}
