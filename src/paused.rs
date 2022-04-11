use bevy::prelude::*;

use crate::prelude::{GameState, Fonts};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);

/// The marker component for a paused menu ui
#[derive(Debug, Clone, Component)]
pub struct PausedMenuUi;

/// The marker component for the resume button
#[derive(Debug, Clone, Component)]
pub struct ResumeButton;

/// The marker component for the settings button
#[derive(Debug, Clone, Component)]
pub struct SettingsButton;

/// The marker component for the exit button
#[derive(Debug, Clone, Component)]
pub struct ExitButton;

/// The plugin for handling the paused state
pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Paused).with_system(setup));
        app.add_system_set(SystemSet::on_update(GameState::Paused)
            .with_system(change_button_looks)
            .with_system(handle_resume_button)
            //.with_system(handle_settings_button)
            .with_system(handle_exit_button));
        app.add_system_set(SystemSet::on_exit(GameState::Paused).with_system(destroy));
    }
}

/// Setup the paused state
fn setup(fonts: Res<Fonts>, mut commands: Commands) {
    commands
    .spawn_bundle(NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: Rect::all(Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
        ..Default::default()
    })
    .insert(PausedMenuUi)
    .with_children(|parent| {
        // Spawn the title text
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Paused",
                TextStyle {
                    font: fonts.fira_sans.clone(),
                    font_size: 60.0,
                    ..Default::default()
                },
                Default::default(),
            ),
            ..Default::default()
        });

        // Spawn the return button
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::BLACK.into(),
                ..Default::default()
            })
            .insert(PausedMenuUi)
            .insert(ResumeButton)
            .with_children(|parent| {
                // Spawn the start button text
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Resume",
                        TextStyle {
                            font: fonts.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            });
        // Spawn the settings button
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::BLACK.into(),
                ..Default::default()
            })
            .insert(PausedMenuUi)
            .insert(SettingsButton)
            .with_children(|parent| {
                // Spawn the settings button text
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Settings",
                        TextStyle {
                            font: fonts.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            });

        // Spawn the exit button
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::BLACK.into(),
                ..Default::default()
            })
            .insert(PausedMenuUi)
            .insert(ExitButton)
            .with_children(|parent| {
                // Spawn the quit button text
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Exit",
                        TextStyle {
                            font: fonts.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            });
    });
}

/// Change the button looks based on the interaction
fn change_button_looks(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

/// Handle the Resume menu button
fn handle_resume_button(
    buttons: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ResumeButton>)>,
    mut state: ResMut<State<GameState>>,
) {
    buttons.for_each(|interaction| {
        if *interaction == Interaction::Clicked {
            // Set the state to gameplay
            if *state.current() != GameState::Gameplay {
                state.set(GameState::Gameplay).unwrap();
            }
        }
    });
}

/// Handle the exit menu button
fn handle_exit_button(
    buttons: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ExitButton>)>,
    mut state: ResMut<State<GameState>>,
) {
    buttons.for_each(|interaction| {
        if *interaction == Interaction::Clicked {
            // Set the state to gameplay
            if *state.current() != GameState::MainMenu {
                state.set(GameState::MainMenu).unwrap();
            }
        }
    });
}

/// Destroys the pause menu ui
fn destroy(ui: Query<Entity, With<PausedMenuUi>>, mut commands: Commands) {
    ui.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}