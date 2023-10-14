use bevy::asset::*;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use crate::common::*;
use crate::common::{BallDestroyer, Collider};
use crate::paddle::Paddle;

use rand::prelude::*;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Ball {}

#[derive(Resource)]
pub struct BallCount(pub u32);

#[derive(Resource)]
pub struct BounceSound(pub Handle<AudioSource>);

pub struct SpawnBall {
    pub transform: Transform,
    pub moving: Option<MovingThing>,
}

impl Command for SpawnBall {
    fn apply(self, world: &mut World) {
        let texture = world
            .get_resource::<AssetServer>()
            .unwrap()
            .load("ball.png");
        world.get_resource_mut::<BallCount>().unwrap().0 += 1;
        let mut ball = world.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 16.0, y: 16.0 }),
                    ..default()
                },
                texture,
                transform: self.transform,
                ..default()
            },
            Ball {},
            //Collider,
            Name::new("Ball"),
        ));
        if let Some(moving) = self.moving {
            ball.insert(MovingThing {
                speed: moving.speed,
            });
        }

        dbg!(ball.id());
    }
}

pub fn spawn_ball(mut commands: Commands) {
    commands.add(SpawnBall {
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: -200.0,
                z: 0.0,
            },
            ..default()
        },
        moving: None,
    })
}

pub fn follow_paddle(
    paddles: Query<(&Transform, With<Paddle>)>,
    mut balls: Query<(
        &mut Transform,
        (With<Ball>, Without<Paddle>, Without<MovingThing>),
    )>,
) {
    let paddle = paddles.single().0;
    for (mut trans, _) in &mut balls {
        trans.translation.x = paddle.translation.x;
    }
}

pub fn launch_ball(
    mut commands: Commands,
    mut balls: Query<(Entity, (With<Ball>, Without<Paddle>, Without<MovingThing>))>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (ent, _) in &mut balls {
            commands.entity(ent).insert(MovingThing {
                speed: Vec2 { x: 100.0, y: 100.0 },
            });
        }
    }
}

pub fn ball_collisions(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut MovingThing, &Transform, &Sprite), With<Ball>>,
    mut collider_query: Query<
        (
            Entity,
            &Transform,
            &Sprite,
            Option<&BallDestroyer>,
            Option<&HealthComponent>,
            Option<&DamagedComponent>,
        ),
        With<Collider>,
    >,
    bounce_sound: Res<BounceSound>,
    mut ball_count: ResMut<BallCount>,
) {
    for (ball_ent, mut moving_ball, ball_transform, ball_sprite) in &mut ball_query {
        for (ent, transform, sprite, ball_destroyer_option, health_option, damaged_option) in
            &mut collider_query
        {
            if ent.index() == ball_ent.index() {
                continue;
            }
            let ball_scale = match ball_sprite.custom_size {
                Some(vec) => vec,
                _ => ball_transform.scale.truncate(),
            };
            let scale = match sprite.custom_size {
                Some(vec) => vec,
                _ => transform.scale.truncate(),
            };
            let collision = collide(
                ball_transform.translation,
                ball_scale,
                transform.translation,
                scale,
            );

            if let Some(collision) = collision {
                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = moving_ball.speed.x > 0.0,
                    Collision::Right => reflect_x = moving_ball.speed.x < 0.0,
                    Collision::Top => reflect_y = moving_ball.speed.y < 0.0,
                    Collision::Bottom => reflect_y = moving_ball.speed.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    moving_ball.speed.x = -moving_ball.speed.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    moving_ball.speed.y = -moving_ball.speed.y;
                }
                let mut rng = rand::thread_rng();

                commands.spawn(AudioBundle {
                    source: bounce_sound.0.clone(),
                    settings: PlaybackSettings {
                        speed: 1. + rng.gen_range(0.0..2.0),
                        ..default()
                    },
                });

                if health_option.is_some() && damaged_option.is_none() {
                    commands.entity(ent).insert(DamagedComponent::default());
                }

                //TODO: Can probably be optimized
                if ball_destroyer_option.is_some() {
                    ball_count.0 -= 1;
                    commands.entity(ball_ent).despawn();
                }
            }
        }
    }
}
