# Quickstart

## Recommended Way To Use This Bundle

1. Keep the folder structure unchanged.
2. Point your application to `db/nrc.db`.
3. If you need Pokemon images, read files from `resources/icons/`.
4. If you need evolution source snapshots or to rebuild part of the dataset, use the files in `resources/`.

## SQLite Examples

List Pokemon:

```bash
sqlite3 db/nrc.db "SELECT id, name, element, ability FROM pokemon ORDER BY id LIMIT 10;"
```

List skills for one Pokemon:

```bash
sqlite3 db/nrc.db "
SELECT s.name, s.element, s.category, s.energy_cost, s.power
FROM skill s
JOIN pokemon_skill ps ON ps.skill_id = s.id
JOIN pokemon p ON p.id = ps.pokemon_id
WHERE p.name = '圣羽翼王'
ORDER BY s.energy_cost, s.name;
"
```

Find all learners of one skill:

```bash
sqlite3 db/nrc.db "
SELECT p.name
FROM pokemon p
JOIN pokemon_skill ps ON ps.pokemon_id = p.id
JOIN skill s ON s.id = ps.skill_id
WHERE s.name = '毒雾'
ORDER BY p.name;
"
```

Find a Pokemon evolution chain:

```bash
sqlite3 db/nrc.db "
SELECT from_name, to_name, evo_level, condition
FROM evolution
WHERE chain_text LIKE '%喵喵%'
ORDER BY id;
"
```

## Matching Database Rows To Icons

Recommended order:

1. Use `pokemon.spirit_no` when it is present.
2. If you also need form-specific matching, combine `spirit_no` and `pokemon.name`.
3. Use the icon filename pattern `NOxxx_名字.png`.

Examples:

- `NO002_喵喵.png`
- `NO152_圣羽翼王.png`

## Important Limitation

The `skill` table contains structured skill metadata such as name, element, category, energy cost, power, description, and source.

It does not contain the full structured battle effect model used by the original simulator. If your target project needs combat rule execution instead of data display or lookup, you will need to port the effect configuration from the original `src/` Python modules as well.
