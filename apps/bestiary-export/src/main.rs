use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use bestiary::sources::nrc::{NrcCatalogLoader, bundled_bundle_dir};
use bestiary::{Evolution, Move, MoveEffect, PetSpecies};
use serde::Serialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = workspace_root()?;
    let output_root = workspace_root
        .join("apps")
        .join("bestiary-web")
        .join("generated");
    let bundle_root = bundled_bundle_dir();

    export_site_data(&bundle_root, &output_root)?;

    println!("Exported bestiary site data to {}", output_root.display());
    Ok(())
}

fn export_site_data(
    bundle_root: &Path,
    output_root: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let catalog = NrcCatalogLoader::load_from_bundle(bundle_root)?;
    let data_dir = output_root.join("data");
    let detail_dir = data_dir.join("species");
    let icons_dir = output_root.join("icons");

    if output_root.exists() {
        fs::remove_dir_all(output_root)?;
    }

    fs::create_dir_all(&detail_dir)?;
    fs::create_dir_all(&icons_dir)?;

    let species = catalog.species().to_vec();
    let evolves_from = build_predecessors(&species);
    let elements = unique_sorted(species.iter().map(|entry| entry.element.clone()));
    let stages = unique_sorted(species.iter().map(|entry| entry.evo_stage.clone()));

    let mut index_species = Vec::with_capacity(species.len());
    for entry in &species {
        let icon = copy_icon(entry, &icons_dir)?;
        let detail = species_detail(entry, icon.clone(), &evolves_from);
        let detail_path = detail_dir.join(format!("{}.json", entry.species_id));
        write_json(&detail_path, &detail)?;

        index_species.push(species_card(entry, icon));
    }

    let index = SpeciesIndex {
        generated_at: generated_timestamp(),
        species_count: index_species.len(),
        elements,
        stages,
        species: index_species,
    };

    write_json(&data_dir.join("species-index.json"), &index)?;

    Ok(())
}

fn species_card(entry: &PetSpecies, icon: Option<String>) -> SpeciesCard {
    SpeciesCard {
        id: entry.species_id.clone(),
        pokemon_id: entry.pokemon_id,
        name: entry.name.clone(),
        element: entry.element.clone(),
        evo_stage: entry.evo_stage.clone(),
        ability: display_text(&entry.ability),
        spirit_no: normalized_option(entry.spirit_no.as_deref()),
        icon,
        stats: StatsDto::from_species(entry),
        move_count: entry.learnset.len(),
        evolution_count: entry.evolutions.len(),
    }
}

fn species_detail(
    entry: &PetSpecies,
    icon: Option<String>,
    evolves_from: &HashMap<String, Vec<String>>,
) -> SpeciesDetail {
    SpeciesDetail {
        id: entry.species_id.clone(),
        pokemon_id: entry.pokemon_id,
        name: entry.name.clone(),
        element: entry.element.clone(),
        evo_stage: entry.evo_stage.clone(),
        ability: display_text(&entry.ability),
        spirit_no: normalized_option(entry.spirit_no.as_deref()),
        icon,
        stats: StatsDto::from_species(entry),
        learnset: entry.learnset.iter().map(MoveDto::from).collect(),
        evolves_from: evolves_from.get(&entry.name).cloned().unwrap_or_default(),
        evolves_to: entry.evolutions.iter().map(EvolutionDto::from).collect(),
    }
}

fn build_predecessors(species: &[PetSpecies]) -> HashMap<String, Vec<String>> {
    let mut predecessors = HashMap::<String, Vec<String>>::new();

    for entry in species {
        for evolution in &entry.evolutions {
            predecessors
                .entry(evolution.to_name.clone())
                .or_default()
                .push(entry.name.clone());
        }
    }

    for entries in predecessors.values_mut() {
        entries.sort();
        entries.dedup();
    }

    predecessors
}

fn unique_sorted(values: impl Iterator<Item = String>) -> Vec<String> {
    values
        .filter(|value| !value.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn copy_icon(
    entry: &PetSpecies,
    icons_dir: &Path,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let Some(source_path) = &entry.icon_path else {
        return Ok(None);
    };

    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("png");
    let file_name = format!("{}.{}", entry.species_id, extension);
    let target_path = icons_dir.join(&file_name);
    fs::copy(source_path, &target_path)?;

    Ok(Some(format!("icons/{file_name}")))
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let body = serde_json::to_string_pretty(value)?;
    fs::write(path, body)?;
    Ok(())
}

fn workspace_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(2)
        .map(Path::to_path_buf)
        .ok_or_else(|| "Failed to resolve workspace root".into())
}

fn generated_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

fn normalized_option(value: Option<&str>) -> Option<String> {
    value.and_then(|text| {
        let normalized = display_text(text);
        if normalized.is_empty() {
            None
        } else {
            Some(normalized)
        }
    })
}

fn display_text(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == ":" {
        String::new()
    } else {
        trimmed.to_string()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SpeciesIndex {
    generated_at: String,
    species_count: usize,
    elements: Vec<String>,
    stages: Vec<String>,
    species: Vec<SpeciesCard>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SpeciesCard {
    id: String,
    pokemon_id: i64,
    name: String,
    element: String,
    evo_stage: String,
    ability: String,
    spirit_no: Option<String>,
    icon: Option<String>,
    stats: StatsDto,
    move_count: usize,
    evolution_count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SpeciesDetail {
    id: String,
    pokemon_id: i64,
    name: String,
    element: String,
    evo_stage: String,
    ability: String,
    spirit_no: Option<String>,
    icon: Option<String>,
    stats: StatsDto,
    learnset: Vec<MoveDto>,
    evolves_from: Vec<String>,
    evolves_to: Vec<EvolutionDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct StatsDto {
    max_hp: i32,
    attack: i32,
    defense: i32,
    speed: i32,
    special_attack: i32,
    special_defense: i32,
    total: i32,
}

impl StatsDto {
    fn from_species(entry: &PetSpecies) -> Self {
        let stats = &entry.stats;

        Self {
            max_hp: stats.max_hp,
            attack: stats.attack,
            defense: stats.defense,
            speed: stats.speed,
            special_attack: stats.special_attack,
            special_defense: stats.special_defense,
            total: stats.max_hp
                + stats.attack
                + stats.defense
                + stats.speed
                + stats.special_attack
                + stats.special_defense,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MoveDto {
    id: String,
    name: String,
    element: String,
    category: String,
    priority: i8,
    energy_cost: i32,
    description: String,
    effect: MoveEffectDto,
}

impl From<&Move> for MoveDto {
    fn from(value: &Move) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            element: value.element.clone(),
            category: value.category.clone(),
            priority: value.priority,
            energy_cost: value.energy_cost,
            description: value.description.clone(),
            effect: MoveEffectDto::from(&value.effect),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MoveEffectDto {
    kind: &'static str,
    power: Option<i32>,
}

impl From<&MoveEffect> for MoveEffectDto {
    fn from(value: &MoveEffect) -> Self {
        match value {
            MoveEffect::Damage { power } => Self {
                kind: "damage",
                power: Some(*power),
            },
            MoveEffect::Status => Self {
                kind: "status",
                power: None,
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EvolutionDto {
    to_name: String,
    evo_level: Option<i32>,
    condition: Option<String>,
    chain_text: Option<String>,
}

impl From<&Evolution> for EvolutionDto {
    fn from(value: &Evolution) -> Self {
        Self {
            to_name: value.to_name.clone(),
            evo_level: value.evo_level,
            condition: value.condition.clone(),
            chain_text: value.chain_text.clone(),
        }
    }
}
