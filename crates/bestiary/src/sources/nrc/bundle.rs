use std::path::PathBuf;

pub fn bundled_bundle_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("data")
        .join("nrc_pokemon_data_bundle")
}
