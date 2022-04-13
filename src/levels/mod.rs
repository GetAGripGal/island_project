use std::path::PathBuf;
use bevy::prelude::*;

use crate::prelude::{
    CameraTarget, Collider, CollidingDirections, CollidingEntities,
    GravityScale, Inventory, PhysicsBody, PhysicsBodyBundle, PlayerBundle,
    PlayerMovementStats, SpawnItemEvent,
};

/// The marker component for a level object
#[derive(Debug, Clone, Component)]
pub struct LevelObject;

/// The event for spawning a level
pub struct SpawnLevelEvent(pub PathBuf);

/// The event that destroys the currently spawned levels
pub struct DestroyLevelsEvent;

/// The plugin for loading and spawning levels
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLevelEvent>();
        app.add_event::<DestroyLevelsEvent>();
        app.add_system(spawn_level_event);
        app.add_system(destroy_levels_event);
    }
}

/// Handles the spawning of a level
fn spawn_level_event(
    mut commands: Commands,
    mut camera_targets: Query<&mut CameraTarget>,
    mut spawn_level_events: EventReader<SpawnLevelEvent>,
    mut spawn_item_events: EventWriter<SpawnItemEvent>,
    asset_server: Res<AssetServer>,
) {
    for _ in spawn_level_events.iter() {
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
            .insert(LevelObject)
            .id();
        
        // Assign the player to the camera target
        camera_targets.for_each_mut(|mut target| {
            target.0 = Some(player);
        });

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
            .insert(Transform::from_xyz(0.0, -64.0, 0.0))
            .insert(LevelObject);

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
            .insert(Transform::from_xyz(-164.0, -48.0, 0.0))
            .insert(LevelObject);

        spawn_item_events.send(SpawnItemEvent {
            item: "mirror".into(),
            position: Vec2::new(-64.0, 20.0),
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
        spawn_item_events.send(SpawnItemEvent {
            item: "bean".into(),
            position: Vec2::new(48.0, 20.0),
            velocity: Vec2::new(0.0, 0.0),
        });
        spawn_item_events.send(SpawnItemEvent {
            item: "bean".into(),
            position: Vec2::new(32.0, 20.0),
            velocity: Vec2::new(0.0, 0.0),
        });
    }
}

/// Handle the level destroying event
fn destroy_levels_event(
    objects: Query<Entity, With<LevelObject>>,
    mut commands: Commands,
    mut camera_targets: Query<&mut CameraTarget>,
    mut destroy_level_events: EventReader<DestroyLevelsEvent>,
) {
    for _ in destroy_level_events.iter() {
        // Remove the player to the camera target
        camera_targets.for_each_mut(|mut target| {
            target.0 = None;
        });

        objects.for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });
    }
}