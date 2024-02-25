use bevy::prelude::*;

use barrenlands::{util, BarrenlandsPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Barrenlands".into(),
                    resolution: (910., 520.).into(),
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            BarrenlandsPlugin,
            util::fps::FPSPlugin
        ))
        .run();
}