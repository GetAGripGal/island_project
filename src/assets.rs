use bevy::prelude::*;

use crate::{prelude::StartupState, state::GameState};

/// The fonts in the game
#[derive(Debug, Default, Clone)]
pub struct Fonts {
    pub fira_sans: Handle<Font>,
}

/// The state the asset loading systems are ing

#[derive(Debug, Default, Clone)]
pub struct AssetLoadingState {
    pub fonts_loaded: bool,
    pub textures_loaded: bool,
    pub sounds_loaded: bool,
    pub models_loaded: bool,
}

impl AssetLoadingState {
    /// Check if the asset loading is completed
    pub fn complete(&self) -> bool {
        self.fonts_loaded // && self.textures_loaded && self.sounds_loaded && self.models_loaded
    }
}

/// The plugin that handles loading the assets
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLoadingState>();
        app.add_system_set(SystemSet::on_enter(GameState::Startup).with_system(load_fonts));
        app.add_system_set(
            SystemSet::on_update(GameState::Startup).with_system(check_for_completion),
        );
    }
}

/// Load the games fonts
fn load_fonts(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut state: ResMut<AssetLoadingState>,
) {
    info!("Loading fonts");
    commands.insert_resource(Fonts {
        fira_sans: asset_server.load("fonts/FiraSans-Bold.ttf"),
    });

    state.fonts_loaded = true;
}

/// Checks whether the asset loading is completed and update the startup state
fn check_for_completion(state: Res<AssetLoadingState>, mut startup_state: ResMut<StartupState>) {
    if state.complete() {
        startup_state.assets_loaded = true;
    }
}
