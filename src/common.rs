use bevy::input::gestures::PinchGesture;
use bevy::input::mouse::MouseWheel;

use bevy::window::PrimaryWindow;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            // .add_systems(Startup)
            .add_systems(Update, (camera_zoom_system, camera_pan_system));
    }
}

pub fn camera_zoom_system(
    mut camera: Query<Option<Mut<OrthographicProjection>>, With<Camera>>,
    mut evr_gesture_pinch: EventReader<PinchGesture>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let win = q_windows.single();

    let width = win.width();
    let height = win.height();

    let amount = evr_gesture_pinch.read().map(|ev| ev.0).sum::<f32>();

    if amount == 0.0 {
        return;
    }

    for projection in camera.iter_mut() {
        let Some(mut projection) = projection else {
            continue;
        };

        let Some(current_cursor_pos) = win.cursor_position() else {
            continue;
        };

        projection.scale -= amount;
        projection.scale = projection.scale.clamp(0.001, 3.0);
        projection.viewport_origin =
            Vec2::new(current_cursor_pos.x / width, current_cursor_pos.y / height);
    }
}

pub fn camera_pan_system(
    mut camera: Query<Mut<Transform>, With<Camera>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    for mut transform in camera.iter_mut() {
        let mut total_motion: Vec2 = evr_scroll.read().map(|ev| Vec2::new(ev.x, ev.y)).sum();

        if total_motion == Vec2::ZERO {
            continue;
        }

        total_motion.x = -total_motion.x;

        transform.translation.x += total_motion.x;
        transform.translation.y += total_motion.y;
    }
}
