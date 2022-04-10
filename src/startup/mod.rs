use bevy::prelude::*;

use crate::state::GameState;

/// Checks whether the
#[derive(Debug, Default, Clone)]
pub struct StartupState {
    pub assets_loaded: bool,
}

impl StartupState {
    /// Check if the startup is completed
    pub fn complete(&self) -> bool {
        self.assets_loaded
    }
}

/// The plugin that manages the startup state'
pub struct StartupStatePlugin;

impl Plugin for StartupStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StartupState>();
        app.add_system_set(
            SystemSet::on_update(GameState::Startup).with_system(check_for_completion),
        );
    }
}

/// Advance to the main menu if the startup is completed
fn check_for_completion(state: Res<StartupState>, mut gamestate: ResMut<State<GameState>>) {
    if state.complete() {
        if *gamestate.current() != GameState::MainMenu {
            gamestate.set(GameState::MainMenu).unwrap();
        }
    }
}
