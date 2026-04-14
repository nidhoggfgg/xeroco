use std::fs;
use std::path::{Path, PathBuf};

use crate::battle::{BattleError, Pet};

pub fn load_pet(path: impl AsRef<Path>) -> Result<Pet, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let pet: Pet = serde_json::from_str(&content)?;
    validate_loaded_pet(pet)
}

pub fn load_pets_dir(path: impl AsRef<Path>) -> Result<Vec<Pet>, Box<dyn std::error::Error>> {
    let mut entries: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
        .collect();
    entries.sort();

    let mut pets = Vec::with_capacity(entries.len());
    for path in entries {
        pets.push(load_pet(path)?);
    }

    Ok(pets)
}

fn validate_loaded_pet(pet: Pet) -> Result<Pet, Box<dyn std::error::Error>> {
    let expected_hp = pet.stats.max_hp;
    let validated = Pet::new(
        pet.id.clone(),
        pet.name.clone(),
        pet.stats,
        pet.skills,
    )
    .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;

    if pet.current_hp != expected_hp {
        return Err(Box::new(BattleError::InvalidPet(format!(
            "pet {} must start with current_hp equal to max_hp",
            pet.id
        ))));
    }

    Ok(validated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_all_sample_pets() {
        let sample_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("pets");
        let pets = load_pets_dir(&sample_dir).expect("sample pets should load");
        assert!(!pets.is_empty());
        assert!(pets.iter().all(|pet| pet.current_hp == pet.stats.max_hp));
        assert!(pets.iter().all(|pet| (1..=4).contains(&pet.skills.len())));
    }
}
