use bevy::prelude::*;

use crate::{
    physics::component::{
        Collider, CollidingDirections, CollidingEntities, GravityScale, PhysicsBody,
        PhysicsBodyBundle,
    },
    player::component::{PlayerBundle, PlayerMovementStats},
    prelude::{
        CameraOffset, CameraSpeed, CameraTarget, GameCameraConfig, Inventory, SpawnItemEvent,
    },
    state::GameState,
};

/// The plugin that manages the gameplay state
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(setup));
    }
}

/// Readies the game for the gameplay state
fn setup(
    mut commands: Commands,
    mut spawn_item_events: EventWriter<SpawnItemEvent>,
    asset_server: Res<AssetServer>,
) {
    // Spawn the player
    let player = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            ..Default::default()
        })
        .insert_bundle(PhysicsBodyBundle {
            gravity_scale: GravityScale(2.0),
            ..Default::default()
        })
        .insert(Collider {
            tags: vec!["player".into()],
            colliding_tags: vec!["ground".into()],
            half_extents: Vec2::new(5.0, 10.0),
        })
        .insert(CollidingDirections::default())
        .insert(CollidingEntities::default())
        .insert_bundle(PlayerBundle {
            player_movement_stats: PlayerMovementStats {
                walking_accel: 325.0,
                sprint_accel: 450.0,
                walking_friction: 6.4,
                jump_force: 1250.0,
                jump_time: 0.08,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Inventory::with_capacity(10))
        .id();

    // Spawn the camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCameraConfig {
            scale: 0.8,
            viewport_size: Vec2::new(320.0, 180.0),
            ..Default::default()
        })
        .insert(CameraTarget(player))
        .insert(CameraSpeed(2.0))
        .insert(CameraOffset(Vec2::new(0.0, 24.0)));

    // TODO: Replace the test level with a loaded level
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba_u8(255, 0, 0, 128),
                custom_size: Vec2::new(128.0, 16.0).into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(PhysicsBodyBundle {
            body: PhysicsBody::Static,
            ..Default::default()
        })
        .insert(Collider {
            tags: vec!["ground".into()],
            colliding_tags: vec!["ground".into()],
            half_extents: Vec2::new(64.0, 8.0),
        })
        .insert(Transform::from_xyz(0.0, -64.0, 0.0));

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba_u8(255, 0, 0, 128),
                custom_size: Vec2::new(96.0, 16.0).into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(PhysicsBodyBundle {
            body: PhysicsBody::Static,
            ..Default::default()
        })
        .insert(Collider {
            tags: vec!["ground".into()],
            colliding_tags: vec!["ground".into()],
            half_extents: Vec2::new(48.0, 8.0),
        })
        .insert(Transform::from_xyz(-164.0, -48.0, 0.0));

    spawn_item_events.send(SpawnItemEvent {
        item: "mirror".into(),
        position: Vec2::new(32.0, 20.0),
        velocity: Vec2::new(0.0, 0.0),
    });
    spawn_item_events.send(SpawnItemEvent {
        item: "monocle".into(),
        position: Vec2::new(-32.0, 20.0),
        velocity: Vec2::new(-0.0, 0.0),
    });
    spawn_item_events.send(SpawnItemEvent {
        item: "bean".into(),
        position: Vec2::new(64.0, 20.0),
        velocity: Vec2::new(0.0, 0.0),
    });
}
