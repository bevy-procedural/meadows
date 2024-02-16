use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use components::Settings;
use std::env;

#[cfg(feature = "hot_reload")]
#[hot_lib_reloader::hot_module(dylib = "bevy_procedural_grass", file_watch_debounce = 200)]
mod bevy_procedural_grass_hot {
    use bevy::prelude::*;
    pub use components::*;
    hot_functions_from_file!("src/lib.rs");

    #[lib_updated]
    pub fn was_updated() -> bool {}
}

#[cfg(not(feature = "hot_reload"))]
use bevy_procedural_grass::*;
#[cfg(feature = "hot_reload")]
use bevy_procedural_grass_hot::*;

fn reload_after_change() {
    #[cfg(feature = "hot_reload")]
    if bevy_procedural_grass_hot::was_updated() {
        println!("Reloading systems");
    }
}

fn main() {
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
        .add_systems(Startup, setup_grass)
        .add_systems(Update, reload_after_change)
        .add_systems(Update, update_grass)
        .run();
}
