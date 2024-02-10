use super::block::Block;
use noise::{
    core::perlin::{self, perlin_2d, perlin_3d, perlin_4d},
    permutationtable::PermutationTable,
    utils::*,
    Fbm, NoiseFn, Perlin,
};
use std::collections::HashMap;
use wgpu::Device;
pub mod instance;
mod mesher;
pub mod vertex;

pub const CHUNK_SIZE: usize = 16;

pub type Chunk = HashMap<(u8, u8, u8), Block>; //[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

fn gen_chunk(chunk_x: isize, chunk_y: isize, chunk_z: isize, perlin: &Fbm<Perlin>) -> Chunk {
    let mut chunk: Chunk = HashMap::new();

    let k = 0.125;
    let k2 = k * 2.0;
    let map = PlaneMapBuilder::<_, 2>::new(perlin)
        .set_size(CHUNK_SIZE, CHUNK_SIZE)
        .set_x_bounds(
            -k2 + 2.0 * k2 * chunk_x as f64,
            k2 + 2.0 * k2 * chunk_x as f64,
        )
        .set_y_bounds(
            -k2 + 2.0 * k2 * chunk_z as f64,
            k2 + 2.0 * k2 * chunk_z as f64,
        )
        .build();
    let big_map = PlaneMapBuilder::<_, 2>::new(perlin)
        .set_size(CHUNK_SIZE, CHUNK_SIZE)
        .set_x_bounds(-k + 2.0 * k * chunk_x as f64, k + 2.0 * k * chunk_x as f64)
        .set_y_bounds(-k + 2.0 * k * chunk_z as f64, k + 2.0 * k * chunk_z as f64)
        .build();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let sample_x = x as isize + chunk_x * CHUNK_SIZE as isize;
                let sample_y = y as isize + chunk_y * CHUNK_SIZE as isize;
                let sample_z = z as isize + chunk_z * CHUNK_SIZE as isize;
                let generated_id;

                let val1 = map.get_value(x, z);
                let val2 = big_map.get_value(x, z);
                let depth = val1 * 4.0 + val2 * 8.0 - sample_y as f64;

                if depth < 0.0 && sample_y < 0 {
                    generated_id = 0;
                } else if depth < 0.0 {
                    continue;
                } else if depth < 1.0 && sample_y < 2 {
                    generated_id = 4;
                } else if depth < 1.0 {
                    generated_id = 1;
                } else if depth < 2.0 {
                    generated_id = 2;
                } else {
                    generated_id = 3;
                }

                chunk.insert(
                    (x as u8, y as u8, z as u8),
                    Block {
                        block_id: generated_id as u8,
                        block_state: 0,
                    },
                );
            }
        }
    }
    chunk
}

pub struct World {
    pub chunks: HashMap<[isize; 3], Chunk>,
    pub meshes: Vec<mesher::Mesh>,
}

impl World {
    pub fn new(world_size: isize, device: &Device) -> Self {
        let perlin = Fbm::<Perlin>::new(1);

        let mut chunks = HashMap::new();
        let todo = world_size * world_size * 3;
        for x in 0..world_size {
            for y in -1..=1 {
                for z in 0..world_size {
                    chunks.insert([x, y, z], gen_chunk(x, y, z, &perlin));
                    let progress = z + (y + 1) * world_size + x * world_size * 3 + 1;
                    println!("Generating terrain: {progress}/{todo}")
                }
            }
        }

        let mut meshes = Vec::new();

        for x in 0..world_size {
            for y in -1..=1 {
                for z in 0..world_size {
                    meshes.push(mesher::get_mesh(&chunks, [x, y, z], device));
                    let progress = z + (y + 1) * world_size + x * world_size * 3 + 1;
                    println!("Generating mesh: {progress}/{todo}")
                }
            }
        }

        Self { chunks, meshes }
    }
    pub fn get_chunk(&self, x: isize, y: isize, z: isize) -> &Chunk {
        if self.chunks.contains_key(&[x, y, z]) {
            self.chunks.get(&[x, y, z]).unwrap()
        } else {
            self.chunks.get(&[0, 0, 0]).unwrap()
        }
    }
}
