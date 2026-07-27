use bevy::prelude::*;
use dwarven_engineering::core::{ItemId, RecipeId};
use dwarven_engineering::{
    CraftRequestEvent, Inventory, ItemCategory, ItemDatabase, PLAYER_INVENTORY_SLOTS,
    PlayerCharacter, RecipeDatabase, StationKind,
};

mod common;
use common::in_game_app;

fn item_id(id: &str) -> ItemId {
    ItemId(id.to_string())
}

fn recipe_id(id: &str) -> RecipeId {
    RecipeId(id.to_string())
}

/// Runs one `FixedUpdate` tick deterministically, independent of wall-clock time — see
/// `tests/input_test.rs` for why `app.update()` alone isn't reliable here.
fn run_one_fixed_tick(app: &mut App) {
    let timestep = app.world().resource::<Time<Fixed>>().timestep();
    app.world_mut()
        .resource_mut::<Time<Fixed>>()
        .advance_by(timestep);
    app.world_mut().run_schedule(FixedUpdate);
}

/// Adds `count` of `item` straight into the player's inventory, bypassing crafting — stands in
/// for a harvesting/pickup mechanic that doesn't exist yet.
fn give_player(app: &mut App, item: &ItemId, count: u32) {
    app.world_mut()
        .resource_scope(|world: &mut World, items: Mut<ItemDatabase>| {
            let mut query = world.query_filtered::<&mut Inventory, With<PlayerCharacter>>();
            let mut inventory = query.single_mut(world).expect("player has an inventory");
            assert!(
                inventory.add(&items, item, count),
                "test setup: inventory must have room"
            );
        });
}

#[test]
fn item_and_recipe_databases_load_from_ron() {
    let app = in_game_app();

    let items = app.world().resource::<ItemDatabase>();
    let wood = items.0.get(&item_id("wood")).expect("wood item defined");
    assert_eq!(wood.max_stack, 64);
    assert!(matches!(wood.category, ItemCategory::RawResource));

    let recipes = app.world().resource::<RecipeDatabase>();
    let axe = recipes
        .0
        .get(&recipe_id("axe"))
        .expect("axe recipe defined");
    assert!(matches!(axe.station, StationKind::HandCraft));
    assert_eq!(
        axe.inputs,
        vec![(item_id("wood"), 3), (item_id("stone"), 2)]
    );
    assert_eq!(axe.outputs, vec![(item_id("axe"), 1)]);
}

#[test]
fn player_spawns_with_an_empty_inventory() {
    let mut app = in_game_app();

    let mut query = app
        .world_mut()
        .query_filtered::<&Inventory, With<PlayerCharacter>>();
    let inventory = query.single(app.world()).expect("player has an inventory");

    assert_eq!(inventory.slots.len(), PLAYER_INVENTORY_SLOTS);
    assert!(inventory.slots.iter().all(Option::is_none));
}

#[test]
fn crafting_an_axe_consumes_inputs_and_produces_the_output() {
    let mut app = in_game_app();

    give_player(&mut app, &item_id("wood"), 3);
    give_player(&mut app, &item_id("stone"), 2);

    app.world_mut().write_message(CraftRequestEvent {
        recipe: recipe_id("axe"),
    });
    run_one_fixed_tick(&mut app);

    let mut query = app
        .world_mut()
        .query_filtered::<&Inventory, With<PlayerCharacter>>();
    let inventory = query.single(app.world()).expect("player has an inventory");

    assert_eq!(inventory.count(&item_id("axe")), 1, "axe was crafted");
    assert_eq!(inventory.count(&item_id("wood")), 0, "wood was consumed");
    assert_eq!(inventory.count(&item_id("stone")), 0, "stone was consumed");
}

#[test]
fn crafting_is_ignored_when_inventory_lacks_inputs() {
    let mut app = in_game_app();

    app.world_mut().write_message(CraftRequestEvent {
        recipe: recipe_id("axe"),
    });
    run_one_fixed_tick(&mut app);

    let mut query = app
        .world_mut()
        .query_filtered::<&Inventory, With<PlayerCharacter>>();
    let inventory = query.single(app.world()).expect("player has an inventory");

    assert_eq!(
        inventory.count(&item_id("axe")),
        0,
        "no materials, no craft"
    );
}
