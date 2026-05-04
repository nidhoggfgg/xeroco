use crate::battle::{Action, BattleMoveEffect, BattlePet};

pub(crate) fn action_priority(active_pet: &BattlePet, action: &Action) -> i8 {
    match action {
        Action::UseMove { move_index } => active_pet.moves[*move_index].priority,
        Action::Switch { .. } => 6,
        Action::Pass => i8::MIN,
    }
}

pub(crate) fn calculate_damage(
    source_pet: &BattlePet,
    target_pet: &BattlePet,
    effect: &BattleMoveEffect,
) -> Option<i32> {
    match effect {
        BattleMoveEffect::Damage { power } => {
            Some((power + source_pet.stats.attack - target_pet.stats.defense).max(1))
        }
        BattleMoveEffect::Status => None,
    }
}
