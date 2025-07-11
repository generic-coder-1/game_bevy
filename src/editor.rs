use bevy::{
    app::Plugin,
    asset::{AssetServer, Assets},
    ecs::system::{Commands, Res, ResMut},
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
    sprite::Material2dPlugin,
};

use crate::{
    camera_movement::CameraMovemntPlugin,
    chunk::{chunk_bundle, Chunk, ChunkMaterial, ChunkPos, Chunks, Tile},
    mouse_management::MousePosition,
    AppState,
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            Material2dPlugin::<ChunkMaterial>::default(),
            CameraMovemntPlugin,
        ))
        .add_systems(
            OnEnter(AppState::Editing),
            (create_chunks_resource, create_ball).chain(),
        )
        .add_systems(
            Update,
            change_tile
                .run_if(input_just_pressed(MouseButton::Left).or(
                        resource_changed::<MousePosition>.and(
                            input_pressed(MouseButton::Left)
                        )
                    ))
                .run_if(not(input_pressed(KeyCode::ShiftLeft)))
                .run_if(in_state(AppState::Editing)),
        );
    }
}

fn change_tile(
    camera: Query<(&Transform, &Camera), With<Camera2d>>,
    cur_pos: Res<MousePosition>,
    chunks: ResMut<Chunks>,
    mut chunk_data: ResMut<Assets<Image>>,
    mut chunk_material_data: ResMut<Assets<ChunkMaterial>>,
) {
    let (transform, camera) = camera.single().expect("camera is loaded");
    let world_pos = camera
        .viewport_to_world_2d(&GlobalTransform::from(*transform), cur_pos.pos)
        .expect("viewport is valid");
    let tile_pos = world_pos
        .rem_euclid(Vec2::splat(Chunk::CHUNK_SIZE as f32))
        .as_uvec2();
    let chunk_pos = world_pos.div_euclid(Vec2::splat(Chunk::CHUNK_SIZE as f32));

    let Some(chunk_material) = chunks
        .chunks
        .get(&Into::<ChunkPos>::into(chunk_pos.as_ivec2()))
    else {
        return;
    };

    let Some(chunk_material) = chunk_material_data.get_mut(chunk_material) else {
        return;
    };

    let chunk = chunk_material.tile_data.clone();

    chunk.set_tile_at(&mut chunk_data, tile_pos.x, tile_pos.y, Tile::Block);
}

fn create_chunks_resource(mut commads: Commands) {
    commads.insert_resource(Chunks::default());
}

fn create_ball(
    mut commads: Commands,
    meshs: ResMut<Assets<Mesh>>,
    material: ResMut<Assets<ChunkMaterial>>,
    asset_server: Res<AssetServer>,
    chunks: ResMut<Chunks>,
) {
    commads.spawn(chunk_bundle(
        ivec2(0, 0).into(),
        meshs,
        material,
        chunks,
        asset_server,
    ));
}
