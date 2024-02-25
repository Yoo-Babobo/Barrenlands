use bevy::prelude::*;
use scenes::title;

pub mod cameras;
pub mod scenes;
pub mod states;
pub mod util;

pub struct BarrenlandsPlugin;

impl Plugin for BarrenlandsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                || {
                    println!("Hello Barrenlands!");
                },
                title::setup,
                startup
            ));
    }
}

fn startup(mut commands: Commands) {
    
}