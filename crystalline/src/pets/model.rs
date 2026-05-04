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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pet {
    pub id: String,
    pub name: String,
    pub element: String,
    pub stats: Stats,
    pub current_hp: i32,
    pub moves: Vec<Move>,
}
