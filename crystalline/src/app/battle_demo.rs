use std::path::Path;

use crate::battle::{Action, BattleError, BattleState, Side, Team, TurnEvent};
use crate::battle_adapter::{AdapterError, BattleRosterBuilder};
use crate::pets::{PetCatalog, PetCatalogService, PetSystemError};

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
    PetSystem(PetSystemError),
    Adapter(AdapterError),
    Battle(BattleError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PetSystem(error) => write!(f, "{error}"),
            Self::Adapter(error) => write!(f, "{error}"),
            Self::Battle(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<PetSystemError> for AppError {
    fn from(value: PetSystemError) -> Self {
        Self::PetSystem(value)
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
