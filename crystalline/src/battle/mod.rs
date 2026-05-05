mod action;
mod engine;
mod error;
mod event;
mod rules;
mod state;

pub use action::Action;
pub use engine::BattleState;
pub use error::BattleError;
pub use event::{TurnEvent, TurnOutcome};
pub use state::{
    BattleEffect, BattleMove, BattleMoveSemantics, BattlePet, BattleStats, BattleTarget, Side, Team,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn strike() -> BattleMove {
        BattleMove {
            id: "strike".to_string(),
            name: "Strike".to_string(),
            priority: 0,
            semantics: BattleMoveSemantics {
                effects: vec![BattleEffect::DealDamage {
                    power: 10,
                    target: BattleTarget::OpponentActive,
                }],
            },
        }
    }

    fn quick_strike() -> BattleMove {
        BattleMove {
            id: "quick_strike".to_string(),
            name: "Quick Strike".to_string(),
            priority: 1,
            semantics: BattleMoveSemantics {
                effects: vec![BattleEffect::DealDamage {
                    power: 6,
                    target: BattleTarget::OpponentActive,
                }],
            },
        }
    }

    fn pet(name: &str, speed: i32) -> BattlePet {
        BattlePet::new(
            name.to_lowercase(),
            name.to_lowercase(),
            name,
            "Neutral",
            BattleStats {
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

    #[test]
    fn status_placeholder_produces_event() {
        let mut alpha = pet("Alpha", 20);
        alpha.moves = vec![BattleMove {
            id: "focus".to_string(),
            name: "Focus".to_string(),
            priority: 0,
            semantics: BattleMoveSemantics {
                effects: vec![BattleEffect::StatusPlaceholder],
            },
        }];
        let beta = pet("Beta", 10);
        let left = Side::new("left", Team::new(vec![alpha]).unwrap());
        let right = Side::new("right", Team::new(vec![beta]).unwrap());
        let mut battle = BattleState::new(left, right);

        let outcome = battle
            .resolve_turn([Action::UseMove { move_index: 0 }, Action::Pass])
            .unwrap();

        assert!(
            outcome
                .events
                .iter()
                .any(|event| matches!(event, TurnEvent::MoveHadNoEffect { side: 0, .. }))
        );
    }
}
