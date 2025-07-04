use bevy::{
    app::{Plugin, Update},
    ecs::query::With,
    input::
        common_conditions::input_pressed
    ,
    prelude::*,
    state::condition::in_state,
};

use crate::AppState;

pub struct CameraMovemntPlugin;

#[derive(Resource)]
pub struct MousePosition {
    pub pos: Vec2,
}

#[derive(Resource)]
pub struct PreviousMousePosition {
    pub pos: Vec2,
}

impl Plugin for CameraMovemntPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(MousePosition { pos: Vec2::ZERO })
            .insert_resource(PreviousMousePosition { pos: Vec2::ZERO })
            .add_systems(PreUpdate, update_mouse_pos)
            .add_systems(PostUpdate, update_prev_mouse_pos)
            .add_systems(
                Update,
                move_camera
                    .run_if(in_state(AppState::Editing))
                    .run_if(input_pressed(MouseButton::Left)),
            );
    }
}

fn update_mouse_pos(
    mut cur_pos: ResMut<MousePosition>,
    mut curosr_moved: EventReader<CursorMoved>,
) {
    let Some(cursor) = curosr_moved.read().last() else {
        return;
    };

    *cur_pos = MousePosition {
        pos: cursor.position,
    };
}

fn update_prev_mouse_pos(cur_pos: Res<MousePosition>, mut prev_pos: ResMut<PreviousMousePosition>) {
    prev_pos.pos = cur_pos.pos;
}

fn move_camera(
    cur_pos: Res<MousePosition>,
    prev_pos: Res<PreviousMousePosition>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut camera_transform: Query<&mut Transform, With<Camera2d>>,
) {
    let (camera, camera_global_transform) = camera_query.single().expect("camera should exist");

    let moved_by = camera
        .viewport_to_world_2d(camera_global_transform, cur_pos.pos-prev_pos.pos)
        .expect("projection is valid")
        - camera
            .viewport_to_world_2d(camera_global_transform, Vec2::ZERO) 
            .expect("projection is valid");
    camera_transform
        .single_mut()
        .expect("camera exists")
        .translation -= moved_by.extend(0.0);
}
