use crate::ball::{Ball, SpawnBall};
use crate::common::{MovingThing, PowerupPickupArea};
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

#[derive(Component, Default)]
pub struct Powerup;

pub struct SpawnPowerup {
    pub transform: Transform,
}

impl Command for SpawnPowerup {
    fn apply(self, world: &mut World) {
        let texture = world
            .get_resource::<AssetServer>()
            .unwrap()
            .load("powerup.png");
        let powerup = world.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 12.0, y: 12.0 }),
                    ..default()
                },
                texture,
                transform: self.transform,
                ..default()
            },
            MovingThing {
                speed: Vec2 { x: 0., y: -50. },
            },
            Powerup,
            Name::new("Powerup"),
        ));

        dbg!(powerup.id());
    }
}

pub fn powerup_pickup(
    mut commands: Commands,
    mut powerup_query: Query<(Entity, &Transform, &Sprite), With<Powerup>>,
    collider_query: Query<(&Transform, &Sprite, &PowerupPickupArea)>,
    balls: Query<(&Transform, &MovingThing), With<Ball>>,
) {
    for (powerup_ent, power_transform, power_sprite) in &mut powerup_query {
        for (transform, sprite, pickup) in &collider_query {
            let power_scale = match power_sprite.custom_size {
                Some(vec) => vec,
                _ => power_transform.scale.truncate(),
            };
            let scale = match sprite.custom_size {
                Some(vec) => vec,
                _ => transform.scale.truncate(),
            };
            let collision = collide(
                power_transform.translation,
                power_scale,
                transform.translation,
                scale,
            );
            if collision.is_some() {
                if let PowerupPickupArea::PowerupConsumer = pickup {
                    info!("Would spawn power");

                    for (loops, (trans, moving)) in (&balls).into_iter().enumerate() {
                        if loops > 100 {
                            break;
                        }
                        let spawn = SpawnBall {
                            transform: Transform {
                                translation: trans.translation,
                                ..default()
                            },
                            moving: Some(MovingThing {
                                speed: Vec2 {
                                    x: -moving.speed.x,
                                    y: -moving.speed.y,
                                },
                            }),
                        };
                        commands.add(spawn);
                    }
                }
                commands.entity(powerup_ent).despawn();
            }
        }
    }
}
