use crate::prelude::GameState;

use self::component::{
    Collider, CollidingDirections, CollidingEntities, Friction, GravityScale, PhysicsBody, Velocity,
};
use bevy::prelude::*;
pub mod component;

/// The config for the physics
#[derive(Debug)]
pub struct PhysicsConfig {
    pub enabled: bool,
    pub gravity: Vec2,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            gravity: Vec2::new(0f32, -98.1f32),
        }
    }
}

/// The games physics plugin
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsConfig>();

        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(apply_gravity.label("apply_gravity"))
                .with_system(
                    handle_collisions
                        .label("handle_collision")
                        .after("apply_gravity"),
                )
                .with_system(
                    apply_velocity
                        .label("apply_velocity")
                        .after("handle_collision"),
                )
                .with_system(
                    apply_friction
                        .label("apply_friction")
                        .after("apply_velocity"),
                ),
        );
    }
}

/// Handle collisions between the bodies
fn handle_collisions(
    time: Res<Time>,
    mut collision_dirs: Query<&mut CollidingDirections>,
    mut colliding_entities: Query<&mut CollidingEntities>,
    mut bodies: Query<(&mut Velocity, Entity, &Collider, &Transform, &PhysicsBody)>,
) {
    // Reset the collision direction components
    collision_dirs.for_each_mut(|mut collision_dirs| {
        collision_dirs.clear();
    });
    // Reset the colliding entities
    colliding_entities.for_each_mut(|mut colliding_entities| {
        colliding_entities.clear();
    });

    // Handle collisions
    let mut iter = bodies.iter_combinations_mut();
    while let Some(
        [(mut velocity_a, entity_a, collider_a, transform_a, _), (mut velocity_b, entity_b, collider_b, transform_b, _)],
    ) = iter.fetch_next()
    {
        // Calulate the next positions for body a
        let next_x = transform_a.translation.x + velocity_a.x * time.delta_seconds();
        let next_y = transform_a.translation.y + velocity_a.y * time.delta_seconds();

        // The collision direction
        let mut direction_a = Vec2::ZERO;
        let mut colliding_entity_a = None;

        // Check the collisions between the different axises
        if !((next_x - transform_b.translation.x).abs()
            > collider_a.half_extents.x + collider_b.half_extents.x)
            && !((transform_a.translation.y - transform_b.translation.y).abs()
                > collider_a.half_extents.y + collider_b.half_extents.y)
        {
            // Assing the collision direction
            if velocity_a.x > 0.0 {
                direction_a.x = 1.0;
            }
            if velocity_a.x < 0.0 {
                direction_a.x = -1.0;
            }
            if collider_a
                .colliding_tags
                .iter()
                .all(|tag| collider_b.tags.contains(tag))
            {
                velocity_a.x = 0.0;
            }

            colliding_entity_a = Some(entity_b);
        }
        if !((transform_a.translation.x - transform_b.translation.x).abs()
            > collider_a.half_extents.x + collider_b.half_extents.x)
            && !((next_y - transform_b.translation.y).abs()
                > collider_a.half_extents.y + collider_b.half_extents.y)
        {
            // Assing the collision direction
            if velocity_a.y > 0.0 {
                direction_a.y = 1.0;
            }
            if velocity_a.y < 0.0 {
                direction_a.y = -1.0;
            }

            if collider_a
                .colliding_tags
                .iter()
                .all(|tag| collider_b.tags.contains(tag))
            {
                velocity_a.y = 0.0;
            }

            colliding_entity_a = Some(entity_b);
        }

        // Assign the collision direction
        if let Ok(mut directions_a) = collision_dirs.get_mut(entity_a) {
            directions_a.push(direction_a);
        }
        // Assign the colliding entity
        if let Ok(mut colliding_entities_a) = colliding_entities.get_mut(entity_a) {
            if let Some(colliding_entity_a) = colliding_entity_a {
                colliding_entities_a.push(colliding_entity_a);

                // Add the entity_a to entity_b's list
                if let Ok(mut colliding_entities_b) = colliding_entities.get_mut(entity_b) {
                    colliding_entities_b.push(entity_a);
                }
            }
        }

        // Calulate the next positions for body b
        let next_x = transform_b.translation.x + velocity_b.x * time.delta_seconds();
        let next_y = transform_b.translation.y + velocity_b.y * time.delta_seconds();

        // The collision direction
        let mut direction_b = Vec2::ZERO;
        let mut colliding_entity_b = None;

        // Check the collisions between the different axises
        if !((next_x - transform_a.translation.x).abs()
            > collider_b.half_extents.x + collider_a.half_extents.x)
            && !((transform_b.translation.y - transform_a.translation.y).abs()
                > collider_b.half_extents.y + collider_a.half_extents.y)
        {
            if velocity_b.x > 0.0 {
                direction_b.x = 1.0;
            }
            if velocity_b.x < 0.0 {
                direction_b.x = -1.0;
            }
            if collider_b
                .colliding_tags
                .iter()
                .all(|tag| collider_a.tags.contains(tag))
            {
                velocity_b.x = 0.0;
            }

            colliding_entity_b = Some(entity_a);
        }
        if !((transform_b.translation.x - transform_a.translation.x).abs()
            > collider_b.half_extents.x + collider_a.half_extents.x)
            && !((next_y - transform_a.translation.y).abs()
                > collider_b.half_extents.y + collider_a.half_extents.y)
        {
            if velocity_b.y > 0.0 {
                direction_b.y = 1.0;
            }
            if velocity_b.y < 0.0 {
                direction_b.y = -1.0;
            }
            if collider_b
                .colliding_tags
                .iter()
                .all(|tag| collider_a.tags.contains(tag))
            {
                velocity_b.y = 0.0;
            }
            colliding_entity_b = Some(entity_a);
        }

        // Assign the collision direction
        if let Ok(mut directions_b) = collision_dirs.get_mut(entity_b) {
            directions_b.push(direction_b);
        }
        // Assign the colliding entity
        if let Ok(mut colliding_entities_b) = colliding_entities.get_mut(entity_b) {
            if let Some(colliding_entity_b) = colliding_entity_b {
                colliding_entities_b.push(colliding_entity_b);

                // Add the entity_b to entity_a's list
                if let Ok(mut colliding_entities_a) = colliding_entities.get_mut(entity_a) {
                    colliding_entities_a.push(entity_b);
                }
            }
        }
    }
}

/// Apply the gravity to the body
fn apply_gravity(
    time: Res<Time>,
    physics_conf: Res<PhysicsConfig>,
    mut bodies: Query<(&mut Velocity, &PhysicsBody, &GravityScale)>,
) {
    for (mut velocity, body, scale) in bodies.iter_mut() {
        if *body == PhysicsBody::Dynamic {
            velocity.0 += physics_conf.gravity * scale.0 * time.delta_seconds();
        }
    }
}

/// Applies the velocity to the bodies
fn apply_velocity(time: Res<Time>, mut bodies: Query<(&mut Transform, &PhysicsBody, &Velocity)>) {
    for (mut transform, body, velocity) in bodies.iter_mut() {
        if *body == PhysicsBody::Dynamic {
            transform.translation.x += velocity.x * time.delta_seconds();
            transform.translation.y += velocity.y * time.delta_seconds();
        }
    }
}

/// Applies the friction to the bodies
fn apply_friction(time: Res<Time>, mut bodies: Query<(&mut Velocity, &PhysicsBody, &Friction)>) {
    for (mut velocity, body, friction) in bodies.iter_mut() {
        if *body == PhysicsBody::Dynamic {
            velocity.x += (friction.x * time.delta_seconds()) * (0f32 - velocity.x);
            velocity.y += (friction.y * time.delta_seconds()) * (0f32 - velocity.y);
        }
    }
}
