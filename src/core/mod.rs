//! `core` — shared identifier newtypes referenced across plugins, per
//! `docs/game-design/04-project-skeleton.md`.

use serde::Deserialize;

/// Identifies a row in `ItemDatabase`. Wraps the RON-authored string id (e.g. `"wood"`) rather
/// than an integer index, so `assets/data/*.ron` stays hand-editable without renumbering.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
#[serde(transparent)]
pub struct ItemId(pub String);

/// Identifies a row in `RecipeDatabase`. Same string-id rationale as `ItemId`.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
#[serde(transparent)]
pub struct RecipeId(pub String);

/// Identifies a row in `tech_tree::TechTree`, and gates `RecipeDef::requires_tech`.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
#[serde(transparent)]
pub struct TechNodeId(pub String);

/// Identifies a row in `factory_sim::FactoryDatabase`. Unlike the other ids on this page, this is
/// a closed enum, not a RON-authored string — factory *machine types* are a small, code-defined
/// set (per `03-ecs-design.md`'s `StructureKind::Factory(FactoryKind)`), not an open-ended
/// content catalog. Lives here (not in `factory_sim`) so `structures` and `tech_tree` can both
/// reference it without depending on `factory_sim` itself.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize)]
pub enum FactoryKind {
    Smelter,
}
