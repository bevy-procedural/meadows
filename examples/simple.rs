use bevy::prelude::*;
use procedural_grass::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Settings::default())
        .add_systems(Startup, setup_grass)
        .add_systems(Update, update_grass)
        .run();
}
