use bevy::prelude::*;

use crate::{cameras::MenuCamera, scenes, states::GameState, util::ui::{back_button, ui_node, BackToTitleSceneButtonTag}};

pub struct SingleplayerScenePlugin;

impl Plugin for SingleplayerScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::SingleplayerScene), (scenes::despawn, setup).chain())
            .add_systems(Update, (|| {})
                .run_if(in_state(GameState::SingleplayerScene)));
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
            "Singleplayer",
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