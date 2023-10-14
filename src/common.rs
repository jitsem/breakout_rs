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

#[derive(Component)]
pub struct BallDestroyer;

#[derive(Component, Debug)]
pub struct HealthComponent(pub i32);

#[derive(Component, Debug)]
pub struct DamagedComponent {
    timer: Timer,
}

impl Default for DamagedComponent {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

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

pub fn do_damage(
    mut commands: Commands,
    mut query: Query<(Entity, &mut HealthComponent, &mut DamagedComponent)>,
    time: Res<Time>,
) {
    for (ent, mut health, mut damage) in &mut query {
        damage.timer.tick(time.delta());
        if !damage.timer.finished() {
            continue;
        }
        health.0 -= 1;
        commands.entity(ent).remove::<DamagedComponent>();
    }
}
