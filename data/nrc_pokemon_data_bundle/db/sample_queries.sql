-- List the first 20 Pokemon
SELECT id, name, element, evo_stage, ability
FROM pokemon
ORDER BY id
LIMIT 20;

-- Get one Pokemon by exact name
SELECT *
FROM pokemon
WHERE name = '圣羽翼王';

-- Search Pokemon by keyword
SELECT id, name, element, ability
FROM pokemon
WHERE name LIKE '%翼王%' OR ability LIKE '%翼王%'
ORDER BY name;

-- List all skills for one Pokemon
SELECT s.name, s.element, s.category, s.energy_cost, s.power, s.description
FROM skill s
JOIN pokemon_skill ps ON ps.skill_id = s.id
JOIN pokemon p ON p.id = ps.pokemon_id
WHERE p.name = '圣羽翼王'
ORDER BY s.energy_cost, s.name;

-- List all Pokemon that can learn one skill
SELECT p.name, p.element, p.evo_stage
FROM pokemon p
JOIN pokemon_skill ps ON ps.pokemon_id = p.id
JOIN skill s ON s.id = ps.skill_id
WHERE s.name = '毒雾'
ORDER BY p.name;

-- Get evolution links for a Pokemon chain
SELECT from_name, to_name, evo_level, condition, chain_text
FROM evolution
WHERE chain_text LIKE '%喵喵%'
ORDER BY id;

-- Match Pokemon rows that have a resource number
SELECT id, name, spirit_no
FROM pokemon
WHERE spirit_no IS NOT NULL AND spirit_no != ''
ORDER BY spirit_no, name
LIMIT 50;
