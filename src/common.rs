use bevy::input::gestures::PinchGesture;
use bevy::input::mouse::MouseWheel;

use bevy::window::PrimaryWindow;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

/// Provides some common functionallity for all examples.
/// Like toggling visibility and through origin.
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            // .add_systems(Startup)
            .add_systems(
                Update,
                (
                    // keyboard_input_system,
                    // fps_text_update_system,
                    // origin_text_update_system,
                    camera_zoom_system,
                    camera_pan_system,
                ),
            );
    }
}

// fn setup_legend(mut commands: Commands, asset_server: Res<AssetServer>) {
// let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
// let font_medium = asset_server.load("fonts/FiraMono-Medium.ttf");

// commands
//     .spawn((
//         Text::default(),
//         TextColor::WHITE,
//         TextFont::from_font(font_medium).with_font_size(20.0),
//         Node {
//             position_type: PositionType::Absolute,
//             top: Val::Px(5.0),
//             right: Val::Px(15.0),
//             ..default()
//         },
//     ))
//     .with_children(|commands| {
//         commands.spawn((
//             TextSpan::new("Key Info"),
//             TextFont::from_font(font_bold.clone()).with_font_size(30.0),
//         ));
//         commands.spawn((TextSpan::new("\nF"), TextFont::from_font(font_bold.clone())));
//         commands.spawn(TextSpan::new(" - Toggle Frame Diagnostics"));
//         commands.spawn((TextSpan::new("\nO"), TextFont::from_font(font_bold.clone())));
//         commands.spawn(TextSpan::new(" - Cycle through Origins"));
//         commands.spawn((TextSpan::new("\nV"), TextFont::from_font(font_bold.clone())));
//         commands.spawn(TextSpan::new(" - Toggle visibility"));
//     });
// }

#[derive(Component)]
pub struct DontChange;

/// This system toggles SVG visibility when 'V' is pressed and toggles through
/// origin when 'O' is pressed.
// fn keyboard_input_system(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut svg_query: Query<
//         (&mut Origin, &mut Visibility),
//         (Or<(With<Svg2d>, With<Svg3d>)>, Without<DontChange>),
//     >,
//     mut ui_query: Query<
//         &mut Visibility,
//         (
//             With<Text>,
//             Or<(With<FpsTextRoot>, With<OriginTextRoot>)>,
//             Without<Svg2d>,
//             Without<Svg3d>,
//         ),
//     >,
// ) {
//     if keyboard_input.just_pressed(KeyCode::KeyV) {
//         for (_, mut visible) in svg_query.iter_mut() {
//             *visible = match *visible {
//                 Visibility::Hidden => Visibility::Inherited,
//                 Visibility::Visible | Visibility::Inherited => Visibility::Hidden,
//             };
//         }
//     } else if keyboard_input.just_pressed(KeyCode::KeyO) {
//         for (mut origin, _) in svg_query.iter_mut() {
//             *origin = match origin.as_ref() {
//                 Origin::BottomLeft => Origin::BottomRight,
//                 Origin::BottomRight => Origin::TopRight,
//                 Origin::Center => Origin::BottomLeft,
//                 Origin::TopLeft => Origin::Center,
//                 Origin::TopRight => Origin::TopLeft,
//                 Origin::Custom(coord) => Origin::Custom(*coord),
//             }
//         }
//     } else if keyboard_input.just_pressed(KeyCode::KeyF) {
//         for mut visible in &mut ui_query {
//             *visible = match *visible {
//                 Visibility::Hidden => Visibility::Inherited,
//                 Visibility::Visible | Visibility::Inherited => Visibility::Hidden,
//             };
//         }
//     }
// }

// #[derive(Component)]
// struct FpsText;

// #[derive(Component)]
// struct FpsMinText;

// #[derive(Component)]
// struct FpsMaxText;

// #[derive(Component)]
// struct FrameTimeText;

// #[derive(Component)]
// struct FpsTextRoot;

// #[derive(Resource)]
// struct FpsValues {
//     min: f64,
//     max: f64,
// }

// impl Default for FpsValues {
//     fn default() -> Self {
//         Self {
//             min: 10000.0,
//             max: 0.0,
//         }
//     }
// }

// fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
//     let font_medium = asset_server.load("fonts/FiraMono-Medium.ttf");

//     commands
//         .spawn((
//             Text::default(),
//             TextColor::WHITE,
//             TextFont::from_font(font_medium).with_font_size(20.0),
//             Node {
//                 position_type: PositionType::Absolute,
//                 top: Val::Px(5.0),
//                 left: Val::Px(15.0),
//                 ..default()
//             },
//             FpsTextRoot,
//         ))
//         .with_children(|commands| {
//             commands.spawn((
//                 TextSpan::new("FPS: "),
//                 TextFont::from_font(font_bold.clone()).with_font_size(30.0),
//             ));
//             commands.spawn((
//                 TextSpan::default(),
//                 TextFont::from_font_size(30.0),
//                 TextColor::from(GOLD),
//                 FpsText,
//             ));
//             commands.spawn((
//                 TextSpan::new("\n(min: "),
//                 TextFont::from_font(font_bold.clone()),
//             ));
//             commands.spawn((TextSpan::default(), TextColor::from(GOLD), FpsMinText));
//             commands.spawn((
//                 TextSpan::new(" - max: "),
//                 TextFont::from_font(font_bold.clone()),
//             ));
//             commands.spawn((TextSpan::default(), TextColor::from(GOLD), FpsMaxText));
//             commands.spawn((TextSpan::new(")"), TextFont::from_font(font_bold.clone())));
//             commands.spawn((
//                 TextSpan::new("\nms/frame: "),
//                 TextFont::from_font(font_bold.clone()).with_font_size(30.0),
//             ));
//             commands.spawn((
//                 TextSpan::default(),
//                 TextFont::from_font_size(30.0),
//                 TextColor::from(GREEN),
//                 FrameTimeText,
//             ));
//         });
// }

// fn fps_text_update_system(
//     diagnostics: Res<DiagnosticsStore>,
//     mut fps_values: Local<FpsValues>,
//     mut query: ParamSet<(
//         Query<&mut TextSpan, With<FpsText>>,
//         Query<&mut TextSpan, With<FpsMinText>>,
//         Query<&mut TextSpan, With<FpsMaxText>>,
//         Query<&mut TextSpan, With<FrameTimeText>>,
//     )>,
// ) {
//     if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
//         if let Some(fps_smoothed) = fps.smoothed() {
//             if let Ok(mut text) = query.p0().get_single_mut() {
//                 *text.write_span() = format!("{fps_smoothed:.2}");
//             }
//             fps_values.min = fps_values.min.min(fps_smoothed);
//             if let Ok(mut text) = query.p1().get_single_mut() {
//                 *text.write_span() = format!("{:.2}", fps_values.min);
//             }
//             fps_values.max = fps_values.max.max(fps_smoothed);
//             if let Ok(mut text) = query.p2().get_single_mut() {
//                 *text.write_span() = format!("{:.2}", fps_values.max);
//             }
//         }
//     }
//     if let Some(frame_time) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
//         if let Some(frame_time_smoothed) = frame_time.smoothed() {
//             if let Ok(mut text) = query.p3().get_single_mut() {
//                 *text.write_span() = format!("{frame_time_smoothed:.2}");
//             }
//         }
//     }
// }

// #[derive(Component)]
// struct OriginText;

// #[derive(Component)]
// struct OriginTextRoot;

// fn setup_origin_text(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
//     let font_medium = asset_server.load("fonts/FiraMono-Medium.ttf");

//     commands
//         .spawn((
//             Text::default(),
//             TextColor::WHITE,
//             TextFont::from_font(font_medium).with_font_size(20.0),
//             Node {
//                 position_type: PositionType::Absolute,
//                 bottom: Val::Px(5.0),
//                 left: Val::Px(15.0),
//                 ..default()
//             },
//             OriginTextRoot,
//         ))
//         .with_children(|commands| {
//             commands.spawn((TextSpan::new("Origin: "), TextFont::from_font(font_bold)));
//             commands.spawn((TextSpan::default(), TextColor::from(GOLD), OriginText));
//         });
// }

// fn origin_text_update_system(
//     mut text_query: Query<&mut TextSpan, With<OriginText>>,
//     query: Query<&Origin>,
// ) {
//     for mut text in &mut text_query {
//         if let Some(origin) = query.iter().next() {
//             *text.write_span() = format!("{origin:?}");
//         }
//     }
// }

pub fn camera_zoom_system(
    // mut evr_scroll: EventReader<MouseWheel>,
    mut camera: Query<(Option<Mut<OrthographicProjection>>, Mut<Transform>), With<Camera>>,
    mut evr_gesture_pinch: EventReader<PinchGesture>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    // mut evr_gesture_rotate: EventReader<RotationGesture>,
) {
    for ev in evr_gesture_pinch.read() {
        // Positive numbers are zooming in
        // Negative numbers are zooming out
        // println!("Two-finger zoom by {}", ev.0);

        let current_cursor_pos = q_windows.single().cursor_position().unwrap();

        for (projection, mut transform) in camera.iter_mut() {
            // let amount = match ev.unit {
            //     MouseScrollUnit::Line => ev.y,
            //     MouseScrollUnit::Pixel => ev.y,
            // };

            let amount = ev.0;

            if let Some(mut projection) = projection {
                projection.scale -= amount;
                projection.viewport_origin = Vec2::new(
                    current_cursor_pos.x / q_windows.single().width(),
                    current_cursor_pos.y / q_windows.single().height(),
                );
                projection.scale = projection.scale.clamp(0.001, 3.0);
            } else {
                transform.translation.z -= amount;
            }
        }
    }
    // for ev in evr_scroll.read() {

    // }
}

pub fn camera_pan_system(
    // input: Res<Drag<KeyCode>>,
    mut camera: Query<Mut<Transform>, With<Camera>>,
    // mut evr_motion: EventReader<MouseMotion>,
    // buttons: Res<ButtonInput<MouseButton>>,
    // mut evr_gesture_pan: EventReader<PanGesture>,
    // mut evr_gesture_pan: EventReader<PanGesture>,
    // mut evr_gesture_doubletap: EventReader<PanGesture>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    // for ev_pinch in evr_gesture_pinch.read() {
    //     // Positive numbers are zooming in
    //     // Negative numbers are zooming out
    //     println!("Two-finger zoom by {}", ev_pinch.0);
    // }
    // for ev_rotate in evr_gesture_rotate.read() {
    //     // Positive numbers are anticlockwise
    //     // Negative numbers are clockwise
    //     println!("Two-finger rotate by {}", ev_rotate.0);
    // }
    // for ev_pan in evr_gesture_pan.read() {
    //     // Each event is a Vec2 giving you the X/Y pan amount
    //     println!("Two-finger pan by X: {}, Y: {}", ev_pan.0.x, ev_pan.0.y);
    // }
    // for ev_doubletap in evr_gesture_doubletap.read() {
    //     // This one has no data
    //     println!("Double-Tap gesture!");
    // }

    for mut transform in camera.iter_mut() {
        // evr_gesture_pan.read().for_each(|ev| {

        // });
        // if !buttons.pressed(MouseButton::Right) {
        //     continue;
        // }

        let mut total_motion: Vec2 = evr_scroll.read().map(|ev| Vec2::new(ev.x, ev.y)).sum();

        // println!("Total motion: {:?}", total_motion);

        if total_motion == Vec2::ZERO {
            continue;
        }

        total_motion.x = -total_motion.x;

        println!(
            "Mouse moved: X: {} px, Y: {} px",
            total_motion.x, total_motion.y
        );

        transform.translation.x += total_motion.x;
        transform.translation.y += total_motion.y;

        // if input.pressed(KeyCode::KeyW) {
        //     transform.translation.y += 1.0;
        // }
        // if input.pressed(KeyCode::KeyS) {
        //     transform.translation.y -= 1.0;
        // }
        // if input.pressed(KeyCode::KeyA) {
        //     transform.translation.x -= 1.0;
        // }
        // if input.pressed(KeyCode::KeyD) {
        //     transform.translation.x += 1.0;
        // }
    }
}
