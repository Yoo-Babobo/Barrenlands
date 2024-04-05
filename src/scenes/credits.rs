use bevy::prelude::*;

use crate::{cameras::MenuCamera, scenes, states::GameState, util::ui::{back_button, ui_node, BackToTitleSceneButtonTag}};

pub struct CreditsScenePlugin;

impl Plugin for CreditsScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::CreditsScene), (scenes::despawn, setup).chain())
            .add_systems(Update, (|| {})
                .run_if(in_state(GameState::CreditsScene)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MenuCamera
    ));

    commands
        .spawn(ui_node())
            .with_children(|builder| {
                builder.spawn(TextBundle {
                    style: Style {
                        justify_self: JustifySelf::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "Credits",
                        TextStyle {
                            font_size: 50.,
                            color: Color::GOLD,
                            ..default()
                        }
                    ),
                    ..default()
                });
                builder.spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
                        TextStyle {
                            font_size: 20.,
                            color: Color::GOLD,
                            ..default()
                        }
                    ),
                    ..default()
                });
                
                back_button(builder, BackToTitleSceneButtonTag);
            });
}