use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::ball::BallCount;
use crate::tile::TileCount;

#[derive(Component)]
pub struct BallText;

#[derive(Component)]
pub struct TileText;

pub fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(15.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    padding: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    display: Display::Flex,
                    ..default()
                },
                background_color: Color::Rgba {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                    alpha: 0.,
                }
                .into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Balls!",
                        TextStyle {
                            font_size: 24.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                BallText,
            ));
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Tiles!",
                        TextStyle {
                            font_size: 24.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                TileText,
            ));
        });
}

pub fn update_balls_ui(mut texts: Query<&mut Text, With<BallText>>, balls: Res<BallCount>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Balls: {:?}", balls.0);
    }
}

pub fn update_tiles_ui(mut texts: Query<&mut Text, With<TileText>>, tiles: Res<TileCount>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Tiles: {:?}", tiles.0);
    }
}
