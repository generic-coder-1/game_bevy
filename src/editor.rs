use bevy::{
    app::Plugin,
    asset::{AssetServer, Assets},
    ecs::system::{Commands, Res, ResMut},
    prelude::*,
    sprite::Material2dPlugin,
};

use crate::{
    chunk::{chunk_bundle, ChunkMaterial, Chunks},
    AppState,
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(Material2dPlugin::<ChunkMaterial>::default())
            .add_systems(
                OnEnter(AppState::Editing),
                (create_chunks_resource, create_ball).chain(),
            );
    }
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
        uvec2(0, 0).into(),
        meshs,
        material,
        chunks,
        asset_server,
    ));
}
