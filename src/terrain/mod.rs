use super::block::Block;
use cgmath::Vector3;
use noise::{
    core::perlin::{perlin_2d, perlin_3d, perlin_4d},
    permutationtable::PermutationTable,
    utils::*,
};
use std::collections::HashMap;
use wgpu::{core::device, Device};
pub mod instance;
mod mesher;
pub mod vertex;

pub const CHUNK_SIZE: usize = 32;

pub type Chunk = [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

fn gen_chunk(chunk_x: isize, chunk_y: isize, chunk_z: isize) -> Chunk {
    let hasher = PermutationTable::new(1);
    let map = PlaneMapBuilder::new_fn(|point, hasher| perlin_3d(point.into(), hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build();
    let mut chunk: Chunk = [[[Block {
        block_id: 0,
        block_state: 0,
    }; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let sample_x = x as isize + chunk_x * CHUNK_SIZE as isize;
                let sample_y = y as isize + chunk_y * CHUNK_SIZE as isize;
                let sample_z = z as isize + chunk_z * CHUNK_SIZE as isize;
                let k = 10;
                let generated_id;
                let depth = 16.0 * map.get_value(sample_x as usize * k, sample_z as usize * k)
                    + 16.0
                    - (y as isize + chunk_y * CHUNK_SIZE as isize) as f64;
                //let depth =
                //   perlin_3d([sample_x as f64, 0.0, sample_z as f64], &hasher) * 16.0 - y as f64;
                if depth < 0.0 {
                    generated_id = 0;
                } else if depth < 1.0 {
                    generated_id = 1;
                } else if depth < 2.0 {
                    generated_id = 2;
                } else {
                    generated_id = 3;
                }

                chunk[x][y][z] = Block {
                    block_id: generated_id as u8,
                    block_state: 0,
                };
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
        let mut chunks = HashMap::new();

        for x in 0..world_size {
            for y in 0..world_size {
                for z in 0..world_size {
                    chunks.insert([x, y, z], gen_chunk(x, y, z));
                }
            }
        }

        let mut meshes = Vec::new();

        for x in 0..world_size {
            for y in 0..world_size {
                for z in 0..world_size {
                    meshes.push(mesher::get_mesh(
                        chunks.get(&[x, y, z]).unwrap(),
                        Vector3{x:x, y:y, z:z},
                        device,
                    ));
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
