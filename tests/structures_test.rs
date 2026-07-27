use bevy::prelude::*;
use dwarven_engineering::core::ItemId;
use dwarven_engineering::{
    Inventory, ItemDatabase, PlaceStructureRequest, PlayerCharacter, Structure, StructureDatabase,
    StructureKind, StructurePosition,
};

mod common;
use common::{in_game_app, run_one_fixed_tick};

fn item_id(id: &str) -> ItemId {
    ItemId(id.to_string())
}

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

fn placed_structures(app: &mut App) -> Vec<(StructureKind, IVec2)> {
    let mut query = app
        .world_mut()
        .query_filtered::<(&StructureKind, &StructurePosition), With<Structure>>();
    query
        .iter(app.world())
        .map(|(kind, pos)| (*kind, pos.0))
        .collect()
}

#[test]
fn structure_database_loads_from_ron() {
    let app = in_game_app();

    let structures = app.world().resource::<StructureDatabase>();
    let storage = structures
        .0
        .get(&StructureKind::Storage)
        .expect("storage structure defined");
    assert_eq!(storage.footprint, IVec2::new(1, 1));
    assert_eq!(storage.build_cost, vec![(item_id("wood"), 5)]);
}

#[test]
fn placing_a_structure_consumes_materials_and_spawns_it() {
    let mut app = in_game_app();

    give_player(&mut app, &item_id("wood"), 5);

    app.world_mut().write_message(PlaceStructureRequest {
        kind: StructureKind::Storage,
        position: IVec2::new(3, 3),
    });
    run_one_fixed_tick(&mut app);

    assert_eq!(
        placed_structures(&mut app),
        vec![(StructureKind::Storage, IVec2::new(3, 3))]
    );

    let mut query = app
        .world_mut()
        .query_filtered::<&Inventory, With<PlayerCharacter>>();
    let inventory = query.single(app.world()).expect("player has an inventory");
    assert_eq!(inventory.count(&item_id("wood")), 0, "wood was consumed");
}

#[test]
fn placement_is_ignored_when_inventory_lacks_materials() {
    let mut app = in_game_app();

    app.world_mut().write_message(PlaceStructureRequest {
        kind: StructureKind::Storage,
        position: IVec2::new(3, 3),
    });
    run_one_fixed_tick(&mut app);

    assert_eq!(
        placed_structures(&mut app),
        vec![],
        "no materials, no build"
    );
}

#[test]
fn placement_is_ignored_when_footprint_overlaps_an_existing_structure() {
    let mut app = in_game_app();

    give_player(&mut app, &item_id("wood"), 10);

    app.world_mut().write_message(PlaceStructureRequest {
        kind: StructureKind::Storage,
        position: IVec2::new(0, 0),
    });
    run_one_fixed_tick(&mut app);

    app.world_mut().write_message(PlaceStructureRequest {
        kind: StructureKind::Storage,
        position: IVec2::new(0, 0),
    });
    run_one_fixed_tick(&mut app);

    assert_eq!(
        placed_structures(&mut app),
        vec![(StructureKind::Storage, IVec2::new(0, 0))],
        "second placement overlaps the first and must be rejected"
    );
    let mut query = app
        .world_mut()
        .query_filtered::<&Inventory, With<PlayerCharacter>>();
    let inventory = query.single(app.world()).expect("player has an inventory");
    assert_eq!(
        inventory.count(&item_id("wood")),
        5,
        "materials for the rejected placement must not be spent"
    );
}
