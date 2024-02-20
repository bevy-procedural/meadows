use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use procedural_grass::*;
use std::env;

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // or "full"

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .register_type::<Settings>()
        .insert_resource(Settings::default())
        .add_plugins(EditorPlugin::on_second_monitor_fullscreen(
            EditorPlugin::default(),
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup_grass)
        .add_systems(Update, update_grass)
        .run();
}
