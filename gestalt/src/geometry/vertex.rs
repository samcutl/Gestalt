//! Vertex types.


/// A vertex type with position, normal, uv, and color data.
#[derive(Debug, Clone, Default)]
pub struct VertexPositionNormalUVColor {
    pub position: [f32; 3],
    pub normal:   [f32; 3],
    pub uv:       [f32; 2],
    pub color:    [f32; 3]
}
impl_vertex!(VertexPositionNormalUVColor, position, normal, uv, color);


/// A vertex type with position, normal, uv, tangent, and color data.
#[derive(Debug, Clone, Default)]
pub struct VertexPositionNormalTangentUVColor {
    pub position: [f32; 3],
    pub normal:   [f32; 3],
    pub tangent:  [f32; 3],
    pub uv:       [f32; 2],
    pub color:    [f32; 3]
}
impl_vertex!(VertexPositionNormalTangentUVColor, position, normal, tangent, uv, color);


/// Vertex type for voxel meshes: position, normal, tangent, uv, color, and edge adjacency data.
#[derive(Debug, Clone, Default)]
pub struct VoxelVertex {
    pub position:  [f32; 3],
    pub normal:    [f32; 3],
    pub tangent:   [f32; 3],
    pub binormal:  [f32; 3],
    pub uv:        [f32; 2],
    pub color:     [f32; 3],
    pub adjacency: u32,
}
impl_vertex!(VoxelVertex, position, normal, tangent, binormal, uv, color, adjacency);


/// Vertex type for pbr pipeline: position, normal, tangent, and uv data.
#[derive(Debug, Clone, Default)]
pub struct PBRPipelineVertex {
    pub position:  [f32; 3],
    pub normal:    [f32; 3],
    pub tangent:   [f32; 3],
    pub uv:        [f32; 2]
}
impl_vertex!(PBRPipelineVertex, position, normal, tangent, uv);


/// A vertex type with position and color + alpha data.
#[derive(Debug, Clone, Default)]
pub struct VertexPositionColorAlpha {
    pub position: [f32; 3],
    pub color:    [f32; 4]
}
impl_vertex!(VertexPositionColorAlpha, position, color);


/// A vertex type with position data.
#[derive(Debug, Clone, Default)]
pub struct VertexPosition {
    pub position: [f32; 3]
}
impl_vertex!(VertexPosition, position);


/// A vertex type with position and uv data.
#[derive(Debug, Clone, Default)]
pub struct VertexPositionUV {
    pub position: [f32; 3],
    pub uv:       [f32; 2]
}
impl_vertex!(VertexPositionUV, position, uv);


/// A vertex type with position, uv, and color data.
#[derive(Debug, Clone, Default)]
pub struct VertexPositionUVColor {
    pub position: [f32; 3],
    pub uv:       [f32; 2],
    pub color:    [f32; 4]
}
impl_vertex!(VertexPositionUVColor, position, uv, color);