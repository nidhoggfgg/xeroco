use bestiary::BestiaryError;
use crystalline::battle::BattleError;

#[derive(Debug)]
pub enum AdapterError {
    MissingMove(String),
    InvalidSelection(String),
    Battle(BattleError),
    Bestiary(BestiaryError),
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingMove(move_id) => write!(f, "missing move definition for {move_id}"),
            Self::InvalidSelection(message) => write!(f, "{message}"),
            Self::Battle(error) => write!(f, "{error}"),
            Self::Bestiary(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for AdapterError {}

impl From<BattleError> for AdapterError {
    fn from(value: BattleError) -> Self {
        Self::Battle(value)
    }
}

impl From<BestiaryError> for AdapterError {
    fn from(value: BestiaryError) -> Self {
        Self::Bestiary(value)
    }
}
