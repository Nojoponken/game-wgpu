use super::vertex::*;
use crate::block::Block;
use crate::atlas;
const fn offset_face(face: [u16; 6], offset: u16) -> [u16; 6] {
    let [a, b, c, d, e, f] = face;
    [
        a + offset,
        b + offset,
        c + offset,
        d + offset,
        e + offset,
        f + offset,
    ]
}
type Chunk = [[[Block; 16]; 16]; 16];
/*

    .E------F
  .' |    .'|
 A---+--B'  |
 |   |  |   |
 |  .G--+---H
 |.'    | .'
 C------D'

*/
const TEXTURE_X: f32 = 0.0;
const TEXTURE_Y: f32 = 0.0625;
const TEXTURE_SIZE: f32 = 0.0625;
const TL: [f32; 2] = [TEXTURE_X, TEXTURE_Y];
const TR: [f32; 2] = [TEXTURE_X + TEXTURE_SIZE, TEXTURE_Y];
const BL: [f32; 2] = [TEXTURE_X, TEXTURE_Y + TEXTURE_SIZE];
const BR: [f32; 2] = [TEXTURE_X + TEXTURE_SIZE, TEXTURE_Y + TEXTURE_SIZE];
pub struct Vertexer {
    pub voxeldata: Chunk,
}

impl Vertexer {
    pub fn new(chunk: Chunk) -> Self {
        Self { voxeldata: chunk }
    }
    pub fn change_block(&mut self, pos: [usize; 3], new_block: Block) {
        self.voxeldata[pos[0]][pos[1]][pos[2]] = new_block;
    }
    pub const fn get_vertices() -> &'static [Vertex] {
        &[
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: TL,
            }, // A
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: TR,
            }, // B
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: BL,
            }, // C
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: BR,
            }, // D
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: TR,
            }, // E
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: TL,
            }, // F
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: BR,
            }, // G
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: BL,
            }, // H
        ]
    }

    pub fn get_indices() -> &'static [u16] {
        const generic_face: [u16; 6] = [0, 2, 3, 0, 3, 1];
        const indices: [u16; 36] = unsafe {
            std::mem::transmute([
                generic_face,
                offset_face(generic_face, 4),
                offset_face(generic_face, 8),
                offset_face(generic_face, 12),
                offset_face(generic_face, 16),
                offset_face(generic_face, 20),
            ])
        };
        &indices
    }
}
