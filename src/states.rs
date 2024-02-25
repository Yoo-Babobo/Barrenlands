use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Menu(fn(Commands) -> ()),
    World
}