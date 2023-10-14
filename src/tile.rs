use crate::common::*;
use crate::powerup::*;
use bevy::prelude::*;
use rand::prelude::*;
#[derive(Component, Default)]
pub struct Tile {}

#[derive(Resource)]
pub struct TileCount(pub u32);

pub fn spawn_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tile_count: ResMut<TileCount>,
) {
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
                HealthComponent(2),
                Name::new("Tile"),
            ));
            tile_count.0 += 1;
        }
    }
}

pub fn tile_health(
    mut commands: Commands,
    mut query: Query<(Entity, &HealthComponent, &Transform), With<Tile>>,
    mut tile_count: ResMut<TileCount>,
) {
    for (ent, health, transform) in &mut query {
        if health.0 <= 0 {
            tile_count.0 -= 1;
            commands.entity(ent).despawn();
            let mut rng = rand::thread_rng();

            if rng.gen_range(0..=10) == 5 {
                commands.add(SpawnPowerup {
                    transform: *transform,
                });
            }
        }
    }
}
