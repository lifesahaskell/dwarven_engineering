use std::collections::VecDeque;

use bevy::prelude::*;
use dwarven_engineering::core::{FactoryKind, ItemId};
use dwarven_engineering::{
    BeltSegment, Direction, FactoryDatabase, FactoryState, Inventory, ItemDatabase,
    PlaceStructureRequest, SimulationDetailLevel, StructureKind,
};

mod common;
use common::{in_game_app, run_one_fixed_tick};

fn item_id(id: &str) -> ItemId {
    ItemId(id.to_string())
}

fn give_factory(app: &mut App, entity: Entity, item: &ItemId, count: u32) {
    app.world_mut()
        .resource_scope(|world: &mut World, items: Mut<ItemDatabase>| {
            let mut inventory = world
                .get_mut::<Inventory>(entity)
                .expect("factory has an inventory");
            assert!(
                inventory.add(&items, item, count),
                "test setup: factory inventory must have room"
            );
        });
}

fn place_factory(app: &mut App, position: IVec2) {
    app.world_mut().write_message(PlaceStructureRequest {
        kind: StructureKind::Factory(FactoryKind::Smelter),
        position,
    });
    // `FactorySimSet` runs before `StructureSet` each tick (docs/game-design/03-ecs-design.md),
    // so placement (this tick) and `attach_factory_state` noticing it (next tick) are two ticks.
    run_one_fixed_tick(app);
    run_one_fixed_tick(app);
}

fn factory_entity(app: &mut App) -> Entity {
    let mut query = app
        .world_mut()
        .query_filtered::<Entity, With<FactoryState>>();
    query.single(app.world()).expect("exactly one factory")
}

#[test]
fn factory_database_loads_from_ron() {
    let app = in_game_app();

    let factories = app.world().resource::<FactoryDatabase>();
    let smelter = factories
        .0
        .get(&FactoryKind::Smelter)
        .expect("smelter factory defined");
    assert_eq!(smelter.recipe_slots, 1);
}

#[test]
fn placing_a_factory_structure_consumes_materials_and_attaches_factory_state() {
    let mut app = in_game_app();

    give_factory_build_materials(&mut app);
    place_factory(&mut app, IVec2::new(3, 3));

    let mut query = app
        .world_mut()
        .query_filtered::<(&StructureKind, &FactoryState, &Inventory), With<FactoryState>>();
    let (kind, state, inventory) = query
        .single(app.world())
        .expect("factory has FactoryState and an onboard Inventory");

    assert!(matches!(kind, StructureKind::Factory(FactoryKind::Smelter)));
    assert!(state.active_recipe.is_none());
    assert!(inventory.slots.iter().all(Option::is_none));
}

/// Structure placement needs materials from the *player's* inventory, so give the player enough
/// stone/ore to afford a Smelter's build cost (see assets/data/structures.ron) before placing one.
fn give_factory_build_materials(app: &mut App) {
    use dwarven_engineering::PlayerCharacter;

    app.world_mut()
        .resource_scope(|world: &mut World, items: Mut<ItemDatabase>| {
            let mut query = world.query_filtered::<&mut Inventory, With<PlayerCharacter>>();
            let mut inventory = query.single_mut(world).expect("player has an inventory");
            assert!(inventory.add(&items, &item_id("stone"), 15));
            assert!(inventory.add(&items, &item_id("ore"), 5));
        });
}

#[test]
fn factory_crafts_its_recipe_over_time_once_it_has_ore() {
    let mut app = in_game_app();

    give_factory_build_materials(&mut app);
    place_factory(&mut app, IVec2::new(3, 3));

    let factory = factory_entity(&mut app);
    give_factory(&mut app, factory, &item_id("ore"), 1);

    // ingot's craft_time_secs is 2.0 (assets/data/recipes.ron); run well past that at the default
    // fixed timestep.
    for _ in 0..200 {
        run_one_fixed_tick(&mut app);
    }

    let inventory = app
        .world()
        .get::<Inventory>(factory)
        .expect("factory has an inventory");
    assert_eq!(inventory.count(&item_id("ingot")), 1, "ingot was crafted");
    assert_eq!(inventory.count(&item_id("ore")), 0, "ore was consumed");
}

#[test]
fn a_factory_far_from_the_player_is_simulated_at_reduced_detail() {
    let mut app = in_game_app();

    give_factory_build_materials(&mut app);
    place_factory(&mut app, IVec2::new(100, 100));

    let factory = factory_entity(&mut app);
    let detail = app
        .world()
        .get::<SimulationDetailLevel>(factory)
        .expect("factory has a detail level");
    assert_eq!(
        *detail,
        SimulationDetailLevel::Paused,
        "far outside FACTORY_PAUSED_RADIUS of the origin-spawned player"
    );
}

#[test]
fn a_factory_near_the_player_is_simulated_at_full_detail() {
    let mut app = in_game_app();

    give_factory_build_materials(&mut app);
    place_factory(&mut app, IVec2::new(1, 1));

    let factory = factory_entity(&mut app);
    let detail = app
        .world()
        .get::<SimulationDetailLevel>(factory)
        .expect("factory has a detail level");
    assert_eq!(*detail, SimulationDetailLevel::Full);
}

#[test]
fn placing_a_factory_without_materials_spawns_nothing() {
    let mut app = in_game_app();

    place_factory(&mut app, IVec2::new(3, 3));

    let mut factories = app
        .world_mut()
        .query_filtered::<Entity, With<FactoryState>>();
    assert_eq!(
        factories.iter(app.world()).count(),
        0,
        "no materials, no build"
    );
}

#[test]
fn belt_segment_advances_item_progress_and_caps_at_full() {
    let mut app = in_game_app();

    let belt = app
        .world_mut()
        .spawn(BeltSegment {
            direction: Direction::East,
            speed: 1.0,
            items: VecDeque::from([(item_id("ore"), 0.0), (item_id("ore"), 0.999)]),
        })
        .id();

    let timestep = app
        .world()
        .resource::<Time<Fixed>>()
        .timestep()
        .as_secs_f32();
    run_one_fixed_tick(&mut app);

    let belt = app.world().get::<BeltSegment>(belt).expect("belt exists");
    assert_eq!(belt.items[0].1, timestep, "advances by speed * dt");
    assert_eq!(belt.items[1].1, 1.0, "caps at 1.0 instead of overshooting");
}
