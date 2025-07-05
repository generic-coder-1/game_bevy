use std::ops::Range;

use bevy::{
    app::{Plugin, Update},
    ecs::query::With,
    input::{common_conditions::input_pressed, mouse::MouseWheel},
    prelude::*,
    state::condition::in_state,
};

use crate::{mouse_management::{MousePosition, PreviousMousePosition}, AppState};

pub struct CameraMovemntPlugin;

impl Plugin for CameraMovemntPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
                Update,
                move_camera
                    .run_if(in_state(AppState::Editing))
                    .run_if(input_pressed(MouseButton::Left))
                    .run_if(input_pressed(KeyCode::ShiftLeft)),
            )
            .add_systems(Update, zoom_system);
    }
}

fn zoom_system(
    mut camera: Query<(&mut Transform, &Camera), With<Camera2d>>,
    mut mouse_wheel_input: EventReader<MouseWheel>,
    cur_pos: Res<MousePosition>,
) {
    const CAMERA_ZOOM_SPEED: f32 = 10.0;
    const CAMERA_ZOOM_RANGE: Range<f32> = 0.1..10.0;
    let delta_zoom: f32 = mouse_wheel_input.read().map(|e| e.y).sum();
    if delta_zoom == 0. {
        return;
    }

    let (mut transform, camera) = camera.single_mut().expect("camera exists");
    let mouse_world_pos = camera
        .viewport_to_world_2d(&GlobalTransform::from(*transform), cur_pos.pos)
        .expect("valid viewport");
    let scale_factor = (transform.scale.x * 2.0_f32.powf(-delta_zoom / CAMERA_ZOOM_SPEED))
        .clamp(CAMERA_ZOOM_RANGE.start, CAMERA_ZOOM_RANGE.end)
        / transform.scale.x;
    transform.scale *= scale_factor;
    let g_tranform = GlobalTransform::from(*transform);
    let mouse_world_pos_post = camera
        .viewport_to_world_2d(&g_tranform, cur_pos.pos)
        .expect("valid viewport");
    //transform.translation += (camera
    //    .world_to_viewport(&g_tranform, mouse_world_pos.extend(0.0))
    //    .expect("valid viewport")
    //    - camera
    //        .world_to_viewport(&g_tranform, mouse_world_pos_post.extend(0.0))
    //        .expect("valid viewport"))
    //.extend(0.0);
    transform.translation += (mouse_world_pos - mouse_world_pos_post).extend(0.0);
}

fn move_camera(
    cur_pos: Res<MousePosition>,
    prev_pos: Res<PreviousMousePosition>,
    mut camera_transform: Query<(&mut Transform, &Camera, &GlobalTransform), With<Camera2d>>,
) {
    let (mut transform, camera, camera_global_transform) =
        camera_transform.single_mut().expect("camera should exist");

    let moved_by = camera
        .viewport_to_world_2d(camera_global_transform, cur_pos.pos - prev_pos.pos)
        .expect("projection is valid")
        - camera
            .viewport_to_world_2d(camera_global_transform, Vec2::ZERO)
            .expect("projection is valid");
    transform.translation -= moved_by.extend(0.0);
}
