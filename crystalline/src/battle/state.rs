use crate::battle::BattleError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleStats {
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special_attack: i32,
    pub special_defense: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattleTarget {
    SelfActive,
    OpponentActive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattleEffect {
    DealDamage { power: i32, target: BattleTarget },
    StatusPlaceholder,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleMoveSemantics {
    pub effects: Vec<BattleEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleMove {
    pub id: String,
    pub name: String,
    pub priority: i8,
    pub semantics: BattleMoveSemantics,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattlePet {
    pub id: String,
    pub species_id: String,
    pub name: String,
    pub element: String,
    pub stats: BattleStats,
    pub current_hp: i32,
    pub moves: Vec<BattleMove>,
}

impl BattlePet {
    pub fn new(
        id: impl Into<String>,
        species_id: impl Into<String>,
        name: impl Into<String>,
        element: impl Into<String>,
        stats: BattleStats,
        moves: Vec<BattleMove>,
    ) -> Result<Self, BattleError> {
        if moves.is_empty() || moves.len() > 4 {
            return Err(BattleError::InvalidBattlePet(format!(
                "pet must have between 1 and 4 moves, got {}",
                moves.len()
            )));
        }

        Ok(Self {
            id: id.into(),
            species_id: species_id.into(),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub pets: Vec<BattlePet>,
    pub active: usize,
}

impl Team {
    pub fn new(pets: Vec<BattlePet>) -> Result<Self, BattleError> {
        if pets.is_empty() || pets.len() > 6 {
            return Err(BattleError::InvalidTeam(format!(
                "team must have between 1 and 6 pets, got {}",
                pets.len()
            )));
        }

        Ok(Self { pets, active: 0 })
    }

    pub fn active_pet(&self) -> &BattlePet {
        &self.pets[self.active]
    }

    pub fn active_pet_mut(&mut self) -> &mut BattlePet {
        &mut self.pets[self.active]
    }

    pub fn has_available_pet(&self) -> bool {
        self.pets.iter().any(|pet| !pet.is_fainted())
    }

    pub fn first_available_bench(&self) -> Option<usize> {
        self.pets
            .iter()
            .enumerate()
            .find(|(index, pet)| *index != self.active && !pet.is_fainted())
            .map(|(index, _)| index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Side {
    pub name: String,
    pub team: Team,
}

impl Side {
    pub fn new(name: impl Into<String>, team: Team) -> Self {
        Self {
            name: name.into(),
            team,
        }
    }
}
