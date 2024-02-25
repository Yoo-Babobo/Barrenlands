use bevy::prelude::*;

pub mod boot;
pub mod credits;
pub mod loading;
pub mod settings;
pub mod title;
pub mod world;

pub fn despawn(mut commands: Commands, entities: Query<Entity, Without<Window>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}