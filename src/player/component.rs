pub use bevy::prelude::*;

/// The market component for a player
#[derive(Debug, Clone, Component, Default)]
pub struct Player;

/// The player state
#[derive(Debug, Default, Clone, Component)]
pub struct PlayerState {
    // The player's input state
    pub input: PlayerInputState,
    // The player's movement state
    pub movement: PlayerMovementState,
    // The amount of time since the player jumped
    pub time_since_jump: f32,
}

/// The enum that signifies what controller the player is using
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerState {
    Keyboard,
    Gamepad(usize),
}

impl Default for ControllerState {
    fn default() -> Self {
        Self::Keyboard
    }
}

/// The input state for the player
#[derive(Debug, Default, Clone)]
pub struct PlayerInputState {
    // The controller used by the player
    pub controller: ControllerState,
    // The flag for if the player is jumping
    pub is_jumping: bool,
    // The flag for if the player is sprinting
    pub is_sprinting: bool,
    // The horizontal movement
    pub horizontal: f32,
}

/// The player movement stats
#[derive(Debug, Default, Component, Clone)]
pub struct PlayerMovementStats {
    // The player's walking accel
    pub walking_accel: f32,
    // The player's sprint accel
    pub sprint_accel: f32,
    // The walking friction
    pub walking_friction: f32,
    // The player's jump force
    pub jump_force: f32,
    // The amount of time the player can jump
    pub jump_time: f32,
}

/// The player movement state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerMovementState {
    Idle,
    Walking,
    Jumping,
    Falling,
}

impl Default for PlayerMovementState {
    fn default() -> Self {
        Self::Idle
    }
}

/// The bundle for player components
#[derive(Debug, Default, Bundle, Clone)]
pub struct PlayerBundle {
    pub player: Player,
    pub player_state: PlayerState,
    pub player_movement_stats: PlayerMovementStats,
}
