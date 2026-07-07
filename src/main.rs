use bevy::{camera::ScalingMode, input::keyboard::Key, prelude::*, window::WindowMode};

use crate::chess::ChessPlugin;

mod chess;

const WINDOW_SIZE: Vec2 = vec2(16.0 / 9.0 * 480.0, 480.0);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".into(),
                name: Some("chess".into()),
                resolution: (WINDOW_SIZE.x as u32 * 2, WINDOW_SIZE.y as u32 * 2).into(),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()), ChessPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, check_fullscreen)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::Fixed { width: WINDOW_SIZE.x, height: WINDOW_SIZE.y },
        ..OrthographicProjection::default_2d()
    })));
}

fn check_fullscreen(input: Res<ButtonInput<Key>>, mut window: Single<&mut Window>) {
    if input.just_released(Key::F11) {
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            WindowMode::BorderlessFullscreen(_) => WindowMode::Windowed,
            WindowMode::Fullscreen(_, _) => unreachable!(),
        };
    }
}
