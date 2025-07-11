#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var tile_data: texture_2d<u32>;

@group(2) @binding(1)
var tile_atlas: texture_2d<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let chunk_size = 32.0;
    let tile_size:u32 = 16u;
    let atlas_width = 3u;
    
    let tile_index = textureLoad(tile_data, vec2<u32>(in.uv*chunk_size), 0).x;
    let current_pixel = min(vec2<u32>(in.uv * vec2<f32>(textureDimensions(tile_data)) * f32(tile_size)), tile_size * textureDimensions(tile_data));

    let tile_col = tile_index%atlas_width;
    let tile_row = tile_index/atlas_width;
    let atlas_tile_offset = vec2<u32>(tile_col, tile_row)*tile_size;
    let atlas_uv:vec2<u32> = min(atlas_tile_offset + current_pixel%tile_size, textureDimensions(tile_atlas));


    let color = textureLoad(tile_atlas, atlas_uv, 0);
    if color.w<0.999{
      discard;
    }
    return color;
}
