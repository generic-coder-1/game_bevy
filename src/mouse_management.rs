use bevy::{app::Plugin, ecs::resource::Resource, prelude::*};

pub struct MouseManagementPlugin;

#[derive(Resource)]
pub struct MousePosition {
    pub pos: Vec2,
}

#[derive(Resource)]
pub struct PreviousMousePosition {
    pub pos: Vec2,
}

impl Plugin for MouseManagementPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(MousePosition { pos: Vec2::ZERO })
            .insert_resource(PreviousMousePosition { pos: Vec2::ZERO })
            .add_systems(PreUpdate, update_mouse_pos)
            .add_systems(PostUpdate, update_prev_mouse_pos);
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
