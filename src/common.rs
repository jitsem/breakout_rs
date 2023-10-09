use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component)]
pub enum PowerupPickupArea {
    PowerupConsumer,
    PowerupDestroyer,
}

#[derive(Component)]
pub struct Collider;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct MovingThing {
    pub speed: Vec2,
}

pub fn thing_movement(mut things: Query<(&mut Transform, &MovingThing)>, time: Res<FixedTime>) {
    for (mut trans, thing) in &mut things {
        trans.translation.x += thing.speed.x * time.period.as_secs_f32();
        trans.translation.y += thing.speed.y * time.period.as_secs_f32();
    }
}
