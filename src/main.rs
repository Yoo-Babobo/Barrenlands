#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*, window::{PresentMode, WindowResolution}};

use barrenlands::{util, BarrenlandsPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    prevent_default_event_handling: false,
                    resolution: WindowResolution::new(910., 540.),
                    title: "Barrenlands".into(),
                    ..default()
                }),
                ..default()
            }),
            BarrenlandsPlugin,
            util::fps::FPSPlugin
        ))
        .run();
}