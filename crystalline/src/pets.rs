use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::battle::{BattleError, Move, Pet, Stats};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PetDefinition {
    pub id: String,
    pub name: String,
    pub stats: Stats,
    pub move_ids: Vec<String>,
}

pub fn load_move(path: impl AsRef<Path>) -> Result<Move, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

pub fn load_moves_dir(
    path: impl AsRef<Path>,
) -> Result<HashMap<String, Move>, Box<dyn std::error::Error>> {
    let mut entries: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
        .collect();
    entries.sort();

    let mut moves = HashMap::with_capacity(entries.len());
    for path in entries {
        let battle_move = load_move(path)?;
        moves.insert(battle_move.id.clone(), battle_move);
    }

    Ok(moves)
}

pub fn load_pet_definition(
    path: impl AsRef<Path>,
) -> Result<PetDefinition, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

pub fn load_pets_dir(
    pets_path: impl AsRef<Path>,
    moves_path: impl AsRef<Path>,
) -> Result<Vec<Pet>, Box<dyn std::error::Error>> {
    let move_catalog = load_moves_dir(moves_path)?;
    let mut entries: Vec<PathBuf> = fs::read_dir(pets_path)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
        .collect();
    entries.sort();

    let mut pets = Vec::with_capacity(entries.len());
    for path in entries {
        let definition = load_pet_definition(path)?;
        pets.push(build_pet(definition, &move_catalog)?);
    }

    Ok(pets)
}

fn build_pet(
    definition: PetDefinition,
    move_catalog: &HashMap<String, Move>,
) -> Result<Pet, Box<dyn std::error::Error>> {
    if definition.move_ids.is_empty() || definition.move_ids.len() > 4 {
        return Err(Box::new(BattleError::InvalidPet(format!(
            "pet {} must reference between 1 and 4 moves, got {}",
            definition.id,
            definition.move_ids.len()
        ))));
    }

    let mut moves = Vec::with_capacity(definition.move_ids.len());
    for move_id in &definition.move_ids {
        let battle_move = move_catalog.get(move_id).ok_or_else(|| {
            Box::new(BattleError::InvalidPet(format!(
                "pet {} references unknown move {}",
                definition.id, move_id
            ))) as Box<dyn std::error::Error>
        })?;
        moves.push(battle_move.clone());
    }

    Ok(Pet::new(
        definition.id,
        definition.name,
        definition.stats,
        moves,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_all_sample_pets() {
        let sample_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let pets = load_pets_dir(sample_root.join("pets"), sample_root.join("moves"))
            .expect("sample pets should load");
        assert!(!pets.is_empty());
        assert!(pets.iter().all(|pet| pet.current_hp == pet.stats.max_hp));
        assert!(pets.iter().all(|pet| (1..=4).contains(&pet.moves.len())));
    }
}
