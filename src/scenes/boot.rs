use bevy::prelude::*;

use crate::{cameras::MenuCamera, scenes, states::GameState, util::mods::Mods};

#[derive(Resource)]
struct BootTimer(Timer);

pub struct BootScenePlugin;

impl Plugin for BootScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BootTimer(Timer::from_seconds(2., TimerMode::Once)))
            .add_systems(OnEnter(GameState::BootScene), (scenes::despawn, setup).chain())
            .add_systems(Update, change
                .run_if(in_state(GameState::BootScene)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.init_resource::<Mods>();
    
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
            "Loading...",
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

fn change(time: Res<Time>, mut timer: ResMut<BootTimer>, mut next_state: ResMut<NextState<GameState>>) {
    if timer.0.tick(time.delta()).just_finished() { next_state.set(GameState::TitleScene) }
}