use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{items::inventory::Inventory, player::component::Player};

/// The state of the deug menu
#[derive(Debug, Default, Clone)]
pub struct DebugMenuState {
    pub is_shown: bool,
}

/// The plugin that handles the debug mode
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // Check if the game is in debug mode
        #[cfg(debug_assertions)]
        let debug_mode = true;
        #[cfg(not(debug_assertions))]
        let debug_mode = {
            let mut args = std::env::args();
            args.any(|arg| (arg == "--debug") || (arg == "-d"))
        };  

        if debug_mode {
            // Add the bevy egui plugin
            app.add_plugin(bevy_egui::EguiPlugin);

            app.init_resource::<DebugMenuState>();
            app.add_system(toggle_debug_menu);
            app.add_system(draw_debug_menu);
        }
    }
}

/// Toggles the debug menu
fn toggle_debug_menu(input: Res<Input<KeyCode>>, mut state: ResMut<DebugMenuState>) {
    if input.just_pressed(KeyCode::F12) {
        state.is_shown = !state.is_shown;
    }
}

/// Draw the debug menu using egui
fn draw_debug_menu(
    state: Res<DebugMenuState>,
    inventories: Query<(Entity, &Inventory), With<Player>>,
    mut egui: ResMut<EguiContext>,
) {
    // If the debug menu is not supposed to be shown, return
    if !state.is_shown {
        return
    }

    // The debug menu for the inventory
    egui::Window::new("Inventories")
        .show(egui.ctx_mut(), |ui| {
            // Draw the invwentory for each player
            inventories.for_each(|(entity, inventory)| {
                ui.collapsing(format!("Player: {}", entity.id()), |ui| {
                    for item in inventory.items.iter() {
                        ui.label(format!("{}", item.name));
                    }
                });
            });
        });
}