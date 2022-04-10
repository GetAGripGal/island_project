use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

/// Represents the physics body
#[derive(Debug, Clone, Component, PartialEq, Eq)]
pub enum PhysicsBody {
    Dynamic,
    Static,
}

/// Represents a physics body's velocity
#[derive(Debug, Clone, Component)]
pub struct Velocity(pub Vec2);

/// Represents a phsyics body's friction
#[derive(Debug, Clone, Component)]
pub struct Friction(pub Vec2);

/// Represents the gravity scale
#[derive(Debug, Clone, Component)]
pub struct GravityScale(pub f32);

impl From<Vec2> for Velocity {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}

impl From<Vec2> for Friction {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}

impl Deref for Velocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Friction {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Friction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The direction of the collision
#[derive(Debug, Default, Clone, Component)]
pub struct CollidingDirections(pub Vec<Vec2>);

impl Deref for CollidingDirections {
    type Target = Vec<Vec2>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CollidingDirections {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The entities the entity collided with
#[derive(Debug, Default, Clone, Component)]
pub struct CollidingEntities(pub Vec<Entity>);

impl Deref for CollidingEntities {
    type Target = Vec<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CollidingEntities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Represents an AABB physics collider
#[derive(Debug, Clone, Component)]
pub struct Collider {
    pub tags: Vec<String>,
    pub colliding_tags: Vec<String>,
    pub half_extents: Vec2,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            tags: vec![],
            colliding_tags: vec!["ground".into()],
            half_extents: Vec2::ZERO,
        }
    }
}

/// A bundle for a physics bundle
#[derive(Debug, Clone, Bundle)]
pub struct PhysicsBodyBundle {
    pub transform: Transform,
    pub body: PhysicsBody,
    pub velocity: Velocity,
    pub friction: Friction,
    pub gravity_scale: GravityScale,
}

impl Default for PhysicsBodyBundle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            body: PhysicsBody::Dynamic,
            velocity: Vec2::ZERO.into(),
            friction: Vec2::ZERO.into(),
            gravity_scale: GravityScale(1.0),
        }
    }
}
