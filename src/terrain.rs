use super::block::Block;
use noise::{NoiseFn, Perlin, Seedable};
pub type Chunk = [[[Block; 16]; 16]; 16];

pub fn get_chunk(chunk_x: i32, chunk_y: i32) -> Chunk {
    let perlin = Perlin::new(1);

    let mut chunk: Chunk = [[[Block {
        block_id: 0,
        block_state: 0,
    }; 16]; 16]; 16];

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                chunk[x][y][z] = Block {
                    block_id: perlin.get([x as f64, y as f64, z as f64]).ceil() as u8,
                    block_state: 0,
                };
            }
        }
    }
    chunk
}
