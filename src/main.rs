use bevy::core::Name;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::components::Paddle;

mod components;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Breakout".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Grave)),
        )
        .register_type::<Paddle>()
        .add_systems(Startup, setup)
        .add_systems(Update, paddle_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::TEAL),
        },
        ..default()
    };
    camera.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: 640.0,
        max_height: 480.0,
    };
    commands.spawn(camera);

    let texture = asset_server.load("paddle.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(40.0, 10.0)),
                ..default()
            },
            texture,
            transform: Transform {
                translation: Vec3 {
                    x:0.0,
                    y:-230.0,
                    z:0.0
                },
                ..default()
            },
            ..default()
        },
        Paddle {
            speed: 0.0,
            acceleration: 250.0,
            deceleration: 100.0,
            max_speed: 200.0
        },
        Name::new("Paddle"),
    ));
}

fn paddle_movement(
    mut paddles: Query<(&mut Transform, &mut Paddle)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut paddle) in &mut paddles {
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

        paddle.speed = (paddle.speed + accel * time.delta_seconds()).clamp(-paddle.max_speed, paddle.max_speed);
        transform.translation.x += paddle.speed * time.delta_seconds();
    }
}
