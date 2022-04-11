use bevy::prelude::*;

use crate::prelude::{CollidingDirections, Velocity, GameState};

use self::component::{
    ControllerState, Player, PlayerMovementState, PlayerMovementStats, PlayerState,
};
pub mod component;

/// The plugin that handles the player controls
pub struct PlayerPlugin;

// Implement the plugin for player
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Gameplay)
            .with_system(check_controller.label("check_controller"))
            .with_system(update_input.label("update_input").after("check_controller"))
            .with_system(
                update_movement_state
                    .label("update_movement_state")
                    .after("update_input"),
            )
            .with_system(
                handle_movement
                    .label("handle_movement")
                    .after("update_movement_state"),
            )
            .with_system(animate)
        );
    }
}

/// Checks what controller the player is using and updates the player state
fn check_controller(
    keyboard_input: Res<Input<KeyCode>>,
    gamepad_input: Res<Input<GamepadButton>>,
    mut states: Query<&mut PlayerState>,
) {
    states.for_each_mut(|mut state| {
        // Check if the player is using a keyboard
        if keyboard_input.get_pressed().len() > 0 {
            state.input.controller = ControllerState::Keyboard;
        }

        // Check if the player is using a gamepad
        if gamepad_input.get_pressed().len() > 0 {
            state.input.controller = ControllerState::Gamepad(0);
        }
    });
}

/// Update the input for all the players
fn update_input(
    keyboard_input: Res<Input<KeyCode>>,
    gamepad_axis_input: Res<Axis<GamepadAxis>>,
    gamepad_button_input: Res<Input<GamepadButton>>,
    mut states: Query<&mut PlayerState>,
) {
    states.for_each_mut(|mut state| {
        match state.input.controller {
            ControllerState::Keyboard => {
                // Check if the player is jumping or sprinting
                state.input.is_jumping = keyboard_input.pressed(KeyCode::Space);
                state.input.is_sprinting = keyboard_input.pressed(KeyCode::LShift);

                // Get the horizontal input
                state.input.horizontal = -(keyboard_input.pressed(KeyCode::A) as i8 as f32)
                    + keyboard_input.pressed(KeyCode::D) as i8 as f32;
            }
            ControllerState::Gamepad(_) => {
                // Check if the player is jumping or sprinting
                state.input.is_jumping = gamepad_button_input
                    .pressed(GamepadButton(Gamepad(0), GamepadButtonType::South));
                state.input.is_sprinting = gamepad_button_input
                    .pressed(GamepadButton(Gamepad(0), GamepadButtonType::West));

                // Get the horizontal input
                state.input.horizontal = gamepad_axis_input
                    .get(GamepadAxis(Gamepad(0), GamepadAxisType::LeftStickX))
                    .unwrap();
            }
        }
    });
}

/// Update the player movement state
fn update_movement_state(mut states: Query<(&mut PlayerState, &Velocity)>) {
    states.for_each_mut(|(mut state, velocity)| {
        // Default to idle
        state.movement = PlayerMovementState::Idle;
        // Check if walking
        if state.input.horizontal.abs() > 0.0 {
            state.movement = PlayerMovementState::Walking;
        }
        // Check if jumping
        if velocity.y > 0.0 {
            state.movement = PlayerMovementState::Jumping;
        }
        // Check if falling
        if velocity.y < 0.0 {
            state.movement = PlayerMovementState::Falling;
        }
    });
}

/// Handle player movement
fn handle_movement(
    time: Res<Time>,
    mut states: Query<(
        &mut PlayerState,
        &CollidingDirections,
        &PlayerMovementStats,
        &mut Velocity,
    )>,
) {
    states.for_each_mut(|(mut state, collision_dir, stats, mut velocity)| {
        // Check if the player is on the floot
        let on_floor = collision_dir.contains(&Vec2::new(0.0, -1.0));

        // Check if the player is sprinting
        let speed = if state.input.is_sprinting {
            stats.sprint_accel
        } else {
            stats.walking_accel
        };

        // Handle the jumping
        if on_floor {
            state.time_since_jump = 0.0;
        }
        if state.input.is_jumping {
            if on_floor {
                // The initial jump force
                velocity.y = stats.jump_force * time.delta_seconds();
            } else if state.time_since_jump < stats.jump_time {
                velocity.y += stats.jump_force * time.delta_seconds();
                state.time_since_jump += time.delta_seconds();
            }
        } else {
            state.time_since_jump = stats.jump_time;
        }
        // Check if the player is walking
        if state.movement == PlayerMovementState::Walking
            || state.movement == PlayerMovementState::Jumping
            || state.movement == PlayerMovementState::Falling
        {
            // Move the player
            velocity.x += state.input.horizontal * speed * time.delta_seconds();
        }
        // Apply the friction
        velocity.x = velocity.x * (1.0 - stats.walking_friction * time.delta_seconds());
    });
}

/// Animates the player
fn animate(mut players: Query<(&mut Sprite, &Velocity), With<Player>>) {
    players.for_each_mut(|(mut sprite, velocity)| {
        // Check if the player is walking
        if velocity.x.abs() > 0.0 {
            // Check if the player is facing left
            if velocity.x < 0.0 {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }
        }
    });
}
