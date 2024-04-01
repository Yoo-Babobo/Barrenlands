use bevy::prelude::*;

use crate::{cameras::MenuCamera, scenes, states::{GameState, OptionsState}, util::{mods::Mods, ui::{basic_button, BasicButton, ButtonId, ButtonStyle, TitleSceneButtonId, TitleSceneButtonTag}}};

pub struct TitleScenePlugin;

impl Plugin for TitleScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::TitleScene), (scenes::despawn, setup).chain())
            .add_systems(Update, (interaction)
                .run_if(in_state(GameState::TitleScene)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mods: Res<Mods>) {
    println!("{:#?}", mods.errors);

    commands.spawn((
        Camera2dBundle::default(),
        MenuCamera
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(25.),
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                justify_content: JustifyContent::Center,
                grid_template_columns: vec![GridTrack::percent(100.)],
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::auto(),
                    GridTrack::auto()
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::CYAN),
            ..default()
        })
            .with_children(|builder| {
                basic_button(builder, BasicButton {
                    id: ButtonId::TitleSceneButton(TitleSceneButtonId::TitleSceneSingleplayer),
                    text: String::from("Singleplayer"),
                    style: ButtonStyle::Primary,
                    tag: TitleSceneButtonTag
                });
                basic_button(builder, BasicButton {
                    id: ButtonId::TitleSceneButton(TitleSceneButtonId::TitleSceneMultiplayer),
                    text: String::from("Multiplayer"),
                    style: ButtonStyle::Secondary,
                    tag: TitleSceneButtonTag
                });
                basic_button(builder, BasicButton {
                    id: ButtonId::TitleSceneButton(TitleSceneButtonId::TitleSceneOptions),
                    text: String::from("Options"),
                    style: ButtonStyle::Tertiary,
                    tag: TitleSceneButtonTag
                });
            });

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

fn interaction(
    mut interaction_query: Query<(&Interaction, &ButtonId), (Changed<Interaction>, With<TitleSceneButtonTag>)>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for (interaction, id) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let &ButtonId::TitleSceneButton(id) = &id {
                next_state.set(match id {
                    TitleSceneButtonId::TitleSceneSingleplayer => GameState::SingleplayerScene,
                    TitleSceneButtonId::TitleSceneMultiplayer => GameState::MultiplayerScene,
                    TitleSceneButtonId::TitleSceneOptions => GameState::OptionsScene(OptionsState::Main),
                    TitleSceneButtonId::TitleSceneCredits => GameState::CreditsScene,
                });
            }
        }
    }
}