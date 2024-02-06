use super::block::Block;
use noise::{
    core::perlin::{perlin_2d, perlin_3d, perlin_4d},
    permutationtable::PermutationTable,
    utils::*,
};

pub const CHUNK_SIZE: usize = 32;

pub type Chunk = [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

pub fn get_chunk(chunk_x: i32, chunk_y: i32, chunk_z: i32) -> Chunk {
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
                let k = 10;
                let sample_x = x as i32 + chunk_x * CHUNK_SIZE as i32;
                let sample_y = y as i32 + chunk_y * CHUNK_SIZE as i32;
                let sample_z = z as i32 + chunk_z * CHUNK_SIZE as i32;
                let mut generated_id;
                let depth = (map.get_value(sample_x as usize * k, sample_z as usize * k) + 1.0)
                    * 16.0
                    - y as f64;
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
