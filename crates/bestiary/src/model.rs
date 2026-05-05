use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stats {
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special_attack: i32,
    pub special_defense: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveEffect {
    Damage { power: i32 },
    Status,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Move {
    pub id: String,
    pub name: String,
    pub element: String,
    pub category: String,
    pub priority: i8,
    pub energy_cost: i32,
    pub description: String,
    pub effect: MoveEffect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evolution {
    pub to_name: String,
    pub evo_level: Option<i32>,
    pub condition: Option<String>,
    pub chain_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetSpecies {
    pub pokemon_id: i64,
    pub species_id: String,
    pub name: String,
    pub element: String,
    pub evo_stage: String,
    pub ability: String,
    pub spirit_no: Option<String>,
    pub icon_path: Option<PathBuf>,
    pub stats: Stats,
    pub learnset: Vec<Move>,
    pub evolutions: Vec<Evolution>,
}
