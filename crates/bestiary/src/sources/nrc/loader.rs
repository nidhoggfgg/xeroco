use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::{BestiaryError, Evolution, Move, MoveEffect, PetCatalog, PetSpecies, Stats};

#[derive(Debug, Clone, Copy, Default)]
pub struct NrcCatalogLoader;

impl NrcCatalogLoader {
    pub fn load_from_bundle(bundle_root: impl AsRef<Path>) -> Result<PetCatalog, BestiaryError> {
        let bundle_root = bundle_root.as_ref();
        let db_path = bundle_root.join("db").join("nrc.db");
        let icons_dir = bundle_root.join("resources").join("icons");
        let connection = Connection::open(&db_path)?;

        Self::load_from_connection(
            &connection,
            if icons_dir.is_dir() {
                Some(icons_dir.as_path())
            } else {
                None
            },
        )
    }

    pub fn load_from_connection(
        connection: &Connection,
        icons_dir: Option<&Path>,
    ) -> Result<PetCatalog, BestiaryError> {
        let icon_index = build_icon_index(icons_dir)?;
        let evolutions = load_evolutions(connection)?;
        let species = load_species(connection, &icon_index, &evolutions)?;
        Ok(PetCatalog::new(species))
    }
}

pub fn load_catalog(bundle_root: impl AsRef<Path>) -> Result<PetCatalog, BestiaryError> {
    NrcCatalogLoader::load_from_bundle(bundle_root)
}

fn load_species(
    connection: &Connection,
    icon_index: &HashMap<String, Vec<PathBuf>>,
    evolutions: &HashMap<String, Vec<Evolution>>,
) -> Result<Vec<PetSpecies>, BestiaryError> {
    #[derive(Debug)]
    struct SpeciesRow {
        pokemon_id: i64,
        name: String,
        element: String,
        evo_stage: String,
        ability: String,
        base_hp: i32,
        base_atk: i32,
        base_spatk: i32,
        base_def: i32,
        base_spdef: i32,
        base_speed: i32,
        spirit_no: Option<String>,
        move_id: Option<i64>,
        move_name: Option<String>,
        move_element: Option<String>,
        move_category: Option<String>,
        move_energy_cost: Option<i32>,
        move_power: Option<i32>,
        move_description: Option<String>,
    }

    let mut statement = connection.prepare(
        r#"
        SELECT
            p.id,
            p.name,
            p.element,
            p.evo_stage,
            p.ability,
            p.base_hp,
            p.base_atk,
            p.base_spatk,
            p.base_def,
            p.base_spdef,
            p.base_speed,
            p.spirit_no,
            s.id,
            s.name,
            s.element,
            s.category,
            s.energy_cost,
            s.power,
            s.description
        FROM pokemon p
        LEFT JOIN pokemon_skill ps ON ps.pokemon_id = p.id
        LEFT JOIN skill s ON s.id = ps.skill_id
        ORDER BY p.id, s.id
        "#,
    )?;

    let rows = statement.query_map([], |row| {
        Ok(SpeciesRow {
            pokemon_id: row.get(0)?,
            name: row.get(1)?,
            element: row.get(2)?,
            evo_stage: row.get(3)?,
            ability: row.get(4)?,
            base_hp: row.get(5)?,
            base_atk: row.get(6)?,
            base_spatk: row.get(7)?,
            base_def: row.get(8)?,
            base_spdef: row.get(9)?,
            base_speed: row.get(10)?,
            spirit_no: row.get(11)?,
            move_id: row.get(12)?,
            move_name: row.get(13)?,
            move_element: row.get(14)?,
            move_category: row.get(15)?,
            move_energy_cost: row.get(16)?,
            move_power: row.get(17)?,
            move_description: row.get(18)?,
        })
    })?;

    let mut species_by_id = HashMap::<i64, PetSpecies>::new();
    for row in rows {
        let row = row?;
        let species = species_by_id.entry(row.pokemon_id).or_insert_with(|| {
            let spirit_no = row.spirit_no.clone();
            PetSpecies {
                pokemon_id: row.pokemon_id,
                species_id: species_id_for(row.pokemon_id, &row.name),
                name: row.name.clone(),
                element: row.element.clone(),
                evo_stage: row.evo_stage.clone(),
                ability: row.ability.clone(),
                spirit_no: spirit_no.clone(),
                icon_path: resolve_icon_path(icon_index, spirit_no.as_deref(), &row.name),
                stats: Stats {
                    max_hp: row.base_hp,
                    attack: row.base_atk,
                    defense: row.base_def,
                    speed: row.base_speed,
                    special_attack: row.base_spatk,
                    special_defense: row.base_spdef,
                },
                learnset: Vec::new(),
                evolutions: evolutions.get(&row.name).cloned().unwrap_or_default(),
            }
        });

        if let Some(move_id) = row.move_id {
            let move_key = move_id.to_string();
            if species
                .learnset
                .iter()
                .any(|battle_move| battle_move.id == move_key)
            {
                continue;
            }

            let power = row.move_power.unwrap_or_default();
            species.learnset.push(Move {
                id: move_key,
                name: row.move_name.unwrap_or_default(),
                element: row.move_element.unwrap_or_else(|| "普通".to_string()),
                category: row.move_category.unwrap_or_else(|| "状态".to_string()),
                priority: 0,
                energy_cost: row.move_energy_cost.unwrap_or_default(),
                description: row.move_description.unwrap_or_default(),
                effect: if power > 0 {
                    MoveEffect::Damage { power }
                } else {
                    MoveEffect::Status
                },
            });
        }
    }

    let mut species: Vec<_> = species_by_id.into_values().collect();
    species.sort_by_key(|entry| entry.pokemon_id);
    Ok(species)
}

fn load_evolutions(
    connection: &Connection,
) -> Result<HashMap<String, Vec<Evolution>>, BestiaryError> {
    let mut statement = connection.prepare(
        r#"
        SELECT
            from_name,
            to_name,
            evo_level,
            condition,
            chain_text
        FROM evolution
        ORDER BY id
        "#,
    )?;

    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            Evolution {
                to_name: row.get(1)?,
                evo_level: row.get(2)?,
                condition: row.get(3)?,
                chain_text: row.get(4)?,
            },
        ))
    })?;

    let mut evolutions = HashMap::<String, Vec<Evolution>>::new();
    for row in rows {
        let (from_name, evolution) = row?;
        evolutions.entry(from_name).or_default().push(evolution);
    }

    Ok(evolutions)
}

fn build_icon_index(
    icons_dir: Option<&Path>,
) -> Result<HashMap<String, Vec<PathBuf>>, BestiaryError> {
    let Some(icons_dir) = icons_dir else {
        return Ok(HashMap::new());
    };

    let mut icon_index = HashMap::<String, Vec<PathBuf>>::new();
    for entry in std::fs::read_dir(icons_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("png") {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some((prefix, _)) = file_name.split_once('_') else {
            continue;
        };

        icon_index
            .entry(prefix.to_string())
            .or_default()
            .push(path.clone());
    }

    for entries in icon_index.values_mut() {
        entries.sort();
    }

    Ok(icon_index)
}

fn resolve_icon_path(
    icon_index: &HashMap<String, Vec<PathBuf>>,
    spirit_no: Option<&str>,
    name: &str,
) -> Option<PathBuf> {
    let prefix = normalize_spirit_no(spirit_no?)?;
    let candidates = icon_index.get(&prefix)?;
    let exact_suffix = format!("_{name}.png");

    candidates
        .iter()
        .find(|path| {
            path.file_name()
                .and_then(|file_name| file_name.to_str())
                .is_some_and(|file_name| file_name.ends_with(&exact_suffix))
        })
        .cloned()
        .or_else(|| candidates.first().cloned())
}

fn normalize_spirit_no(spirit_no: &str) -> Option<String> {
    let digits: String = spirit_no.chars().filter(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }

    Some(format!("NO{:0>3}", digits))
}

fn species_id_for(pokemon_id: i64, name: &str) -> String {
    let mut slug = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if !ch.is_whitespace() {
            slug.push(ch);
        }
    }

    format!("pokemon-{pokemon_id}-{slug}")
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn loads_species_from_sqlite() {
        let connection = Connection::open_in_memory().unwrap();
        connection
            .execute_batch(
                r#"
                CREATE TABLE pokemon (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    element TEXT NOT NULL,
                    evo_stage TEXT DEFAULT '',
                    ability TEXT DEFAULT '',
                    base_hp INTEGER DEFAULT 0,
                    base_atk INTEGER DEFAULT 0,
                    base_spatk INTEGER DEFAULT 0,
                    base_def INTEGER DEFAULT 0,
                    base_spdef INTEGER DEFAULT 0,
                    base_speed INTEGER DEFAULT 0,
                    base_total INTEGER DEFAULT 0,
                    spirit_no TEXT
                );
                CREATE TABLE skill (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    element TEXT NOT NULL,
                    category TEXT NOT NULL,
                    energy_cost INTEGER DEFAULT 0,
                    power INTEGER DEFAULT 0,
                    description TEXT DEFAULT ''
                );
                CREATE TABLE pokemon_skill (
                    pokemon_id INTEGER NOT NULL,
                    skill_id INTEGER NOT NULL
                );
                CREATE TABLE evolution (
                    id INTEGER PRIMARY KEY,
                    from_name TEXT NOT NULL,
                    to_name TEXT NOT NULL,
                    evo_level INTEGER,
                    condition TEXT DEFAULT '',
                    chain_text TEXT DEFAULT ''
                );

                INSERT INTO pokemon (
                    id, name, element, evo_stage, ability, base_hp, base_atk, base_spatk,
                    base_def, base_spdef, base_speed, spirit_no
                ) VALUES (
                    1, '迪莫', '光', '完全体', '圣光护佑', 120, 95, 110, 85, 90, 100, 'No.439'
                );

                INSERT INTO skill (
                    id, name, element, category, energy_cost, power, description
                ) VALUES (
                    10, '聚能光照', '光', '魔法', 15, 90, '用光能打击对手。'
                );

                INSERT INTO pokemon_skill (pokemon_id, skill_id) VALUES (1, 10);

                INSERT INTO evolution (
                    id, from_name, to_name, evo_level, condition, chain_text
                ) VALUES (
                    100, '迪莫', '圣光迪莫', '100', '觉醒', '迪莫 -> 圣光迪莫'
                );
                "#,
            )
            .unwrap();

        let catalog = NrcCatalogLoader::load_from_connection(&connection, None).unwrap();
        let species = catalog.species();
        assert_eq!(species.len(), 1);

        let dimo = &species[0];
        assert_eq!(dimo.species_id, "pokemon-1-迪莫");
        assert_eq!(dimo.name, "迪莫");
        assert_eq!(dimo.element, "光");
        assert_eq!(dimo.stats.max_hp, 120);
        assert_eq!(dimo.learnset.len(), 1);
        assert_eq!(dimo.learnset[0].name, "聚能光照");
        assert_eq!(dimo.evolutions.len(), 1);
        assert_eq!(dimo.evolutions[0].to_name, "圣光迪莫");
    }
}
