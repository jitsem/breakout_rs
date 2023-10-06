use bevy::core::Name;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::components::{Paddle};

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
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
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
            ..default()
        },
        Paddle { speed: 100.0 },
        Name::new("Paddle"),
    ));
}

fn paddle_movement(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut paddles {
        let movement_amount = player.speed * time.delta_seconds();
        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount
        }
    }
}