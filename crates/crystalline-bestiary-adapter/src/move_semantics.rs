use bestiary::{Move, MoveEffect};
use crystalline::battle::{BattleEffect, BattleMove, BattleMoveSemantics, BattleTarget};

pub(crate) fn map_move(move_definition: Move) -> BattleMove {
    BattleMove {
        id: move_definition.id,
        name: move_definition.name,
        priority: move_definition.priority,
        semantics: map_move_semantics(move_definition.effect),
    }
}

fn map_move_semantics(effect: MoveEffect) -> BattleMoveSemantics {
    match effect {
        MoveEffect::Damage { power } => BattleMoveSemantics {
            effects: vec![BattleEffect::DealDamage {
                power,
                target: BattleTarget::OpponentActive,
            }],
        },
        MoveEffect::Status => BattleMoveSemantics {
            effects: vec![BattleEffect::StatusPlaceholder],
        },
    }
}
