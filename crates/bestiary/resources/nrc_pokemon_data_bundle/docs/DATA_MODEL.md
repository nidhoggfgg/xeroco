# Data Model

## User Tables

### `pokemon`

Primary Pokemon metadata table.

Important fields:

- `id`: internal numeric primary key
- `name`: Pokemon name
- `element`: Pokemon element/type
- `evo_stage`: evolution stage label
- `ability`: raw ability text
- `base_hp`, `base_atk`, `base_spatk`, `base_def`, `base_spdef`, `base_speed`: base stats
- `base_total`: total base stat sum
- `stat_hp`, `stat_atk`, `stat_spatk`, `stat_def`, `stat_spdef`, `stat_speed`: imported stat snapshot from source sheet
- `spirit_no`: external Pokemon number used by resource files

### `skill`

Skill metadata table.

Important fields:

- `id`: internal numeric primary key
- `name`: skill name
- `element`: skill element/type
- `category`: skill category
- `energy_cost`: energy cost
- `power`: power value
- `description`: raw skill description text
- `source`: import source label

### `pokemon_skill`

Many-to-many relation between Pokemon and skills.

Fields:

- `pokemon_id`
- `skill_id`

Use this table together with `pokemon` and `skill` to build learnsets.

### `evolution`

Pokemon evolution relation table.

Fields:

- `id`: internal numeric primary key
- `from_name`: pre-evolution Pokemon name
- `to_name`: evolved Pokemon name
- `evo_level`: evolution level if known
- `condition`: special evolution condition if known
- `chain_text`: raw chain text from source data

## Resource Files

### `resources/icons/`

PNG image files for Pokemon art/icons.

Naming pattern:

- `NO001_迪莫.png`
- `NO078_千棘盔.png`

Notes:

- Alternate forms may share the same numeric prefix but differ by Pokemon name.
- Resource matching should not assume one icon per `spirit_no`.

### `resources/spirit_evolution.csv`

Raw evolution and image metadata source.

Useful columns include:

- `编号`
- `名字`
- `阶段`
- `属性`
- `形态分类`
- `形态`
- `进化链`
- `进化等级`
- `进化条件`
- `图片文件名`

### `resources/pokemon_stats.xlsx`

Original spreadsheet source for Pokemon base data imported into `pokemon`.

## Integration Notes

- For display/search/filter use cases, `nrc.db` is usually enough.
- For artwork, use `resources/icons/`.
- For combat simulation logic, this bundle is incomplete by design because structured effect execution rules are not stored in SQLite.
