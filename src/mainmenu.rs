use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContext};

use crate::{state::GameState, levels::{SpawnLevelEvent, DestroyLevelsEvent}};

/// The marker component for ui for the main menu
#[derive(Debug, Default, Component, Clone)]
pub struct MainMenuUi;

/// Hanldes the main menu state
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(setup)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(draw_main_menu)
        );
    }
}

/// Setuo the main menu state
fn setup(mut destroy_level_events: EventWriter<DestroyLevelsEvent>) {
    // Destroy all the levels
    destroy_level_events.send(DestroyLevelsEvent);
}

/// Draws the main menu using egui
fn draw_main_menu(
    mut egui: ResMut<EguiContext>,
    mut gamestate: ResMut<State<GameState>>,
    mut spawn_level_events: EventWriter<SpawnLevelEvent>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    egui::Window::new("Project Island")
        .title_bar(true)
        .anchor(egui::Align2::CENTER_CENTER, &[0.0, 0.0])
        .fixed_pos(&[0.0, 0.0])
        .fixed_size(&[300.0, 200.0])
        .resizable(false)
        .collapsible(false)
        .show(egui.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                // The start button
                if ui.button("Start").clicked() {
                    if *gamestate.current() != GameState::Gameplay {
                        gamestate.set(GameState::Gameplay).unwrap();
                    }
                    spawn_level_events.send(SpawnLevelEvent("".into()));
                }
                // The settings menu
                if ui.button("Settings").clicked() {
                    todo!("Implement a settings menu");
                }
                // The quit buttom
                if ui.button("Quit").clicked() {
                    app_exit_events.send(AppExit);
                }
            });
        });
}