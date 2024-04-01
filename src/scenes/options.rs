use bevy::prelude::*;

use crate::{cameras::MenuCamera, scenes, states::{GameState, OptionsState}, util::ui::{back_button, ui_node, BackToTitleSceneButtonTag}};

pub struct OptionsScenePlugin;

impl Plugin for OptionsScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::OptionsScene(OptionsState::Main)), (scenes::despawn, setup).chain())
            .add_systems(Update, (interaction)
                .run_if(in_state(GameState::OptionsScene(OptionsState::Main))
                    .or_else(in_state(GameState::OptionsScene(OptionsState::Controls)))
                    .or_else(in_state(GameState::OptionsScene(OptionsState::Sound)))));
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
            "Options",
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

fn interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            
        }
    }
}