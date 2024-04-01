use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    BootScene,
    TitleScene,
    SingleplayerScene,
    MultiplayerScene,
    OptionsScene(OptionsState),
    CreditsScene,
    World
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum OptionsState {
    #[default]
    Main,
    Controls,
    Sound
}