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
    MoveHadNoEffect {
        side: usize,
        move_name: String,
        reason: String,
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
