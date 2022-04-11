use bevy::prelude::*;

/// The target for the camera to follow
#[derive(Debug, Clone, Component)]
pub struct CameraTarget(pub Option<Entity>);

/// The camera offset
#[derive(Debug, Default, Clone, Component)]
pub struct CameraOffset(pub Vec2);

/// The lerping speed for the camera
#[derive(Debug, Default, Clone, Component)]
pub struct CameraSpeed(pub f32);

/// The config for the game camera
#[derive(Debug, Default, Component)]
pub struct GameCameraConfig {
    // The scale of the viewport
    pub scale: f32,
    // The size of the viewport in pixels
    pub viewport_size: Vec2,
    // The id of the entity that should be followed
    pub target_id: Option<Entity>,
}

/// The plugin that handles the game camera
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system(camera_scaling);
        app.add_system(update_position);
    }
}

/// Setup the camera plugin
fn setup(mut commands: Commands) {
    // Spawn the camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCameraConfig {
            scale: 0.8,
            viewport_size: Vec2::new(320.0, 180.0),
            ..Default::default()
        })
        .insert(CameraTarget(None))
        .insert(CameraSpeed(2.0))
        .insert(CameraOffset(Vec2::new(0.0, 24.0)));
}

/// Handles the camera scaling
fn camera_scaling(
    windows: Res<Windows>,
    mut cameras: Query<(&GameCameraConfig, &mut OrthographicProjection)>,
) {
    cameras.iter_mut().for_each(|(config, mut projection)| {
        // Get the camera window
        let window = windows.get_primary().unwrap();
        projection.scale = (config.viewport_size.y * config.scale) / window.height();
    });
}

/// Move towards the lerping position
fn update_position(
    time: Res<Time>,
    transform_entities: Query<&Transform, Without<Camera>>,
    mut cameras: Query<
        (
            &mut Transform,
            &CameraTarget,
            &CameraSpeed,
            Option<&CameraOffset>,
        ),
        With<Camera>,
    >,
) {
    for (mut transform, target, speed, offset) in cameras.iter_mut() {
        if let Some(target) = target.0 {
            if let Ok(target) = transform_entities.get(target) {
                if let Some(offset) = offset {
                    transform.translation.x += (speed.0 * time.delta_seconds())
                        * (target.translation.x + offset.0.x - transform.translation.x);
                    transform.translation.y += (speed.0 * time.delta_seconds())
                        * (target.translation.y + offset.0.y - transform.translation.y);
                } else {
                    transform.translation.x +=
                        (speed.0 * time.delta_seconds()) * (target.translation.x - transform.translation.x);
                    transform.translation.y +=
                        (speed.0 * time.delta_seconds()) * (target.translation.y - transform.translation.y);
                }
            }
        } else {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }
    }
}
