use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use scenes::{boot::BootScenePlugin, credits::CreditsScenePlugin, multiplayer::MultiplayerScenePlugin, options::OptionsScenePlugin, singleplayer::SingleplayerScenePlugin, title::TitleScenePlugin};
use states::GameState;
use util::ui::{BackToTitleSceneButtonTag, ButtonInteractionEvent, ButtonInteractionType, ButtonState};

pub mod cameras;
pub mod client;
pub mod common;
pub mod scenes;
pub mod server;
pub mod states;
pub mod util;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct BarrenlandsPlugin;

impl Plugin for BarrenlandsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins((
                BootScenePlugin,
                CreditsScenePlugin,
                MultiplayerScenePlugin,
                OptionsScenePlugin,
                SingleplayerScenePlugin,
                TitleScenePlugin
            ))
            .init_state::<GameState>()
            .add_event::<ButtonInteractionEvent>()
            .add_systems(Startup, startup)
            .add_systems(Update, (button_press, back_button));
    }
}

fn startup() {
    println!("Barrenlands");
}

fn button_press(
    mut interaction_query: Query<(Entity, &Interaction, &mut ButtonState), Changed<Interaction>>,
    mut interaction_event: EventWriter<ButtonInteractionEvent>
) {
    for (entity, interaction, mut state) in &mut interaction_query {
        // Broken
        if *interaction == Interaction::Pressed && state.previous == Interaction::None {
            state.pressed = true;
            interaction_event.send(ButtonInteractionEvent(entity, ButtonInteractionType::PressStart));
        }

        if *interaction == Interaction::Hovered && state.previous == Interaction::Pressed {
            state.pressed = false;
            interaction_event.send(ButtonInteractionEvent(entity, ButtonInteractionType::PressEnd));
        }

        state.previous = *interaction;
    }
}

fn back_button(
    mut interaction_event: EventReader<ButtonInteractionEvent>,
    interaction_query: Query<&BackToTitleSceneButtonTag>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for event in interaction_event.read() {
        if event.1 == ButtonInteractionType::PressEnd {
            if let Ok(_) = interaction_query.get(**event) {
                if event.1 == ButtonInteractionType::PressEnd {
                    next_state.set(GameState::TitleScene);
                }
            }
        }
    }
}