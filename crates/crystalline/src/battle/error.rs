#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattleError {
    InvalidBattlePet(String),
    InvalidTeam(String),
    InvalidAction(String),
    BattleAlreadyFinished,
}

impl std::fmt::Display for BattleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBattlePet(message) => write!(f, "{message}"),
            Self::InvalidTeam(message) => write!(f, "{message}"),
            Self::InvalidAction(message) => write!(f, "{message}"),
            Self::BattleAlreadyFinished => write!(f, "battle already finished"),
        }
    }
}

impl std::error::Error for BattleError {}
