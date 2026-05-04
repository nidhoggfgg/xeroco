use crate::battle::{BattleMove, BattleMoveEffect, BattlePet, BattleStats};
use crate::pets::{Move, MoveEffect, PetSpecies, PetSystemError};

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

        BattlePet::new(
            species.species_id.clone(),
            species.species_id.clone(),
            species.name.clone(),
            species.element.clone(),
            BattleStats::from(&species.stats),
            moves.into_iter().map(BattleMove::from).collect(),
        )
        .map_err(AdapterError::Battle)
    }

    pub fn build_pet_with_defaults(&self, species: &PetSpecies) -> Result<BattlePet, AdapterError> {
        self.build_pet(species, &[])
    }
}

#[derive(Debug)]
pub enum AdapterError {
    MissingMove(String),
    InvalidSelection(String),
    Battle(crate::battle::BattleError),
    PetSystem(PetSystemError),
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingMove(move_id) => write!(f, "missing move definition for {move_id}"),
            Self::InvalidSelection(message) => write!(f, "{message}"),
            Self::Battle(error) => write!(f, "{error}"),
            Self::PetSystem(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for AdapterError {}

impl From<PetSystemError> for AdapterError {
    fn from(value: PetSystemError) -> Self {
        Self::PetSystem(value)
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

impl From<&crate::pets::Stats> for BattleStats {
    fn from(value: &crate::pets::Stats) -> Self {
        Self {
            max_hp: value.max_hp,
            attack: value.attack,
            defense: value.defense,
            speed: value.speed,
            special_attack: value.special_attack,
            special_defense: value.special_defense,
        }
    }
}

impl From<Move> for BattleMove {
    fn from(value: Move) -> Self {
        Self {
            id: value.id,
            name: value.name,
            priority: value.priority,
            effect: BattleMoveEffect::from(value.effect),
        }
    }
}

impl From<MoveEffect> for BattleMoveEffect {
    fn from(value: MoveEffect) -> Self {
        match value {
            MoveEffect::Damage { power } => Self::Damage { power },
            MoveEffect::Status => Self::Status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pets::{MoveEffect, Stats};

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
