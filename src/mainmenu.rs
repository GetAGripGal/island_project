use bevy::{prelude::*, app::AppExit};

use crate::{state::GameState, prelude::Fonts};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);

/// The marker component for ui for the main menu
#[derive(Debug, Default, Component, Clone)]
pub struct MainMenuUi;

/// The marker component for a button for the main menu
#[derive(Debug, Default, Component, Clone)]
pub struct MainMenuButton;

/// The marker component for the start button
#[derive(Debug, Default, Component, Clone)]
pub struct StartButton;

/// The marker component for the settings button
#[derive(Debug, Default, Component, Clone)]
pub struct SettingsButton;

/// The marker component for the quit button
#[derive(Debug, Default, Component, Clone)]
pub struct QuitButton;

/// Hanldes the main menu state
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup));
        app.add_system_set(SystemSet::on_update(GameState::MainMenu)
            .with_system(change_button_looks)
            .with_system(handle_start_button)
            .with_system(handle_quit_button)
        );
        app.add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(destroy));
    }
}

/// Setup the main menu
fn setup(fonts: Res<Fonts>, mut commands: Commands) {
    // Spawn the ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    // Spawn the main menu ui
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
        .insert(MainMenuUi)
        .with_children(|parent| {
            // Spawn the title text
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section("Island Project", TextStyle {
                        font: fonts.fira_sans.clone(),
                        font_size: 100.0,
                        ..Default::default()
                    }, Default::default()),
                    ..Default::default()
                });

            // Spawn the start button
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
                .insert(MainMenuButton)
                .insert(StartButton)
                .with_children(|parent| {
                    // Spawn the start button text
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Start",
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
                .insert(MainMenuButton)
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

            // Spawn the quit button
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
                .insert(MainMenuButton)
                .insert(QuitButton)
                .with_children(|parent| {
                    // Spawn the quit button text
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Quit",
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

/// Destroys the main menu ui
fn destroy(ui: Query<Entity, With<MainMenuUi>>, mut commands: Commands) {
    ui.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
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

/// Handle the start menu button
fn handle_start_button(buttons: Query<&Interaction, (Changed<Interaction>, With<Button>, With<StartButton>)>, mut state: ResMut<State<GameState>>) {
    buttons.for_each(|interaction| {
        if *interaction == Interaction::Clicked {
            if *state.current() != GameState::Gameplay {
                state.set(GameState::Gameplay).unwrap();
            }
        }
    });
}

/// Handle the quit button
fn handle_quit_button(buttons: Query<&Interaction, (Changed<Interaction>, With<Button>, With<QuitButton>)>, mut quit_event: EventWriter<AppExit>) {
    buttons.for_each(|interaction| {
        if *interaction == Interaction::Clicked {
            quit_event.send(AppExit);
        }
    });
}