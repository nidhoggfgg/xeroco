use std::path::Path;

use crystalline::battle::{Action, BattleState, Side, Team};
use crystalline::pets::{PetCatalog, bundled_nrc_bundle_dir};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (left_name, right_name) = args_from_cli();
    let bundle_dir = bundled_nrc_bundle_dir();
    let battle = sample_battle(&bundle_dir, &left_name, &right_name)?;
    let mut battle = battle;

    let outcome = battle.resolve_turn([
        Action::UseMove { move_index: 0 },
        Action::UseMove { move_index: 0 },
    ])?;

    println!("turn {} outcome:", battle.turn);
    for event in outcome.events {
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

fn sample_battle(
    bundle_dir: &Path,
    left_name: &str,
    right_name: &str,
) -> Result<BattleState, Box<dyn std::error::Error>> {
    let catalog = PetCatalog::from_nrc_bundle(bundle_dir)?;
    let left_pet = catalog.instantiate_by_name(left_name)?;
    let right_pet = catalog.instantiate_by_name(right_name)?;

    let left = Side::new("Player A", Team::new(vec![left_pet])?);
    let right = Side::new("Player B", Team::new(vec![right_pet])?);

    Ok(BattleState::new(left, right))
}
