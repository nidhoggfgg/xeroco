use std::path::Path;

use bestiary::{PetCatalog, PetCatalogService, BestiaryError, bundled_nrc_bundle_dir};
use crystalline::battle::{Action, BattleError, BattleState, Side, Team, TurnEvent};
use crystalline_bestiary_adapter::{AdapterError, BattleRosterBuilder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleDemoRequest {
    pub left_name: String,
    pub right_name: String,
}

impl BattleDemoRequest {
    pub fn new(left_name: impl Into<String>, right_name: impl Into<String>) -> Self {
        Self {
            left_name: left_name.into(),
            right_name: right_name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleTurnReport {
    pub turn: u32,
    pub events: Vec<TurnEvent>,
    pub winner: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct BattleDemoService {
    roster_builder: BattleRosterBuilder,
}

impl BattleDemoService {
    pub fn build_battle(
        &self,
        bundle_dir: &Path,
        request: &BattleDemoRequest,
    ) -> Result<BattleState, AppError> {
        let catalog = PetCatalog::from_nrc_bundle(bundle_dir)?;
        let query = catalog.service();
        let left_species = query.species_by_name(&request.left_name)?;
        let right_species = query.species_by_name(&request.right_name)?;
        let left_pet = self.roster_builder.build_pet_with_defaults(&left_species)?;
        let right_pet = self
            .roster_builder
            .build_pet_with_defaults(&right_species)?;

        let left = Side::new("Player A", Team::new(vec![left_pet])?);
        let right = Side::new("Player B", Team::new(vec![right_pet])?);

        Ok(BattleState::new(left, right))
    }

    pub fn run_turn(
        &self,
        bundle_dir: &Path,
        request: &BattleDemoRequest,
        actions: [Action; 2],
    ) -> Result<BattleTurnReport, AppError> {
        let mut battle = self.build_battle(bundle_dir, request)?;
        let outcome = battle.resolve_turn(actions)?;

        Ok(BattleTurnReport {
            turn: battle.turn,
            events: outcome.events,
            winner: outcome.winner,
        })
    }
}

#[derive(Debug)]
pub enum AppError {
    Bestiary(BestiaryError),
    Adapter(AdapterError),
    Battle(BattleError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bestiary(error) => write!(f, "{error}"),
            Self::Adapter(error) => write!(f, "{error}"),
            Self::Battle(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<BestiaryError> for AppError {
    fn from(value: BestiaryError) -> Self {
        Self::Bestiary(value)
    }
}

impl From<AdapterError> for AppError {
    fn from(value: AdapterError) -> Self {
        Self::Adapter(value)
    }
}

impl From<BattleError> for AppError {
    fn from(value: BattleError) -> Self {
        Self::Battle(value)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (left_name, right_name) = args_from_cli();
    let bundle_dir = bundled_nrc_bundle_dir();
    let service = BattleDemoService::default();
    let report = service.run_turn(
        &bundle_dir,
        &BattleDemoRequest::new(left_name, right_name),
        [
            Action::UseMove { move_index: 0 },
            Action::UseMove { move_index: 0 },
        ],
    )?;

    println!("turn {} outcome:", report.turn);
    for event in report.events {
        println!("{event:?}");
    }

    Ok(())
}

fn args_from_cli() -> (String, String) {
    let mut args = std::env::args_os().skip(1);
    let left_name = args
        .next()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "迪莫".to_string());
    let right_name = args
        .next()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "火花".to_string());

    (left_name, right_name)
}
