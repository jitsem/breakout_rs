use bevy::prelude::*;

use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Paddle {
    #[inspector(min = 0.0)]
    pub speed: f32,
}

