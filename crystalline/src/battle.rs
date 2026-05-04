use std::cmp::Reverse;

use crate::pets::{MoveEffect, Pet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub pets: Vec<Pet>,
    pub active: usize,
}

impl Team {
    pub fn new(pets: Vec<Pet>) -> Result<Self, BattleError> {
        if pets.is_empty() || pets.len() > 6 {
            return Err(BattleError::InvalidTeam(format!(
                "team must have between 1 and 6 pets, got {}",
                pets.len()
            )));
        }

        Ok(Self { pets, active: 0 })
    }

    pub fn active_pet(&self) -> &Pet {
        &self.pets[self.active]
    }

    pub fn active_pet_mut(&mut self) -> &mut Pet {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    UseMove { move_index: usize },
    Switch { target_index: usize },
    Pass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnEvent {
    TurnStarted {
        turn: u32,
    },
    Switched {
        side: usize,
        pet_name: String,
    },
    MoveUsed {
        side: usize,
        pet_name: String,
        move_name: String,
    },
    DamageDealt {
        source_side: usize,
        target_side: usize,
        amount: i32,
        remaining_hp: i32,
    },
    Fainted {
        side: usize,
        pet_name: String,
    },
    AutoSwitched {
        side: usize,
        pet_name: String,
    },
    TurnSkipped {
        side: usize,
        reason: String,
    },
    BattleEnded {
        winner: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnOutcome {
    pub events: Vec<TurnEvent>,
    pub winner: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattleError {
    InvalidTeam(String),
    InvalidAction(String),
    BattleAlreadyFinished,
}

impl std::fmt::Display for BattleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTeam(message) => write!(f, "{message}"),
            Self::InvalidAction(message) => write!(f, "{message}"),
            Self::BattleAlreadyFinished => write!(f, "battle already finished"),
        }
    }
}

impl std::error::Error for BattleError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleState {
    pub sides: [Side; 2],
    pub turn: u32,
    pub winner: Option<usize>,
}

#[derive(Debug, Clone)]
struct ResolvedAction {
    side: usize,
    priority: i8,
    speed: i32,
    action: Action,
}

impl BattleState {
    pub fn new(left: Side, right: Side) -> Self {
        Self {
            sides: [left, right],
            turn: 0,
            winner: None,
        }
    }

    pub fn resolve_turn(&mut self, actions: [Action; 2]) -> Result<TurnOutcome, BattleError> {
        if self.winner.is_some() {
            return Err(BattleError::BattleAlreadyFinished);
        }

        self.turn += 1;
        let mut events = vec![TurnEvent::TurnStarted { turn: self.turn }];
        let mut ordered_actions = self.resolve_action_order(actions)?;

        ordered_actions
            .sort_by_key(|action| (Reverse(action.priority), Reverse(action.speed), action.side));

        for resolved in ordered_actions {
            if self.winner.is_some() {
                break;
            }

            if self.sides[resolved.side].team.active_pet().is_fainted() {
                events.push(TurnEvent::TurnSkipped {
                    side: resolved.side,
                    reason: "active pet has fainted".to_string(),
                });
                continue;
            }

            self.apply_action(resolved.side, resolved.action, &mut events)?;
            self.check_auto_switch(&mut events);
            self.refresh_winner(&mut events);
        }

        Ok(TurnOutcome {
            events,
            winner: self.winner,
        })
    }

    fn resolve_action_order(
        &self,
        actions: [Action; 2],
    ) -> Result<Vec<ResolvedAction>, BattleError> {
        let mut resolved = Vec::with_capacity(2);

        for (side, action) in actions.into_iter().enumerate() {
            self.validate_action(side, &action)?;
            let priority = self.action_priority(side, &action)?;
            let speed = self.sides[side].team.active_pet().stats.speed;
            resolved.push(ResolvedAction {
                side,
                priority,
                speed,
                action,
            });
        }

        Ok(resolved)
    }

    fn validate_action(&self, side: usize, action: &Action) -> Result<(), BattleError> {
        let team = &self.sides[side].team;
        let active = team.active_pet();

        match action {
            Action::UseMove { move_index } => {
                if active.is_fainted() {
                    return Err(BattleError::InvalidAction(format!(
                        "side {side} cannot act with a fainted pet"
                    )));
                }
                if *move_index >= active.moves.len() {
                    return Err(BattleError::InvalidAction(format!(
                        "side {side} selected invalid move index {move_index}"
                    )));
                }
            }
            Action::Switch { target_index } => {
                if *target_index >= team.pets.len() {
                    return Err(BattleError::InvalidAction(format!(
                        "side {side} selected invalid switch target {target_index}"
                    )));
                }
                if *target_index == team.active {
                    return Err(BattleError::InvalidAction(format!(
                        "side {side} cannot switch to the active pet"
                    )));
                }
                if team.pets[*target_index].is_fainted() {
                    return Err(BattleError::InvalidAction(format!(
                        "side {side} cannot switch to a fainted pet"
                    )));
                }
            }
            Action::Pass => {}
        }

        Ok(())
    }

    fn action_priority(&self, side: usize, action: &Action) -> Result<i8, BattleError> {
        let priority = match action {
            Action::UseMove { move_index } => {
                self.sides[side].team.active_pet().moves[*move_index].priority
            }
            Action::Switch { .. } => 6,
            Action::Pass => i8::MIN,
        };
        Ok(priority)
    }

    fn apply_action(
        &mut self,
        side: usize,
        action: Action,
        events: &mut Vec<TurnEvent>,
    ) -> Result<(), BattleError> {
        match action {
            Action::UseMove { move_index } => self.apply_move(side, move_index, events),
            Action::Switch { target_index } => {
                self.switch_active(side, target_index, events);
                Ok(())
            }
            Action::Pass => {
                events.push(TurnEvent::TurnSkipped {
                    side,
                    reason: "side chose to pass".to_string(),
                });
                Ok(())
            }
        }
    }

    fn apply_move(
        &mut self,
        source_side: usize,
        move_index: usize,
        events: &mut Vec<TurnEvent>,
    ) -> Result<(), BattleError> {
        let target_side = 1 - source_side;
        let source_pet = self.sides[source_side].team.active_pet().clone();
        let target_pet = self.sides[target_side].team.active_pet().clone();
        let chosen_move = source_pet.moves[move_index].clone();

        events.push(TurnEvent::MoveUsed {
            side: source_side,
            pet_name: source_pet.name.clone(),
            move_name: chosen_move.name.clone(),
        });

        match chosen_move.effect {
            MoveEffect::Damage { power } => {
                let damage = (power + source_pet.stats.attack - target_pet.stats.defense).max(1);
                let defender = self.sides[target_side].team.active_pet_mut();
                defender.current_hp = (defender.current_hp - damage).max(0);

                events.push(TurnEvent::DamageDealt {
                    source_side,
                    target_side,
                    amount: damage,
                    remaining_hp: defender.current_hp,
                });

                if defender.is_fainted() {
                    events.push(TurnEvent::Fainted {
                        side: target_side,
                        pet_name: defender.name.clone(),
                    });
                }
            }
            MoveEffect::Status => {}
        }

        Ok(())
    }

    fn switch_active(&mut self, side: usize, target_index: usize, events: &mut Vec<TurnEvent>) {
        self.sides[side].team.active = target_index;
        let pet_name = self.sides[side].team.active_pet().name.clone();
        events.push(TurnEvent::Switched { side, pet_name });
    }

    fn check_auto_switch(&mut self, events: &mut Vec<TurnEvent>) {
        for side in 0..2 {
            let team = &mut self.sides[side].team;
            if !team.active_pet().is_fainted() {
                continue;
            }

            if let Some(next_index) = team.first_available_bench() {
                team.active = next_index;
                events.push(TurnEvent::AutoSwitched {
                    side,
                    pet_name: team.active_pet().name.clone(),
                });
            }
        }
    }

    fn refresh_winner(&mut self, events: &mut Vec<TurnEvent>) {
        let left_alive = self.sides[0].team.has_available_pet();
        let right_alive = self.sides[1].team.has_available_pet();

        self.winner = match (left_alive, right_alive) {
            (true, false) => Some(0),
            (false, true) => Some(1),
            (false, false) => None,
            (true, true) => None,
        };

        if let Some(winner) = self.winner {
            if !events
                .iter()
                .any(|event| matches!(event, TurnEvent::BattleEnded { .. }))
            {
                events.push(TurnEvent::BattleEnded { winner });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pets::{Move, Stats};

    fn strike() -> Move {
        Move {
            id: "strike".to_string(),
            name: "Strike".to_string(),
            element: "Neutral".to_string(),
            category: "Physical".to_string(),
            priority: 0,
            energy_cost: 0,
            description: String::new(),
            effect: MoveEffect::Damage { power: 10 },
        }
    }

    fn quick_strike() -> Move {
        Move {
            id: "quick_strike".to_string(),
            name: "Quick Strike".to_string(),
            element: "Neutral".to_string(),
            category: "Physical".to_string(),
            priority: 1,
            energy_cost: 0,
            description: String::new(),
            effect: MoveEffect::Damage { power: 6 },
        }
    }

    fn pet(name: &str, speed: i32) -> Pet {
        Pet::new(
            name.to_lowercase(),
            name,
            "Neutral",
            Stats {
                max_hp: 30,
                attack: 8,
                defense: 4,
                speed,
                special_attack: 7,
                special_defense: 5,
            },
            vec![strike(), quick_strike()],
        )
        .expect("battle test pet should be valid")
    }

    #[test]
    fn higher_priority_moves_first() {
        let left = Side::new("left", Team::new(vec![pet("Alpha", 5)]).unwrap());
        let right = Side::new("right", Team::new(vec![pet("Beta", 99)]).unwrap());
        let mut battle = BattleState::new(left, right);

        let outcome = battle
            .resolve_turn([
                Action::UseMove { move_index: 1 },
                Action::UseMove { move_index: 0 },
            ])
            .unwrap();

        assert!(matches!(
            outcome.events[1],
            TurnEvent::MoveUsed { side: 0, .. }
        ));
    }

    #[test]
    fn speed_breaks_priority_ties() {
        let left = Side::new("left", Team::new(vec![pet("Alpha", 50)]).unwrap());
        let right = Side::new("right", Team::new(vec![pet("Beta", 10)]).unwrap());
        let mut battle = BattleState::new(left, right);

        let outcome = battle
            .resolve_turn([
                Action::UseMove { move_index: 0 },
                Action::UseMove { move_index: 0 },
            ])
            .unwrap();

        assert!(matches!(
            outcome.events[1],
            TurnEvent::MoveUsed { side: 0, .. }
        ));
    }

    #[test]
    fn fainted_pet_is_auto_replaced() {
        let left = Side::new(
            "left",
            Team::new(vec![pet("Alpha", 30), pet("Gamma", 20)]).unwrap(),
        );
        let mut fragile = pet("Beta", 10);
        fragile.current_hp = 5;
        let right = Side::new("right", Team::new(vec![fragile]).unwrap());
        let mut battle = BattleState::new(left, right);

        let outcome = battle
            .resolve_turn([
                Action::UseMove { move_index: 0 },
                Action::UseMove { move_index: 0 },
            ])
            .unwrap();

        assert!(
            outcome
                .events
                .iter()
                .any(|event| matches!(event, TurnEvent::BattleEnded { winner: 0 }))
        );
    }
}
