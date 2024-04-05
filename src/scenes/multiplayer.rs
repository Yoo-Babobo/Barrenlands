use bevy::prelude::*;

use crate::{cameras::MenuCamera, scenes, states::GameState, util::ui::{back_button, ui_node, BackToTitleSceneButtonTag}};

pub struct MultiplayerScenePlugin;

impl Plugin for MultiplayerScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MultiplayerScene), (scenes::despawn, setup).chain())
            .add_systems(Update, (|| {})
                .run_if(in_state(GameState::MultiplayerScene)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MenuCamera
    ));

    commands.spawn(TextBundle {
        style: Style {
            justify_self: JustifySelf::Center,
            ..default()
        },
        text: Text::from_section(
            "Multiplayer",
            TextStyle {
                font_size: 50.,
                color: Color::GOLD,
                ..default()
            }
        ),
        ..default()
    });

    commands
        .spawn(ui_node())
            .with_children(|builder| {
                back_button(builder, BackToTitleSceneButtonTag);
            });
}