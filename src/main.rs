use bevy::prelude::*;
use dwarven_engineering::GamePlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .run();
}
