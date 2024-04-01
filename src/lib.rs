use bevy::prelude::*;
use scenes::{boot::BootScenePlugin, options::OptionsScenePlugin, title::TitleScenePlugin};
use states::GameState;
use util::ui::BackToTitleSceneButtonTag;

pub mod cameras;
pub mod scenes;
pub mod states;
pub mod util;

pub struct BarrenlandsPlugin;

impl Plugin for BarrenlandsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(BootScenePlugin)
            .add_plugins(TitleScenePlugin)
            .add_plugins(OptionsScenePlugin)
            .init_state::<GameState>()
            .add_systems(Startup, (
                || {
                    println!("Hello Barrenlands!");
                },
                startup
            ))
            .add_systems(Update, (back_button));
    }
}

fn startup(mut commands: Commands) {

}

fn back_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackToTitleSceneButtonTag>)>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::TitleScene);
        }
    }
}