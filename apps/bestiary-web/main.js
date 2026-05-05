const DATA_ROOT = "./generated/data";
const EMPTY_ICON =
  "data:image/svg+xml;charset=UTF-8," +
  encodeURIComponent(`
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120">
      <rect width="120" height="120" rx="28" fill="#F4E6D1"/>
      <circle cx="60" cy="46" r="20" fill="#DA5A2A" opacity="0.2"/>
      <path d="M34 88c5-16 17-24 26-24s21 8 26 24" fill="#0F9F82" opacity="0.24"/>
      <circle cx="60" cy="44" r="14" fill="#DA5A2A" opacity="0.55"/>
    </svg>
  `);
const HASH_PREFIX = "#species/";
const detailCache = new Map();

const state = {
  index: null,
  filtered: [],
  selectedId: null,
};

const searchInput = document.querySelector("#search-input");
const elementFilter = document.querySelector("#element-filter");
const stageFilter = document.querySelector("#stage-filter");
const resultsSummary = document.querySelector("#results-summary");
const speciesGrid = document.querySelector("#species-grid");
const detailState = document.querySelector("#detail-state");
const detailCard = document.querySelector("#detail-card");
const speciesCardTemplate = document.querySelector("#species-card-template");

bootstrap().catch((error) => {
  console.error(error);
  resultsSummary.textContent = "图鉴数据加载失败。";
  detailState.textContent =
    "没有成功读取 generated/data，请先运行导出器生成静态资源。";
});

searchInput.addEventListener("input", updateFilters);
elementFilter.addEventListener("change", updateFilters);
stageFilter.addEventListener("change", updateFilters);
window.addEventListener("hashchange", syncSelectionFromHash);

async function bootstrap() {
  const response = await fetch(`${DATA_ROOT}/species-index.json`);
  if (!response.ok) {
    throw new Error(`Failed to fetch species index: ${response.status}`);
  }

  state.index = await response.json();
  populateSelect(elementFilter, "全部属性", state.index.elements);
  populateSelect(stageFilter, "全部阶段", state.index.stages);

  updateFilters();
  syncSelectionFromHash();
}

function populateSelect(element, allLabel, values) {
  element.innerHTML = "";
  element.append(new Option(allLabel, ""));
  values.forEach((value) => element.append(new Option(value, value)));
}

function updateFilters() {
  if (!state.index) {
    return;
  }

  const keyword = searchInput.value.trim().toLowerCase();
  const element = elementFilter.value;
  const stage = stageFilter.value;

  state.filtered = state.index.species.filter((entry) => {
    const matchesKeyword =
      keyword.length === 0 ||
      `${entry.name} ${entry.element} ${entry.ability} ${entry.spiritNo ?? ""}`
        .toLowerCase()
        .includes(keyword);
    const matchesElement = element.length === 0 || entry.element === element;
    const matchesStage = stage.length === 0 || entry.evoStage === stage;
    return matchesKeyword && matchesElement && matchesStage;
  });

  renderGrid();
  resultsSummary.textContent = `当前展示 ${state.filtered.length} / ${state.index.speciesCount} 只宠物`;

  if (!state.selectedId || !state.filtered.some((entry) => entry.id === state.selectedId)) {
    const fallback = state.filtered[0]?.id ?? null;
    setSelectedSpecies(fallback, false);
  }
}

function renderGrid() {
  speciesGrid.innerHTML = "";

  if (state.filtered.length === 0) {
    const empty = document.createElement("p");
    empty.className = "empty-state";
    empty.textContent = "没有找到符合当前筛选条件的宠物。";
    speciesGrid.append(empty);
    return;
  }

  const fragment = document.createDocumentFragment();

  state.filtered.forEach((entry) => {
    const node = speciesCardTemplate.content.firstElementChild.cloneNode(true);
    node.dataset.speciesId = entry.id;
    node.classList.toggle("is-active", entry.id === state.selectedId);
    node.addEventListener("click", () => setSelectedSpecies(entry.id, true));

    const image = node.querySelector("img");
    image.src = entry.icon ? `./generated/${entry.icon}` : EMPTY_ICON;
    image.alt = `${entry.name} 图标`;

    node.querySelector("h2").textContent = entry.name;
    node.querySelector(".species-card__id").textContent = `#${entry.pokemonId}`;
    node.querySelector(".species-card__meta").textContent = `${entry.element} / ${entry.evoStage}`;
    node.querySelector(".species-card__ability").textContent = entry.ability || "暂无特性描述";

    const statList = node.querySelector(".species-card__stats");
    statList.append(statItem("总值", entry.stats.total));
    statList.append(statItem("技能", entry.moveCount));
    statList.append(statItem("进化", entry.evolutionCount));

    fragment.append(node);
  });

  speciesGrid.append(fragment);
}

function statItem(label, value) {
  const wrapper = document.createElement("div");
  const term = document.createElement("dt");
  term.textContent = label;
  const desc = document.createElement("dd");
  desc.textContent = value;
  wrapper.append(term, desc);
  return wrapper;
}

function syncSelectionFromHash() {
  const hash = window.location.hash;
  if (!hash.startsWith(HASH_PREFIX)) {
    return;
  }

  const selectedId = decodeURIComponent(hash.slice(HASH_PREFIX.length));
  if (!state.index?.species.some((entry) => entry.id === selectedId)) {
    return;
  }

  setSelectedSpecies(selectedId, false);
}

async function setSelectedSpecies(speciesId, pushHash) {
  state.selectedId = speciesId;
  renderGrid();

  if (!speciesId) {
    detailCard.classList.add("hidden");
    detailState.hidden = false;
    detailState.textContent = "当前筛选结果为空，请调整搜索条件。";
    if (pushHash) {
      history.replaceState(null, "", window.location.pathname + window.location.search);
    }
    return;
  }

  if (pushHash) {
    window.location.hash = `${HASH_PREFIX}${encodeURIComponent(speciesId)}`;
  }

  detailState.hidden = false;
  detailState.textContent = "正在加载详情…";
  detailCard.classList.add("hidden");

  const detail = await fetchSpeciesDetail(speciesId);
  renderDetail(detail);
}

async function fetchSpeciesDetail(speciesId) {
  if (detailCache.has(speciesId)) {
    return detailCache.get(speciesId);
  }

  const response = await fetch(`${DATA_ROOT}/species/${speciesId}.json`);
  if (!response.ok) {
    throw new Error(`Failed to fetch species detail for ${speciesId}`);
  }

  const detail = await response.json();
  detailCache.set(speciesId, detail);
  return detail;
}

function renderDetail(detail) {
  detailCard.innerHTML = "";

  const hero = document.createElement("section");
  hero.className = "detail-hero";
  hero.innerHTML = `
    <div class="detail-hero__art">
      <img src="${detail.icon ? `./generated/${detail.icon}` : EMPTY_ICON}" alt="${detail.name} 图标" />
    </div>
    <div>
      <p class="eyebrow">Species Detail</p>
      <h2 class="detail-title">${detail.name}</h2>
      <p class="detail-meta">#${detail.pokemonId} / ${detail.element} / ${detail.evoStage}</p>
      <p class="detail-ability">${detail.ability || "暂无特性描述"}</p>
    </div>
  `;

  const tags = document.createElement("section");
  tags.className = "detail-section";
  tags.innerHTML = `<p class="section-kicker">Tags</p>`;
  const tagList = document.createElement("div");
  tagList.className = "chips";
  [detail.element, detail.evoStage, detail.spiritNo || "无灵编号"]
    .filter(Boolean)
    .forEach((label) => {
      const chip = document.createElement("span");
      chip.className = "chip";
      chip.textContent = label;
      tagList.append(chip);
    });
  tags.append(tagList);

  const stats = document.createElement("section");
  stats.className = "detail-section";
  stats.innerHTML = `<p class="section-kicker">Stats</p>`;
  const statBars = document.createElement("div");
  statBars.className = "stat-bars";
  [
    ["HP", detail.stats.maxHp],
    ["ATK", detail.stats.attack],
    ["DEF", detail.stats.defense],
    ["SPD", detail.stats.speed],
    ["SPA", detail.stats.specialAttack],
    ["SDEF", detail.stats.specialDefense],
  ].forEach(([label, value]) => statBars.append(renderStatBar(label, value)));
  stats.append(statBars);

  const evolutions = document.createElement("section");
  evolutions.className = "detail-section";
  evolutions.innerHTML = `<p class="section-kicker">Evolution</p>`;
  evolutions.append(renderEvolutionBlock("来自", detail.evolvesFrom));
  evolutions.append(renderEvolutionBlock("去往", detail.evolvesTo.map(formatEvolution)));

  const moves = document.createElement("section");
  moves.className = "detail-section";
  moves.innerHTML = `<p class="section-kicker">Moves</p>`;
  moves.append(renderMovesTable(detail.learnset));

  detailCard.append(hero, tags, stats, evolutions, moves);
  detailState.hidden = true;
  detailCard.classList.remove("hidden");
}

function renderStatBar(label, value) {
  const row = document.createElement("div");
  row.className = "stat-bar";
  const width = Math.max(6, Math.min(100, (value / 160) * 100));
  row.innerHTML = `
    <span class="stat-bar__label">${label}</span>
    <span class="stat-bar__value">${value}</span>
    <div class="stat-bar__track">
      <div class="stat-bar__fill" style="width:${width}%"></div>
    </div>
  `;
  return row;
}

function renderEvolutionBlock(title, entries) {
  const wrapper = document.createElement("div");
  wrapper.innerHTML = `<p class="detail-meta">${title}</p>`;

  if (!entries.length) {
    const empty = document.createElement("p");
    empty.className = "empty-state";
    empty.textContent = "没有记录。";
    wrapper.append(empty);
    return wrapper;
  }

  const chips = document.createElement("div");
  chips.className = "chips";
  entries.forEach((entry) => {
    const chip = document.createElement("span");
    chip.className = "chip";
    chip.textContent = entry;
    chips.append(chip);
  });
  wrapper.append(chips);
  return wrapper;
}

function formatEvolution(entry) {
  const conditions = [entry.evoLevel ? `Lv.${entry.evoLevel}` : "", entry.condition || ""]
    .filter(Boolean)
    .join(" / ");
  return conditions ? `${entry.toName} (${conditions})` : entry.toName;
}

function renderMovesTable(learnset) {
  if (!learnset.length) {
    const empty = document.createElement("p");
    empty.className = "empty-state";
    empty.textContent = "没有技能数据。";
    return empty;
  }

  const table = document.createElement("table");
  table.className = "moves-table";
  table.innerHTML = `
    <thead>
      <tr>
        <th>技能</th>
        <th>属性</th>
        <th>类型</th>
        <th>能量</th>
        <th>效果</th>
      </tr>
    </thead>
  `;

  const body = document.createElement("tbody");
  learnset.forEach((move) => {
    const row = document.createElement("tr");
    row.innerHTML = `
      <td>${move.name}</td>
      <td>${move.element}</td>
      <td>${move.category}</td>
      <td>${move.energyCost}</td>
      <td>${formatMoveEffect(move)}</td>
    `;
    body.append(row);
  });
  table.append(body);
  return table;
}

function formatMoveEffect(move) {
  const parts = [];
  if (move.effect.kind === "damage" && typeof move.effect.power === "number") {
    parts.push(`威力 ${move.effect.power}`);
  }
  if (move.description) {
    parts.push(move.description);
  }
  return parts.join(" · ") || "状态技能";
}
