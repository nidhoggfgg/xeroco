# NRC Pokemon Data Bundle

This folder is a standalone data bundle extracted from the `NRC_AI` project for reuse in other projects.

## Contents

- `db/nrc.db`
  Main SQLite database. Includes Pokemon, skills, Pokemon-skill learn relations, and evolution data.
- `db/schema.sql`
  SQLite schema snapshot for the bundled database.
- `db/sample_queries.sql`
  Ready-to-run SQL examples.
- `resources/icons/`
  Pokemon icon images in PNG format.
- `resources/spirit_evolution.csv`
  Raw evolution source file used to build the `evolution` table.
- `resources/pokemon_stats.xlsx`
  Raw Pokemon stats spreadsheet used to build the `pokemon` table.
- `docs/QUICKSTART.md`
  Fast integration notes for other projects.
- `docs/DATA_MODEL.md`
  Table and field reference.

## What Is In The Database

`nrc.db` is the main structured data source in this bundle.

- `pokemon`: 461 rows
- `skill`: 491 rows
- `pokemon_skill`: 20433 rows
- `evolution`: 206 rows

Current skill source in this database: `bilibili`

## What Is Not In The Database

Two things are not fully represented inside `nrc.db`:

- Battle effect logic for skills and abilities
  The game-like structured effect rules live in the original codebase under `src/effect_data.py` and `src/skill_effects_generated.py`.
- Image binary data
  Icons are stored as files under `resources/icons/`, not inside SQLite.

If your other project only needs Pokemon metadata, skills, learnsets, and evolution chains, this bundle should be enough on its own.

## Resource Notes

- Icon file count: 371 PNG files
- Icon naming pattern: `NO001_迪莫.png`
- `spirit_no` in the database is useful for matching database rows with icon filenames, but it is not guaranteed to be unique across alternate forms.

## Suggested Usage

Open the database directly from SQLite-compatible tooling:

```bash
sqlite3 db/nrc.db
```

Or query it from application code with any standard SQLite library.
