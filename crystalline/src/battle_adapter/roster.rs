use crate::battle::BattlePet;
use crate::pets::{Move, MoveEffect, PetSpecies};

use super::{AdapterError, mapper, move_semantics};

#[derive(Debug, Clone, Default)]
pub struct BattleRosterBuilder;

impl BattleRosterBuilder {
    pub fn build_pet(
        &self,
        species: &PetSpecies,
        selected_move_ids: &[String],
    ) -> Result<BattlePet, AdapterError> {
        let moves = if selected_move_ids.is_empty() {
            default_battle_moves(&species.learnset)
        } else {
            select_moves(&species.learnset, selected_move_ids)?
        };

        if moves.is_empty() {
            return Err(AdapterError::InvalidSelection(format!(
                "pokemon {} does not have any usable moves",
                species.name
            )));
        }

        mapper::map_species_to_battle_pet(
            species,
            moves.into_iter().map(move_semantics::map_move).collect(),
        )
    }

    pub fn build_pet_with_defaults(&self, species: &PetSpecies) -> Result<BattlePet, AdapterError> {
        self.build_pet(species, &[])
    }
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

fn select_moves(
    learnset: &[Move],
    selected_move_ids: &[String],
) -> Result<Vec<Move>, AdapterError> {
    if selected_move_ids.len() > 4 {
        return Err(AdapterError::InvalidSelection(format!(
            "pet must have between 1 and 4 moves, got {}",
            selected_move_ids.len()
        )));
    }

    selected_move_ids
        .iter()
        .map(|move_id| {
            learnset
                .iter()
                .find(|battle_move| battle_move.id == *move_id)
                .cloned()
                .ok_or_else(|| AdapterError::MissingMove(move_id.clone()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{BattleEffect, BattleTarget};
    use crate::pets::Stats;

    fn species() -> PetSpecies {
        PetSpecies {
            pokemon_id: 1,
            species_id: "pokemon-1-flare".to_string(),
            name: "Flare".to_string(),
            element: "Fire".to_string(),
            evo_stage: "Base".to_string(),
            ability: "Warm Up".to_string(),
            spirit_no: None,
            icon_path: None,
            stats: Stats {
                max_hp: 30,
                attack: 8,
                defense: 4,
                speed: 10,
                special_attack: 7,
                special_defense: 5,
            },
            learnset: vec![
                Move {
                    id: "status".to_string(),
                    name: "Status".to_string(),
                    element: "Fire".to_string(),
                    category: "Status".to_string(),
                    priority: 0,
                    energy_cost: 0,
                    description: String::new(),
                    effect: MoveEffect::Status,
                },
                Move {
                    id: "damage".to_string(),
                    name: "Damage".to_string(),
                    element: "Fire".to_string(),
                    category: "Special".to_string(),
                    priority: 0,
                    energy_cost: 0,
                    description: String::new(),
                    effect: MoveEffect::Damage { power: 20 },
                },
            ],
            evolutions: Vec::new(),
        }
    }

    #[test]
    fn defaults_to_damaging_moves_first() {
        let builder = BattleRosterBuilder;
        let pet = builder.build_pet_with_defaults(&species()).unwrap();

        assert_eq!(pet.moves.len(), 1);
        assert_eq!(pet.moves[0].id, "damage");
        assert_eq!(
            pet.moves[0].semantics.effects,
            vec![BattleEffect::DealDamage {
                power: 20,
                target: BattleTarget::OpponentActive
            }]
        );
    }

    #[test]
    fn validates_selected_moves() {
        let builder = BattleRosterBuilder;
        let error = builder
            .build_pet(&species(), &[String::from("missing")])
            .unwrap_err();

        assert!(matches!(error, AdapterError::MissingMove(_)));
    }
}
