CREATE TABLE pokemon (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        name        TEXT NOT NULL UNIQUE,
        element     TEXT NOT NULL DEFAULT '普通',
        evo_stage   TEXT DEFAULT '',
        ability     TEXT DEFAULT '',
        base_hp     INTEGER DEFAULT 0,
        base_atk    INTEGER DEFAULT 0,
        base_spatk  INTEGER DEFAULT 0,
        base_def    INTEGER DEFAULT 0,
        base_spdef  INTEGER DEFAULT 0,
        base_speed  INTEGER DEFAULT 0,
        base_total  INTEGER DEFAULT 0,
        stat_hp     INTEGER DEFAULT 0,
        stat_atk    INTEGER DEFAULT 0,
        stat_spatk  INTEGER DEFAULT 0,
        stat_def    INTEGER DEFAULT 0,
        stat_spdef  INTEGER DEFAULT 0,
        stat_speed  INTEGER DEFAULT 0
    , spirit_no TEXT DEFAULT '');
CREATE TABLE sqlite_sequence(name,seq);
CREATE TABLE skill (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        name        TEXT NOT NULL UNIQUE,
        element     TEXT NOT NULL DEFAULT '普通',
        category    TEXT NOT NULL DEFAULT '状态',
        energy_cost INTEGER DEFAULT 0,
        power       INTEGER DEFAULT 0,
        description TEXT DEFAULT '',
        source      TEXT DEFAULT 'wiki'
    );
CREATE TABLE pokemon_skill (
        pokemon_id  INTEGER NOT NULL,
        skill_id    INTEGER NOT NULL,
        PRIMARY KEY (pokemon_id, skill_id),
        FOREIGN KEY (pokemon_id) REFERENCES pokemon(id),
        FOREIGN KEY (skill_id)   REFERENCES skill(id)
    );
CREATE INDEX idx_pokemon_name ON pokemon(name);
CREATE INDEX idx_skill_name ON skill(name);
CREATE INDEX idx_pokemon_element ON pokemon(element);
CREATE INDEX idx_skill_element ON skill(element);
CREATE INDEX idx_ps_pokemon ON pokemon_skill(pokemon_id);
CREATE INDEX idx_ps_skill ON pokemon_skill(skill_id);
CREATE TABLE evolution (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            from_name   TEXT NOT NULL,
            to_name     TEXT NOT NULL,
            evo_level   INTEGER,
            condition   TEXT,
            chain_text  TEXT,
            UNIQUE(from_name, to_name)
        );
