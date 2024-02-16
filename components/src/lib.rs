/// To make changes to the systems not break the type ids of components, making a components sub-crate is recommended. This way, they are a separate compilation unit. Otherwise component queries might suddenly be empty after code changes.
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Settings {
    #[inspector(min = 0.0, max = 10.0)]
    pub box_size: f32,
    #[inspector(min = 0.0, max = 10.0)]
    pub box_thickness: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            box_size: 2.0,
            box_thickness: 0.15,
        }
    }
}
