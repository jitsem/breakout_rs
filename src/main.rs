use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;

use ball::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use common::*;
use paddle::*;
use powerup::*;
use tile::*;
use ui::*;
use wall::spawn_walls;

mod ball;
mod common;
mod paddle;
mod powerup;
mod tile;
mod ui;
mod wall;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Breakout".into(),
                    resolution: (1200.0, 720.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build(),
    );
    /* .add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Grave)),
    ) */

    let bounce_source = app
        .world
        .get_resource::<AssetServer>()
        .unwrap()
        .load("impact.ogg");
    app.add_systems(
        Startup,
        (
            setup,
            spawn_game_ui,
            spawn_ball,
            spawn_paddle,
            spawn_tiles,
            spawn_walls,
        ),
    )
    .add_systems(
        FixedUpdate,
        (
            paddle_movement.before(ball_collisions),
            follow_paddle.after(paddle_movement),
            thing_movement.before(ball_collisions),
            ball_collisions,
            powerup_pickup,
            update_balls_ui,
            update_tiles_ui,
        ),
    )
    .add_systems(
        Update,
        (
            paddle_movement,
            follow_paddle.after(paddle_movement),
            launch_ball,
            thing_movement,
            ball_collisions,
            tile_health,
            do_damage,
        ),
    )
    .register_type::<Ball>()
    .register_type::<Paddle>()
    .insert_resource(BallCount(0))
    .insert_resource(TileCount(0))
    .insert_resource(BounceSound(bounce_source))
    .run();
}

fn setup(mut commands: Commands) {
    let camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::TEAL),
        },
        ..default()
    };
    commands.spawn(camera);
}
