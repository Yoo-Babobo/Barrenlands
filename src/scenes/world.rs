use bevy::prelude::*;

use crate::cameras::WorldCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            ..default()
        },
        WorldCamera
    ));
}