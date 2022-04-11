use bevy::prelude::*;

use crate::state::GameState;

/// The plugin that manages the gameplay state
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(setup));
        app.add_system_set(SystemSet::on_update(GameState::Gameplay)
            .with_system(pause_game)
            .with_system(open_player_menu)
        );
    }
}

/// Sets up the gameplay state
fn setup() {
    info!("Initialize the gameplay state.");
}

/// Pause the game on escape
fn pause_game(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if *state.current() != GameState::Paused {
            state.set(GameState::Paused).unwrap();
        }
    }
}

/// Opens the player menu on Q pressed
fn open_player_menu(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        if *state.current() != GameState::Menu {
            state.set(GameState::Menu).unwrap();
        }
    }
}