use std::path::{Path, PathBuf};

use crystalline::battle::{Action, BattleState, Side, Team};
use crystalline::pets::load_pets_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (pets_dir, moves_dir) = data_dirs_from_args()?;
    let battle = sample_battle(&pets_dir, &moves_dir)?;
    let mut battle = battle;

    let outcome = battle.resolve_turn([
        Action::UseMove { move_index: 1 },
        Action::UseMove { move_index: 0 },
    ])?;

    println!("turn {} outcome:", battle.turn);
    for event in outcome.events {
        println!("{event:?}");
    }

    Ok(())
}

fn data_dirs_from_args() -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let pets_dir = args
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| "usage: cargo run -- <pets-dir> <moves-dir>".to_string())?;
    let moves_dir = args
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| "usage: cargo run -- <pets-dir> <moves-dir>".to_string())?;
    Ok((pets_dir, moves_dir))
}

fn sample_battle(
    pets_dir: &Path,
    moves_dir: &Path,
) -> Result<BattleState, Box<dyn std::error::Error>> {
    let pets = load_pets_dir(pets_dir, moves_dir)?;
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
