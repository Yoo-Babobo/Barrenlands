use bevy::prelude::*;

use crate::cameras::MenuCamera;

pub struct TitleScene;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle::default(),
        MenuCamera
    ));

    commands.spawn(TextBundle {
        text: Text::from_section(
            "Barrenlands",
            TextStyle {
                font_size: 100.,
                color: Color::GOLD,
                ..default()
            }
        ),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("logo.png"),
        ..default()
    });
}