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

/// Identifies a row in the future `TechTree` (M4). Exists now only so `RecipeDef::requires_tech`
/// has a type to hold — no `TechTree` resource reads it yet.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
#[serde(transparent)]
pub struct TechNodeId(pub String);
