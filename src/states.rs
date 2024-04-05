use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    BootScene,
    CreditsScene,
    LoadingScene,
    MultiplayerScene,
    OptionsScene(OptionsState),
    SingleplayerScene,
    TitleScene,
    WorldScene
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum OptionsState {
    #[default]
    Main,
    Controls,
    Sound
}