use crystalline::app::{BattleDemoRequest, BattleDemoService};
use crystalline::battle::Action;
use crystalline::pets::bundled_nrc_bundle_dir;

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
