use self::mesher::{get_mesh, Mesh};

use super::block::Block;
use cgmath::{num_traits::abs, Point3};
use image::buffer;
use noise::{utils::*, Fbm, Perlin};
use std::collections::HashMap;
use wgpu::Device;
pub mod instance;
pub(crate) mod mesher;
pub mod vertex;

pub const CHUNK_SIZE: usize = 16;

pub type Chunk = HashMap<Point3<u8>, Block>;

fn gen_chunk(
    chunk_x: isize,
    chunk_y: isize,
    chunk_z: isize,
    perlin: &Fbm<Perlin>,
) -> HashMap<Point3<u8>, Block> {
    let mut voxels: HashMap<Point3<u8>, Block> = HashMap::new();

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
                //let sample_x = x as isize + chunk_x * CHUNK_SIZE as isize;
                //let sample_z = z as isize + chunk_z * CHUNK_SIZE as isize;
                let global_y = y as isize + chunk_y * CHUNK_SIZE as isize;
                let generated_id;

                let val1 = map.get_value(x, z);
                let val2 = big_map.get_value(x, z);
                let depth = val1 * 4.0 + val2 * 8.0 - global_y as f64 + 0.0;

                if depth < 0.0 && global_y < 0 {
                    generated_id = 0;
                } else if depth < 0.0 {
                    continue;
                } else if depth < 1.0 && global_y < 2 {
                    generated_id = 4;
                } else if depth < 1.0 {
                    generated_id = 1;
                } else if depth < 2.0 {
                    generated_id = 2;
                } else {
                    generated_id = 3;
                }

                voxels.insert(
                    [x as u8, y as u8, z as u8].into(),
                    Block {
                        block_id: generated_id as u8,
                        block_state: 0,
                    },
                );
            }
        }
    }
    voxels
}

pub struct World {
    pub chunks: HashMap<Point3<isize>, Chunk>,
    pub meshes: HashMap<Point3<isize>, mesher::Mesh>,
    pub dirty: Vec<Point3<isize>>,
}

impl World {
    pub fn new(world_size: isize, device: &Device) -> Self {
        let perlin = Fbm::<Perlin>::new(1);

        let mut chunks = HashMap::new();
        chunks.insert([0, 0, 0].into(), HashMap::new());

        let todo = world_size * (world_size + 1) * 6 - world_size * 2;
        for x in -world_size..=world_size {
            for y in -1..=1 {
                for z in -world_size..=world_size {
                    chunks.insert([x, y, z].into(), gen_chunk(x, y, z, &perlin));
                    let progress =
                        (z + world_size) + (y + 1) * world_size + (x + world_size) * world_size * 3;
                    println!("Generating terrain: {progress}/{todo}")
                }
            }
        }

        let mut meshes = HashMap::new();

        for x in -world_size..=world_size {
            for y in -1..=1 {
                for z in -world_size..=world_size {
                    meshes.insert(
                        [x, y, z].into(),
                        mesher::get_mesh(&chunks, [x, y, z].into(), device),
                    );
                    let progress =
                        (z + world_size) + (y + 1) * world_size + (x + world_size) * world_size * 3;
                    println!("Generating mesh: {progress}/{todo}")
                }
            }
        }

        World {
            chunks,
            meshes,
            dirty: Vec::new(),
        }
    }
    pub fn get_chunk(&self, x: isize, y: isize, z: isize) -> &Chunk {
        if self.chunks.contains_key(&[x, y, z].into()) {
            self.chunks.get(&[x, y, z].into()).unwrap()
        } else {
            self.chunks.get(&[0, 0, 0].into()).unwrap()
        }
    }
    pub fn block_exists(&self, position: Point3<f32>) -> bool {
        let (chunk_pos, relative_pos) = self.chunk_block_from_global(position);
        if !self.chunks.contains_key(&chunk_pos) {
            return false;
        }
        if self
            .chunks
            .get(&chunk_pos)
            .unwrap()
            .contains_key(&relative_pos)
        {
            return true;
        }
        return false;
    }

    pub fn remove_block(&mut self, position: Point3<f32>) {
        let (chunk_pos, block_pos) = self.chunk_block_from_global(position);

        if self.block_exists(position) {
            self.chunks.get_mut(&chunk_pos).unwrap().remove(&block_pos);
            self.dirty.push(chunk_pos);
        }
    }

    fn chunk_block_from_global(&self, position: Point3<f32>) -> (Point3<isize>, Point3<u8>) {
        let chunk_pos = [
            (position.x / CHUNK_SIZE as f32).floor() as isize,
            (position.y / CHUNK_SIZE as f32).floor() as isize,
            (position.z / CHUNK_SIZE as f32).floor() as isize,
        ]
        .into();
        let block_pos = [
            (((position.x % CHUNK_SIZE as f32) + CHUNK_SIZE as f32) % CHUNK_SIZE as f32) as u8,
            (((position.y % CHUNK_SIZE as f32) + CHUNK_SIZE as f32) % CHUNK_SIZE as f32) as u8,
            (((position.z % CHUNK_SIZE as f32) + CHUNK_SIZE as f32) % CHUNK_SIZE as f32) as u8,
        ]
        .into();
        (chunk_pos, block_pos)
    }
}
