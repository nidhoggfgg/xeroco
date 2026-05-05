use crate::battle::{Action, BattleEffect, BattleMoveSemantics, BattlePet, BattleTarget};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MoveConsequence {
    Damage { target_side: usize, amount: i32 },
    NoEffect { reason: &'static str },
}

pub(crate) fn action_priority(active_pet: &BattlePet, action: &Action) -> i8 {
    match action {
        Action::UseMove { move_index } => active_pet.moves[*move_index].priority,
        Action::Switch { .. } => 6,
        Action::Pass => i8::MIN,
    }
}

pub(crate) fn calculate_damage(source_pet: &BattlePet, target_pet: &BattlePet, power: i32) -> i32 {
    (power + source_pet.stats.attack - target_pet.stats.defense).max(1)
}

pub(crate) fn resolve_move(
    source_side: usize,
    source_pet: &BattlePet,
    target_pet: &BattlePet,
    semantics: &BattleMoveSemantics,
) -> Vec<MoveConsequence> {
    semantics
        .effects
        .iter()
        .map(|effect| resolve_effect(source_side, source_pet, target_pet, effect))
        .collect()
}

fn resolve_effect(
    source_side: usize,
    source_pet: &BattlePet,
    target_pet: &BattlePet,
    effect: &BattleEffect,
) -> MoveConsequence {
    match effect {
        BattleEffect::DealDamage { power, target } => {
            let target_side = match target {
                BattleTarget::SelfActive => source_side,
                BattleTarget::OpponentActive => 1 - source_side,
            };
            let defending_pet = match target {
                BattleTarget::SelfActive => source_pet,
                BattleTarget::OpponentActive => target_pet,
            };

            MoveConsequence::Damage {
                target_side,
                amount: calculate_damage(source_pet, defending_pet, *power),
            }
        }
        BattleEffect::StatusPlaceholder => MoveConsequence::NoEffect {
            reason: "status semantics not implemented",
        },
    }
}
