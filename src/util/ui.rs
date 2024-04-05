use bevy::prelude::*;

#[derive(Component)]
pub struct ButtonState {
    pub previous: Interaction,
    pub pressed: bool,
    pub hovered: bool
}

impl Default for ButtonState {
    fn default() -> Self {
        Self {
            previous: Interaction::None,
            pressed: false,
            hovered: false
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ButtonInteractionType {
    HoverStart,
    HoverEnd,
    PressStart,
    PressEnd
}

#[derive(Deref, Event)]
pub struct ButtonInteractionEvent(#[deref] pub Entity, pub ButtonInteractionType);

#[derive(Component)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Tertiary
}

#[derive(Component, Eq, PartialEq)]
pub enum ButtonId {
    TitleSceneButton(TitleSceneButtonId),
    Id(u32),
    None
}

#[derive(Eq, PartialEq, Debug)]
pub enum TitleSceneButtonId {
    TitleSceneSingleplayer,
    TitleSceneMultiplayer,
    TitleSceneOptions,
    TitleSceneCredits
}

#[derive(Component)]
pub struct TitleSceneButtonTag;
#[derive(Component)]
pub struct SingleplayerSceneButtonTag;
#[derive(Component)]
pub struct MultiplayerSceneButtonTag;
#[derive(Component)]
pub struct OptionsSceneButtonTag;
#[derive(Component)]
pub struct CreditsSceneButtonTag;

#[derive(Component)]
pub struct BackToTitleSceneButtonTag;

pub struct BasicButton<T: Component> {
    pub id: ButtonId,
    pub text: String,
    pub style: ButtonStyle,
    pub tag: T
}

pub fn ui_node() -> NodeBundle {
    NodeBundle {
        background_color: BackgroundColor(Color::NONE),
        style: Style {
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            ..default()
        },
        // z_index: ZIndex::Global(1),
        ..default()
    }
}

pub fn button<T: Component>(builder: &mut ChildBuilder, text: Text, button: ButtonBundle, id: ButtonId, tag: T) {
    builder
        .spawn((button, id, tag, ButtonState::default()))
            .with_children(|builder| button_text(builder, text));
}

fn button_text(builder: &mut ChildBuilder, text: Text) {
    builder.spawn(TextBundle {
        text,
        style: Style {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        ..default()
    });
}

pub fn basic_button<T: Component>(builder: &mut ChildBuilder, b: BasicButton<T>) {
    let BasicButton { id, text, style, tag } = b;

    match style {
        ButtonStyle::Primary => button(
            builder,
            Text::from_section(
                text,
                TextStyle {
                    color: Color::BLACK,
                    font_size: 20.,
                    ..default()
                }
            ),
            ButtonBundle {
                background_color: BackgroundColor(Color::WHITE),
                style: Style {
                    margin: UiRect::all(Val::Px(10.)),
                    padding: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                ..default()
            },
            id,
            tag
        ),
        ButtonStyle::Secondary => button(
            builder,
            Text::from_section(
                text,
                TextStyle {
                    color: Color::WHITE,
                    font_size: 20.,
                    ..default()
                }
            ),
            ButtonBundle {
                background_color: BackgroundColor(Color::BLUE),
                style: Style {
                    margin: UiRect::all(Val::Px(10.)),
                    padding: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                ..default()
            },
            id,
            tag
        ),
        ButtonStyle::Tertiary => button(
            builder,
            Text::from_section(
                text,
                TextStyle {
                    color: Color::WHITE,
                    font_size: 20.,
                    ..default()
                }
            ),
            ButtonBundle {
                background_color: BackgroundColor(Color::GREEN),
                style: Style {
                    margin: UiRect::all(Val::Px(10.)),
                    padding: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                ..default()
            },
            id,
            tag
        )
    };
}

pub fn back_button<T: Component>(builder: &mut ChildBuilder, tag: T) {
    button(
        builder,
        Text::from_section(
            "Back",
            TextStyle {
                color: Color::BLACK,
                font_size: 20.,
                ..default()
            }
        ),
        ButtonBundle {
            background_color: BackgroundColor(Color::WHITE),
            style: Style {
                left: Val::Px(20.),
                margin: UiRect::all(Val::Px(10.)),
                padding: UiRect::all(Val::Px(20.)),
                position_type: PositionType::Absolute,
                top: Val::Px(20.),
                ..default()
            },
            ..default()
        },
        ButtonId::None,
        tag
    );
}