use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use crate::prelude::GameState;

/// The plugin for handling the paused state
pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Paused)
            .with_system(draw_paused_menu)
        );
    }
}

/// Draws the paused menu
fn draw_paused_menu(
    mut gamestate: ResMut<State<GameState>>,
    mut egui: ResMut<EguiContext>,
) {
    egui::Window::new("Paused")
        .title_bar(true)
        .anchor(egui::Align2::CENTER_CENTER, &[0.0, 0.0])
        .fixed_pos(&[0.0, 0.0])
        .fixed_size(&[300.0, 200.0])
        .resizable(false)
        .collapsible(false)
        .show(egui.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                // The resume button
                if ui.button("Resume").clicked() {
                    if *gamestate.current() != GameState::Gameplay {
                        gamestate.set(GameState::Gameplay).unwrap();
                    }
                }
                // The settings menu
                if ui.button("Settings").clicked() {
                    todo!("Implement a settings menu");
                }
                // The exit buttom
                if ui.button("Exit").clicked() {
                    if *gamestate.current() != GameState::MainMenu {
                        gamestate.set(GameState::MainMenu).unwrap();
                    }
                }
            });
        });
}