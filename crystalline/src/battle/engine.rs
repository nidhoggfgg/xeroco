use std::cmp::Reverse;

use crate::battle::{Action, BattleError, BattleMoveEffect, Side, TurnEvent, TurnOutcome, rules};

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
            let active_pet = self.sides[side].team.active_pet();
            let priority = rules::action_priority(active_pet, &action);
            let speed = active_pet.stats.speed;
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
            BattleMoveEffect::Damage { .. } => {
                let damage = rules::calculate_damage(&source_pet, &target_pet, &chosen_move.effect)
                    .expect("damage effect should calculate damage");
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
            BattleMoveEffect::Status => {}
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
