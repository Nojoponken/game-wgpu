use super::vertex::*;
use crate::atlas::*;
use crate::block::Block;
use crate::terrain::*;

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

const fn get_corner(corner: u8) -> [f32; 3] {
    match corner {
        0 => [-0.5, -0.5, -0.5],
        1 => [-0.5, -0.5, 0.5],
        2 => [-0.5, 0.5, -0.5],
        3 => [-0.5, 0.5, 0.5],
        4 => [0.5, -0.5, -0.5],
        5 => [0.5, -0.5, 0.5],
        6 => [0.5, 0.5, -0.5],
        7 => [0.5, 0.5, 0.5],
        _ => [0.0, 0.0, 0.0],
    }
}

fn get_face(face: u8, texture: Atlas) -> [Vertex; 4] {
    let [tl, tr, bl, br] = get_texture(texture);

    let corners = match face {
        0 => [2, 3, 0, 1],
        1 => [5, 4, 1, 0],
        2 => [6, 2, 4, 0],
        3 => [3, 7, 1, 5],
        4 => [3, 2, 7, 6],
        5 => [7, 6, 5, 4],
        _ => [8, 8, 8, 8],
    };

    [
        Vertex {
            position: get_corner(corners[0]),
            tex_coords: tl,
        },
        Vertex {
            position: get_corner(corners[1]),
            tex_coords: tr,
        },
        Vertex {
            position: get_corner(corners[2]),
            tex_coords: bl,
        },
        Vertex {
            position: get_corner(corners[3]),
            tex_coords: br,
        },
    ]
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
pub fn get_mesh(voxeldata: Chunk) -> Mesh {
    let mut verts: Vec<Vertex> = get_face(0, Atlas::MossyCobble).into();
    verts.extend(get_face(1, Atlas::MossyCobble));
    verts.extend(get_face(2, Atlas::MossyCobble));
    verts.extend(get_face(3, Atlas::MossyCobble));
    verts.extend(get_face(4, Atlas::MossyCobble));
    verts.extend(get_face(5, Atlas::MossyCobble));
    let mut inds: Vec<u16> = offset_indices(0).into();
    inds.extend(offset_indices(4));
    inds.extend(offset_indices(8));
    inds.extend(offset_indices(12));
    inds.extend(offset_indices(16));
    inds.extend(offset_indices(20));
    Mesh {
        vertices: verts,
        indices: inds,
    }
}
