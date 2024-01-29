use super::vertex::*;
use crate::block::Block;
use crate::terrain::*;
use crate::{atlas::*, block};
use log::info;

const fn offset_indices(offset: u16) -> [u16; 6] {
    //let [a, b, c, d, e, f] = face;
    [
        0 + offset,
        2 + offset,
        3 + offset,
        0 + offset,
        3 + offset,
        1 + offset,
    ]
}

fn get_corner(corner: u8, coordinates: [f32; 3]) -> [f32; 3] {
    match corner {
        0 => [
            -0.5 + coordinates[0],
            -0.5 + coordinates[1],
            -0.5 + coordinates[2],
        ],
        1 => [
            -0.5 + coordinates[0],
            -0.5 + coordinates[1],
            0.5 + coordinates[2],
        ],
        2 => [
            -0.5 + coordinates[0],
            0.5 + coordinates[1],
            -0.5 + coordinates[2],
        ],
        3 => [
            -0.5 + coordinates[0],
            0.5 + coordinates[1],
            0.5 + coordinates[2],
        ],
        4 => [
            0.5 + coordinates[0],
            -0.5 + coordinates[1],
            -0.5 + coordinates[2],
        ],
        5 => [
            0.5 + coordinates[0],
            -0.5 + coordinates[1],
            0.5 + coordinates[2],
        ],
        6 => [
            0.5 + coordinates[0],
            0.5 + coordinates[1],
            -0.5 + coordinates[2],
        ],
        7 => [
            0.5 + coordinates[0],
            0.5 + coordinates[1],
            0.5 + coordinates[2],
        ],
        _ => [0.0, 0.0, 0.0],
    }
}

fn get_face(face: u8, texture: Atlas, coordinates: [f32; 3]) -> [Vertex; 4] {
    let [tl, tr, bl, br] = get_texture(texture);

    let corners = match face {
        0 => [2, 3, 0, 1],
        1 => [7, 6, 5, 4],
        2 => [5, 4, 1, 0],
        3 => [3, 2, 7, 6],
        4 => [6, 2, 4, 0],
        5 => [3, 7, 1, 5],
        _ => [8, 8, 8, 8],
    };

    [
        Vertex {
            position: get_corner(corners[0], coordinates),
            tex_coords: tl,
        },
        Vertex {
            position: get_corner(corners[1], coordinates),
            tex_coords: tr,
        },
        Vertex {
            position: get_corner(corners[2], coordinates),
            tex_coords: bl,
        },
        Vertex {
            position: get_corner(corners[3], coordinates),
            tex_coords: br,
        },
    ]
}

fn get_neighbor(current: [usize; 3], face: u8) -> [usize; 3] {
    let [x, y, z] = current;

    match face {
        0 => [x - 1, y, z],
        1 => [x + 1, y, z],
        2 => [x, y - 1, z],
        3 => [x, y + 1, z],
        4 => [x, y, z - 1],
        5 => [x, y, z + 1],
        _ => [x, y, z],
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
pub fn get_mesh(voxeldata: Chunk) -> Mesh {
    let mut verts: Vec<Vertex> = Vec::new();
    let mut inds: Vec<u16> = Vec::new();
    let mut off: u16 = 0;
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                if voxeldata[x][y][z].block_id != 0 {
                    for face in 0..6 {
                        let neighbor;

                        let x_border = (x == 0 && face == 0) || (x == 15 && face == 1);
                        let y_border = (y == 0 && face == 2) || (y == 15 && face == 3);
                        let z_border = (z == 0 && face == 4) || (z == 15 && face == 5);
                        if x_border || y_border || z_border {
                            neighbor = false;
                        } else {
                            let [nx, ny, nz] = get_neighbor([x, y, z], face);
                            neighbor = voxeldata[nx][ny][nz].block_id != 0;
                        }

                        if !neighbor {
                            verts.extend(get_face(
                                face,
                                Atlas::StoneSlabTop,
                                [x as f32, y as f32, z as f32],
                            ));
                            inds.extend(offset_indices(off));
                            off += 4;
                        }
                    }
                }
            }
        }
    }
    Mesh {
        vertices: verts,
        indices: inds,
    }
}
