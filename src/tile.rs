use crate::common::Collider;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Tile {}

pub fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    //rows
    for j in -2..11 {
        //colls
        for i in -6..7 {
            if i % 3 == 1 || i % 3 == -1 {
                continue;
            }
            let texture = asset_server.load("tile.png");
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 64.0, y: 16.0 }),
                        ..default()
                    },
                    texture,
                    transform: Transform {
                        translation: Vec3 {
                            x: ((4 + 64) * i) as f32,
                            y: ((4 + 16) * j) as f32,
                            z: 0.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                Tile {},
                Collider {},
                Name::new("Tile"),
            ));
        }
    }
}
