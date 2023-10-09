use crate::common::{Collider, PowerupPickupArea};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Paddle {
    #[inspector(min = 0.0)]
    pub speed: f32,
    #[inspector(min = 0.0)]
    pub acceleration: f32,
    #[inspector(min = 0.0)]
    pub deceleration: f32,
    #[inspector(min = 0.0)]
    pub max_speed: f32,
}

pub fn spawn_paddle(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("paddle.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 64.0, y: 16.0 }),
                ..default()
            },
            texture,
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: -230.0,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        },
        Paddle {
            speed: 0.0,
            acceleration: 250.0,
            deceleration: 200.0,
            max_speed: 200.0,
        },
        Collider,
        PowerupPickupArea::PowerupConsumer,
        Name::new("Paddle"),
    ));
}

pub fn paddle_movement(
    mut paddles: Query<(&mut Transform, &mut Paddle, &Sprite)>,
    collider_query: Query<(&Transform, &Sprite), (With<Collider>, Without<Paddle>)>,
    input: Res<Input<KeyCode>>,
    time: Res<FixedTime>,
) {
    for (mut transform, mut paddle, sprite) in &mut paddles {
        let input_accel = match (
            input.pressed(KeyCode::A) || input.pressed(KeyCode::Left),
            input.pressed(KeyCode::D) || input.pressed(KeyCode::Right),
        ) {
            (true, false) => -paddle.acceleration,
            (false, true) => paddle.acceleration,
            _ => 0.0,
        };
        let accel = if input_accel != 0.0 {
            input_accel
        } else if paddle.speed.abs() > f32::EPSILON {
            -paddle.speed.signum() * paddle.deceleration
        } else {
            0.0
        };

        paddle.speed = (paddle.speed + accel * time.period.as_secs_f32())
            .clamp(-paddle.max_speed, paddle.max_speed);

        for (coll_transform, coll_sprite) in &collider_query {
            let paddle_scale = match sprite.custom_size {
                Some(vec) => vec,
                _ => transform.scale.truncate(),
            };
            let coll_scale = match coll_sprite.custom_size {
                Some(vec) => vec,
                _ => coll_transform.scale.truncate(),
            };
            let collision = collide(
                transform.translation,
                paddle_scale,
                coll_transform.translation,
                coll_scale,
            );

            if let Some(collision) = collision {
                match (collision, paddle.speed.is_sign_positive()) {
                    (Collision::Left, true) => paddle.speed = 0.,
                    (Collision::Right, false) => paddle.speed = 0.,
                    _ => (),
                }
            }
        }
        transform.translation.x += paddle.speed * time.period.as_secs_f32();
    }
}
