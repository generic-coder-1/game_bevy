use bevy::{
    asset::RenderAssetUsages,
    platform::collections::HashMap,
    prelude::*,
    render::render_resource::{
        AsBindGroup, Extent3d, TextureDimension, TextureFormat, TextureUsages,
    },
    sprite::Material2d,
};

#[derive(Component, Deref, Hash, PartialEq, Eq, Clone)]
pub struct ChunkPos(IVec2);

impl From<IVec2> for ChunkPos {
    fn from(value: IVec2) -> Self {
        Self(value)
    }
}

#[derive(Clone, Deref)]
pub struct Chunk(Handle<Image>);

impl<'a> From<&'a Chunk> for Option<&'a Handle<Image>> {
    fn from(value: &'a Chunk) -> Self {
        Some(&value.0)
    }
}

impl Chunk {
    pub const CHUNK_SIZE: u8 = 32;

    pub fn empty(asset_server: Res<AssetServer>) -> Self {
        let image = Image::new_fill(
            Extent3d {
                width: Self::CHUNK_SIZE as u32,
                height: Self::CHUNK_SIZE as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &(0..((Self::CHUNK_SIZE as u32).pow(2)))
                .map(|i| (i % 9) as u8)
                .collect::<Vec<u8>>(),
            TextureFormat::R8Uint,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );
        Self(asset_server.add(image))
    }

    pub fn get_tile_at(&self, images: &ResMut<Assets<Image>>, x: u32, y: u32) -> Option<Tile> {
        ((images
            .get(self.0.id())?
            .get_color_at(x, y)
            .ok()?
            .to_linear()
            .red
            * u8::MAX as f32)
            .round() as u8)
            .try_into()
            .ok()
    }

    pub fn set_tile_at(&self, images: &mut ResMut<Assets<Image>>, x: u32, y: u32, tile: Tile) {
        let Some(image) = (*images).get_mut(self.0.id()) else {
            return;
        };
        let color = Into::<u8>::into(tile) as f32 / u8::MAX as f32;
        let _ = image.set_color_at(x, Self::CHUNK_SIZE as u32 - y - 1, Color::linear_rgb(color, color, color));
    }
}

#[derive(AsBindGroup, Asset, Clone, TypePath)]
pub struct ChunkMaterial {
    #[texture(0, sample_type = "u_int")]
    pub tile_data: Chunk,
    #[texture(1)]
    pub tile_atlas: Handle<Image>,
}

impl ChunkMaterial {
    fn new(asset_server: Res<AssetServer>) -> Self {
        Self {
            tile_atlas: asset_server.load("sprites/sim_tiles.png"),
            tile_data: Chunk::empty(asset_server),
        }
    }
}

impl Material2d for ChunkMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shader/tilemap.wgsl".into()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Elevator,
    Block,
    Flat,
    Right,
    Left,
    Hold, //might replace later
    Conditional,
    Duplicate,
    Spike,
    Empty,
}

impl From<Tile> for u8 {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Elevator => 0,
            Tile::Block => 1,
            Tile::Flat => 2,
            Tile::Right => 3,
            Tile::Left => 4,
            Tile::Hold => 5,
            Tile::Conditional => 6,
            Tile::Duplicate => 7,
            Tile::Spike => 8,
            Tile::Empty => 9,
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = ();

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0 => Tile::Elevator,
            1 => Tile::Block,
            2 => Tile::Flat,
            3 => Tile::Right,
            4 => Tile::Left,
            5 => Tile::Hold,
            6 => Tile::Conditional,
            7 => Tile::Duplicate,
            8 => Tile::Spike,
            9 => Tile::Empty,
            _ => Err(())?,
        })
    }
}

#[derive(Resource, Default)]
pub struct Chunks {
    pub chunks: HashMap<ChunkPos, Handle<ChunkMaterial>>,
}

pub fn chunk_bundle(
    pos: ChunkPos,
    mut meshs: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ChunkMaterial>>,
    mut chunks: ResMut<Chunks>,
    asset_server: Res<AssetServer>,
) -> (Mesh2d, MeshMaterial2d<ChunkMaterial>, Transform) {
    let mat = material.add(ChunkMaterial::new(asset_server));
    chunks.chunks.insert(pos.clone(), mat.clone());
    (
        Mesh2d(meshs.add(Rectangle::from_length(Chunk::CHUNK_SIZE as f32))),
        MeshMaterial2d(mat),
        Transform::from_xyz(
            (pos.x as f32 + 0.5) * Chunk::CHUNK_SIZE as f32,
            (pos.y as f32 + 0.5) * Chunk::CHUNK_SIZE as f32,
            0.0,
        ),
    )
}

#[derive(Bundle)]
pub struct ChunkBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ChunkMaterial>,
    transform: Transform,
}
