//! A story-based 2d metroidvenia with rpg-elements
//! Inspired by games like super paper mario and hollow knight
use bevy::prelude::*;
use island_project::prelude::*;

/// The games entrypoint
/// Builds the bevy app and runs it
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Island Project".into(),
            width: 1280.0 * 1.25,
            height: 720.0 * 1.25,
            ..Default::default()
        })
        .insert_resource(PhysicsConfig {
            enabled: true,
            gravity: Vec2::new(0.0, -80.0),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(GameStatePlugin)
        .add_plugin(StartupStatePlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GameplayPlugin)
        .add_plugin(PausedPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ItemPlugin)
        .run();
}
