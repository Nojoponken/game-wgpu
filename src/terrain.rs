use super::block::Block;
use cgmath::num_traits::Float;
use noise::{
    core::perlin::{perlin_2d, perlin_3d, perlin_4d},
    permutationtable::PermutationTable,
    utils::*,
};

pub type Chunk = [[[Block; 16]; 16]; 16];

pub fn get_chunk(chunk_x: i32, chunk_y: i32) -> Chunk {
    let hasher = PermutationTable::new(0);
    let map = PlaneMapBuilder::new_fn(|point, hasher| perlin_3d(point.into(), hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build();
    let mut chunk: Chunk = [[[Block {
        block_id: 0,
        block_state: 0,
    }; 16]; 16]; 16];

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let k = 10;
                let generated_id = (map.get_value(x * k, z * k) + 1.0) * 4.0 > y as f64;
                chunk[x][y][z] = Block {
                    block_id: generated_id as u8,
                    block_state: 0,
                };
            }
        }
    }

    chunk
}
