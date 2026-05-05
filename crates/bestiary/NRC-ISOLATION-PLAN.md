# `bestiary` 包内继续下沉 NRC 实现方案

## 目标

当前 `bestiary` 已经是独立 crate，但 NRC 相关实现仍直接暴露在核心入口中：

- `lib.rs` 直接导出 `NrcRepository`
- `lib.rs` 直接导出 `bundled_nrc_bundle_dir`
- `PetCatalog::from_nrc_bundle(...)` 直接内建 NRC 装配入口

这说明 `bestiary` 虽然已经和 `crystalline` 解耦，但还没有做到“核心图鉴能力”和“NRC 数据源实现”之间的清晰内部分层。

这份方案的目标不是拆出 `bestiary-nrc` 新包，而是在 **同一个 `bestiary` crate 内** 继续下沉 NRC 实现，使得：

1. `bestiary` 对外仍然只是一个包
2. 核心图鉴模型和查询接口不被 NRC 细节污染
3. NRC 成为 `bestiary` 内部的可替换实现，而不是默认长在根 API 上
4. 后续如果接入别的数据源，不需要先拆新包才能演进

## 当前问题

### 1. 根 API 暴露了实现细节

当前 `bestiary` 根入口同时暴露了：

- 图鉴模型：`PetSpecies`、`Move`、`Evolution`
- 图鉴接口：`PetRepository`、`PetCatalogService`
- NRC 实现：`NrcRepository`
- NRC bundle 路径约定：`bundled_nrc_bundle_dir`

这样会让使用者默认把 “`bestiary` = NRC bundle + SQLite” 视为同一件事。

### 2. `PetCatalog` 同时承担了模型容器和具体加载入口

`PetCatalog` 现在既是内存目录结构，又包含：

- `from_nrc_bundle(...)`
- `from_connection(...)`

这使核心模型类型认识了 NRC 的加载路径、SQLite 连接和图标目录约定。

### 3. 替换数据源的扩展点还不够清晰

虽然 `PetRepository` trait 已存在，但实际默认用法仍强烈指向 NRC：

- 直接构造 `PetCatalog::from_nrc_bundle(...)`
- 直接使用 `bundled_nrc_bundle_dir()`

这会让未来接入 JSON、HTTP、远程服务或别的 bundle 结构时，继续把实现细节推回根 API。

## 设计原则

这次下沉遵循四条原则：

1. 不新增 `bestiary-nrc` crate
2. `bestiary` 根 API 优先暴露抽象能力，不优先暴露具体实现
3. NRC 相关代码可以保留在 crate 内，但应该进入更明确的实现层命名空间
4. demo/app 如果需要 NRC，应该显式选择 NRC 入口，而不是从核心模型“顺手拿到”

## 建议的 crate 内结构

推荐把 `bestiary` 内部分成两层：

```text
crates/bestiary/src/
  lib.rs
  catalog.rs
  error.rs
  model.rs
  query.rs
  repository.rs

  sources/
    mod.rs
    nrc/
      mod.rs
      bundle.rs
      loader.rs
```

其中：

- `catalog.rs / model.rs / query.rs / repository.rs` 是核心层
- `sources/nrc/*` 是 NRC 实现层

## 对外 API 调整建议

### 保留在根入口的内容

这些仍然应该从 `bestiary` 根导出：

- `PetCatalog`
- `PetSpecies`
- `Move`
- `Evolution`
- `Stats`
- `BestiaryError`
- `PetRepository`
- `PetCatalogService`
- `PetQueryService`

这些都是 “图鉴是什么、图鉴能做什么”。

### 从根入口移走的内容

这些不建议继续放在根入口：

- `NrcRepository`
- `bundled_nrc_bundle_dir`
- `PetCatalog::from_nrc_bundle(...)`
- `PetCatalog::from_connection(...)`

原因很简单：它们描述的是 “NRC 怎么加载”，不是 “图鉴是什么”。

### 替代入口

如果仍希望在同一个 crate 内提供 NRC 能力，建议改成显式命名空间：

```rust
use bestiary::sources::nrc::{NrcCatalogLoader, bundled_bundle_dir};
```

或者：

```rust
let catalog = bestiary::sources::nrc::load_catalog(bundle_dir)?;
```

这会让调用方明确知道：自己现在选的是 NRC 实现，而不是在使用 `bestiary` 的纯核心 API。

## 类型职责调整

### `PetCatalog`

`PetCatalog` 应只负责：

- 保存已加载好的 `species`
- 提供索引
- 作为内存仓储实现 `PetRepository`
- 提供查询入口

它不应再负责：

- 打开 SQLite
- 猜 bundle 目录
- 扫描 icons 目录
- 处理 NRC schema

### `NrcRepository` 或 `NrcCatalogLoader`

NRC 实现层负责：

- bundle 根目录约定
- SQLite 连接建立
- SQL 查询
- icons 路径索引
- NRC 行数据到 `PetSpecies` 的装配

这部分完全可以留在 `bestiary` crate 内，但不应再伪装成核心模型的一部分。

## 推荐迁移步骤

### 第一步：移动模块位置，不改行为

- 将 `nrc.rs` 拆到 `sources/nrc/`
- 将 `bundle.rs` 合并或移动到 `sources/nrc/`
- 保持现有加载逻辑不变

验收：

- 行为与测试不变
- 代码位置上已经出现“核心层 / 实现层”分界

### 第二步：移除根导出

- `lib.rs` 不再 `pub use NrcRepository`
- `lib.rs` 不再 `pub use bundled_nrc_bundle_dir`
- 改为从 `sources::nrc` 命名空间访问

验收：

- 调用方需要显式写出 `sources::nrc`
- 根 API 不再暗示 NRC 是默认实现

### 第三步：移除 `PetCatalog` 上的 NRC 构造函数

- 删除 `PetCatalog::from_nrc_bundle(...)`
- 删除 `PetCatalog::from_connection(...)`
- 将这些构造动作改为 NRC loader 的职责

验收：

- `PetCatalog` 只保留内存目录职责
- `PetCatalog` 类型不再认识 SQLite / bundle / icons

### 第四步：让 app/demo 显式装配 NRC

例如在 `battle-demo` 中改为：

```rust
use bestiary::sources::nrc::{NrcCatalogLoader, bundled_bundle_dir};
```

由 demo 选择 NRC，而不是由 `PetCatalog` 默认内嵌 NRC。

验收：

- `battle-demo` 明确依赖 NRC 实现层
- 如果以后换数据源，改 demo 装配层即可

## 为什么不拆成 `bestiary-nrc`

当前阶段不拆新包是合理的，原因有三点：

1. 现在的主要矛盾是 API 分层不清，不是包数量不够
2. 先在同一个 crate 内拉开“核心层 / 实现层”边界，改动风险更低
3. 未来如果确实需要单独版本化 NRC，再从清晰的内部实现层继续外提，会更平滑

换句话说，这一步要先解决“边界认知问题”，再决定是否解决“发布单元问题”。

## 完成后的理想状态

当这一步完成后，`bestiary` 应呈现为：

- 根入口只代表图鉴能力
- NRC 是 crate 内一个显式可选的数据源实现
- `PetCatalog` 不再承担具体加载逻辑
- app/demo 显式选择数据源实现

这样虽然仍然只有一个 `bestiary` 包，但它的内部边界会清楚很多，也更符合“核心稳定、实现可替换”的目标。
