use bevy::prelude::*;

use crate::util::NoDespawn;

pub mod boot;
pub mod credits;
pub mod loading;
pub mod options;
pub mod title;
pub mod world;

pub fn despawn(mut commands: Commands, entities: Query<Entity, (Without<Window>, Without<NoDespawn>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}