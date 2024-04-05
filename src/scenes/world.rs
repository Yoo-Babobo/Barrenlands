use bevy::prelude::*;

use crate::{cameras::WorldCamera, scenes, states::GameState};

pub struct LoadingScenePlugin;

impl Plugin for LoadingScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::WorldScene), (scenes::despawn, setup).chain())
            .add_systems(Update, (|| {})
                .run_if(in_state(GameState::WorldScene)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle::default(),
        WorldCamera
    ));

    
}