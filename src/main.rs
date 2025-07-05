use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use editor::EditorPlugin;
use mouse_management::MouseManagementPlugin;
mod chunk;
mod editor;
mod ui;
mod camera_movement;
mod mouse_management;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "My Bevy Application".to_owned(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()), //because we are using pixel art, we want nearest neighbor
        )
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((ui::UiPlugin, EditorPlugin, MouseManagementPlugin))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                Camera2d,
                Projection::Orthographic(OrthographicProjection {
                    scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                        min_width: 800.0,
                        min_height: 600.0,
                    },
                    ..OrthographicProjection::default_2d()
                }),
            ));
        })
        .add_systems(Update, fullscreen_system)
        .init_state::<AppState>()
        .run();
}

fn fullscreen_system(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if input.just_pressed(KeyCode::F11) {
        let mut window = query.single_mut().expect("Main Window Exists");
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            WindowMode::Fullscreen(_, _) | WindowMode::BorderlessFullscreen(_) => {
                WindowMode::Windowed
            }
        };
    }
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    #[default]
    MainMenu,
    Editing,
}
