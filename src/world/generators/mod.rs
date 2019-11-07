use voxel::subdivmath::OctPos;
use voxel::subdivstorage::{NaiveVoxelOctree, NaiveOctreeNode, SubdivVoxelDrain};
use cgmath::{Vector3, MetricSpace};
use noise::{NoiseFn, Perlin, Seedable};
use world::{CHUNK_SIZE, CHUNK_SIZE_F32};

pub trait ChunkGenerator {
    fn generate(&self, pos: (i32, i32, i32)) -> NaiveVoxelOctree<u8, ()>;
}

#[allow(dead_code)]
pub struct SphereGenerator {
    seed: u32,
}

#[allow(dead_code)]
impl SphereGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            seed
        }
    }
}

#[allow(dead_code)]
impl ChunkGenerator for SphereGenerator {
    fn generate(&self, _pos: (i32, i32, i32)) -> NaiveVoxelOctree<u8, ()> {
        let mut tree = NaiveVoxelOctree{scale : 6 , root: NaiveOctreeNode::new_leaf(0)};

        let center = Vector3::new(CHUNK_SIZE_F32, CHUNK_SIZE_F32, CHUNK_SIZE_F32);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let pos = Vector3::new(x as f32, y as f32, z as f32);
                    let dist = pos.distance(center);
                    if dist < 15.0 {
                        tree.set(opos!((x, y, z) @ 0), 1).unwrap();
                    }
                }
            }
        }

        tree
    }
}

pub struct PerlinGenerator {
    perlin: Perlin,
    scale: f64,
    offset: f64,
    block_type_noise: Perlin,
    block_type_scale: f64,
}

impl PerlinGenerator {
    pub fn new(seed: u32) -> PerlinGenerator {
        let perlin = Perlin::new();
        perlin.set_seed(seed);

        let block_type_noise = Perlin::new();
        perlin.set_seed(seed*51);

        PerlinGenerator {
            perlin,
            scale: 0.008126,
            offset: 0.26378,
            block_type_noise,
            block_type_scale: 0.043647,
        }
    }
}

impl ChunkGenerator for PerlinGenerator {
    fn generate(&self, pos: (i32, i32, i32)) -> NaiveVoxelOctree<u8, ()> {
        let mut tree = NaiveVoxelOctree{scale : 6 , root: NaiveOctreeNode::new_leaf(0)};

        const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;

        for x in 0..CHUNK_SIZE_I32 {
            for z in 0..CHUNK_SIZE_I32 {
                let height_norm = self.perlin.get([((pos.0 * CHUNK_SIZE_I32 + x) as f64 + self.offset) * self.scale, ((pos.2 * CHUNK_SIZE_I32 + z) as f64 + self.offset) * self.scale]) / 2.0 + 0.5;
                let height_abs = height_norm as f32 * 32.0;

                for y in 0..CHUNK_SIZE_I32 {
                    if (pos.1 as f32 * CHUNK_SIZE_F32) + y as f32 <= height_abs {
                        let block_type_val = self.block_type_noise.get([((pos.0 * CHUNK_SIZE_I32 + x) as f64) * self.block_type_scale, ((pos.2 * CHUNK_SIZE_I32 + z) as f64) * self.block_type_scale]) / 2.0 + 0.5;
                        let block_id = ((block_type_val * 3.0) + 1.0) as u8;
                        tree.set(opos!((x, y, z) @ 0), block_id).unwrap();
                    }
                }
            }
        }

        tree
    }
}