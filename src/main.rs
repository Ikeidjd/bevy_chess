use bevy::{camera::ScalingMode, input::keyboard::Key, prelude::*, window::{PrimaryWindow, WindowMode}};

use crate::chess::ChessPlugin;

pub mod layers;
mod chess;

const WINDOW_SIZE: Vec2 = vec2(16.0 / 9.0 * 540.0, 540.0);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".into(),
                name: Some("chess".into()),
                resolution: (WINDOW_SIZE.x as u32, WINDOW_SIZE.y as u32).into(),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()), ChessPlugin))
        .init_state::<MainState>()
        .init_resource::<CursorWorldCoordinates>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (check_fullscreen, update_cursor_world_coordinates))
        .run();
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MainState {
    #[default]
    Chess,
}

#[derive(Resource, Default)]
pub struct CursorWorldCoordinates(Vec2);

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

fn update_cursor_world_coordinates(mut cursor: ResMut<CursorWorldCoordinates>, window: Single<&Window, With<PrimaryWindow>>, camera: Single<(&Camera, &GlobalTransform)>) {
    let (camera, camera_transform) = *camera;

    if let Some(position) = window.cursor_position().and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok()).map(|ray| ray.origin.truncate()) {
        cursor.0 = position;
    }
}
