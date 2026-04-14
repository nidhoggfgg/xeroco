use std::path::{Path, PathBuf};

use crystalline::battle::{Action, BattleState, Side, Team};
use crystalline::pets::load_pets_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pets_dir = pets_dir_from_args()?;
    let battle = sample_battle(&pets_dir)?;
    let mut battle = battle;

    let outcome = battle.resolve_turn([
        Action::UseSkill { skill_index: 1 },
        Action::UseSkill { skill_index: 0 },
    ])?;

    println!("turn {} outcome:", battle.turn);
    for event in outcome.events {
        println!("{event:?}");
    }

    Ok(())
}

fn pets_dir_from_args() -> Result<PathBuf, Box<dyn std::error::Error>> {
    std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| "usage: cargo run -- <pets-dir>".into())
}

fn sample_battle(pets_dir: &Path) -> Result<BattleState, Box<dyn std::error::Error>> {
    let pets = load_pets_dir(pets_dir)?;
    let firecub = pets
        .iter()
        .find(|pet| pet.id == "firecub")
        .cloned()
        .ok_or("missing firecub pet data")?;
    let leafling = pets
        .iter()
        .find(|pet| pet.id == "leafling")
        .cloned()
        .ok_or("missing leafling pet data")?;

    let left = Side::new("Player A", Team::new(vec![firecub])?);
    let right = Side::new("Player B", Team::new(vec![leafling])?);

    Ok(BattleState::new(left, right))
}
