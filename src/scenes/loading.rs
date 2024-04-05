use bevy::prelude::*;

use crate::{cameras::MenuCamera, scenes, states::GameState};

#[derive(Resource)]
struct LoadingTimer(Timer);

pub struct LoadingScenePlugin;

impl Plugin for LoadingScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LoadingTimer(Timer::from_seconds(2., TimerMode::Once)))
            .add_systems(OnEnter(GameState::LoadingScene), (scenes::despawn, setup).chain())
            .add_systems(Update, change
                .run_if(in_state(GameState::LoadingScene)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    commands.spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        text: Text::from_section(
            "Loading world...",
            TextStyle {
                font_size: 50.,
                color: Color::CYAN,
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

fn change(time: Res<Time>, mut timer: ResMut<LoadingTimer>, mut next_state: ResMut<NextState<GameState>>) {
    if timer.0.tick(time.delta()).just_finished() { next_state.set(GameState::WorldScene) }
}