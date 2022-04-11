use bevy::prelude::*;

/// The enum for the game state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Startup,
    MainMenu,
    Gameplay,
    Paused,
    Menu,
    Cutscene,
    GameOver,
}

/// The plugin that manages the game state
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Startup);
    }
}
