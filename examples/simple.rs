use bevy::prelude::*;
use procedural_meadows::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Settings::default())
        .add_systems(Startup, setup_meadows)
        .add_systems(Update, update_meadows)
        .run();
}
