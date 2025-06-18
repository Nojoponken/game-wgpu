use super::vertex::*;
use super::*;
use crate::atlas::*;
use crate::block;
use cgmath::Array;
use cgmath::Deg;
use cgmath::Quaternion;
use cgmath::Rotation3;
use cgmath::Vector3;
use wgpu::util::DeviceExt;
use wgpu::Buffer;
use wgpu::Device;

fn offset_indices(offset: u32, flip: bool) -> [u32; 6] {
    //let [a, b, c, d, e, f] = face;
    let mut ret = [
        0 + offset,
        2 + offset,
        3 + offset,
        0 + offset,
        3 + offset,
        1 + offset,
    ];
    if flip {
        ret.reverse();
    }
    ret
}

fn get_corners(normal: Vector3<i8>, position: Point3<i8>) -> [[f32; 3]; 4] {
    let half = Vector3 {
        x: normal.x as f32 * 0.5,
        y: normal.y as f32 * 0.5,
        z: normal.z as f32 * 0.5,
    };
    let middle = Point3 {
        x: position.x as f32,
        y: position.y as f32,
        z: position.z as f32,
    } + Vector3 {
        x: 0.5,
        y: 0.5,
        z: 0.5,
    } + half;
    [
        [
            middle.x + half.y + half.z,
            middle.y + half.z + half.x,
            middle.z + half.x + half.y,
        ],
        [
            middle.x - half.y + half.z,
            middle.y - half.z + half.x,
            middle.z - half.x + half.y,
        ],
        [
            middle.x + half.y - half.z,
            middle.y + half.z - half.x,
            middle.z + half.x - half.y,
        ],
        [
            middle.x - half.y - half.z,
            middle.y - half.z - half.x,
            middle.z - half.x - half.y,
        ],
    ]
}

fn get_face(
    normal: Vector3<i8>,
    texture: Atlas,
    coordinates: Point3<i8>,
    occluders: [f32; 8],
) -> [Vertex; 4] {
    let mut rotation: u8 = 0;

    if normal.x == -1 {
        rotation = 2;
    }
    if normal.z == -1 {
        rotation = 3;
    }
    if normal.z == 1 {
        rotation = 3;
    }

    let textures = get_texture_coordinates(texture, rotation);
    let corners = get_corners(normal, coordinates);
    let flip = normal.sum().is_negative() as usize;

    let normal_dir = [normal.x as f32, normal.y as f32, normal.z as f32];

    [
        Vertex {
            position: corners[0],
            tex_coords: textures[0 + flip],
            normal: normal_dir,
            ao: occluders[0] + occluders[1] + occluders[2],
        },
        Vertex {
            position: corners[1],
            tex_coords: textures[1 - flip],
            normal: normal_dir,
            ao: occluders[2] + occluders[3] + occluders[4],
        },
        Vertex {
            position: corners[2],
            tex_coords: textures[2 + flip],
            normal: normal_dir,
            ao: occluders[6] + occluders[7] + occluders[0],
        },
        Vertex {
            position: corners[3],
            tex_coords: textures[3 - flip],
            normal: normal_dir,
            ao: occluders[4] + occluders[5] + occluders[6],
        },
    ]
}

fn get_occluders(position: Point3<i8>, normal: Vector3<i8>) -> [Point3<i8>; 8] {
    let h = Vector3 {
        x: normal.y,
        y: normal.z,
        z: normal.x,
    };

    let v = Vector3 {
        x: normal.z,
        y: normal.x,
        z: normal.y,
    };

    [
        position + h,
        position + h + v,
        position + v,
        position + v - h,
        position - h,
        position - h - v,
        position - v,
        position - v + h,
    ]
}

fn get_normal(face: u8) -> Vector3<i8> {
    Vector3 {
        x: (face == 0) as i8 - (face == 1) as i8,
        y: (face == 2) as i8 - (face == 3) as i8,
        z: (face == 4) as i8 - (face == 5) as i8,
    }
}

fn outside_chunk(position: &Point3<i8>) -> bool {
    position.x.is_negative()
        || position.x >= CHUNK_SIZE as i8
        || position.y.is_negative()
        || position.y >= CHUNK_SIZE as i8
        || position.z.is_negative()
        || position.z >= CHUNK_SIZE as i8
}

fn check_neighbor_at_edge_of_chunk(
    chunks: &HashMap<Point3<isize>, Chunk>,
    chunk_pos: &Point3<isize>,
    normal: &Vector3<i8>,
    neighbor: &Point3<i8>,
    self_id: u8,
) -> bool {
    let neighbor_chunk = &[
        chunk_pos.x + normal.x as isize,
        chunk_pos.y + normal.y as isize,
        chunk_pos.z + normal.z as isize,
    ]
    .into();
    let relative_pos = Point3 {
        x: (neighbor.x + CHUNK_SIZE as i8 * -normal.x) as i8,
        y: (neighbor.y + CHUNK_SIZE as i8 * -normal.y) as i8,
        z: (neighbor.z + CHUNK_SIZE as i8 * -normal.z) as i8,
    };
    block_opaque(chunks, neighbor_chunk, &relative_pos, self_id)
}

fn get_relative_chunk(position: &Point3<i8>) -> Vector3<i8> {
    let less_than_zero_x = position.x.is_negative() as i8;
    let larger_than_chunk_x = (position.x >= CHUNK_SIZE as i8) as i8;
    let less_than_zero_y = position.y.is_negative() as i8;
    let larger_than_chunk_y = (position.y >= CHUNK_SIZE as i8) as i8;
    let less_than_zero_z = position.z.is_negative() as i8;
    let larger_than_chunk_z = (position.z >= CHUNK_SIZE as i8) as i8;

    Vector3 {
        x: less_than_zero_x * -1 + larger_than_chunk_x,
        y: less_than_zero_y * -1 + larger_than_chunk_y,
        z: less_than_zero_z * -1 + larger_than_chunk_z,
    }
}
pub fn block_opaque(
    chunks: &HashMap<Point3<isize>, Chunk>,
    chunk_pos: &Point3<isize>,
    relative_pos: &Point3<i8>,
    self_id: u8,
) -> bool {
    let chunk = chunks.get(chunk_pos);

    if chunk.is_none() {
        return false;
    }

    let block = chunk.unwrap().get(relative_pos);
    if block.is_some() {
        if block.unwrap().block_id == 7
            || block.unwrap().block_id == 9
            || block.unwrap().block_id == 0
        {
            return block.unwrap().block_id == self_id;
        }
    } else {
        return false;
    }

    true
}

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub instance_buffer: Buffer,
    pub num_indices: usize,
}

pub fn get_mesh(
    chunks: &HashMap<Point3<isize>, Chunk>,
    chunk_pos: Point3<isize>,
    device: &Device,
) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut off: u32 = 0;

    let voxeldata = chunks.get(&chunk_pos).unwrap();
    for block in voxeldata {
        for face in 0..6 {
            let normal = get_normal(face);
            let block_pos: &Point3<i8> =
                &[block.0.x as i8, block.0.y as i8, block.0.z as i8].into();
            let neighbor_position = block_pos + normal;
            let mut neighbor_chunk = chunk_pos;

            if outside_chunk(&neighbor_position) {
                neighbor_chunk += normal.cast().unwrap();
                if block_opaque(
                    chunks,
                    &neighbor_chunk,
                    &(neighbor_position - normal * CHUNK_SIZE as i8),
                    block.1.block_id,
                ) {
                    continue;
                }
            } else if block_opaque(chunks, &chunk_pos, &neighbor_position, block.1.block_id) {
                continue;
            }

            let mut occluders: [f32; 8] = [0.33; 8];

            let occluders_pos = get_occluders(neighbor_position, normal);

            let mut i = 0;
            for pos in occluders_pos {
                if outside_chunk(&pos) {
                    occluders[i] = !check_neighbor_at_edge_of_chunk(
                        chunks,
                        &chunk_pos,
                        &get_relative_chunk(&pos),
                        &pos,
                        block.1.block_id,
                    ) as u8 as f32
                        * 0.33;
                } else {
                    occluders[i] = !block_opaque(chunks, &chunk_pos, &pos, block.1.block_id) as u8
                        as f32
                        * 0.33;
                }
                i += 1;
            }

            let flip = normal.sum().is_negative();
            let texture = block::get_texture(block.1.block_id, normal.into());
            vertices.extend(get_face(normal, texture, *block.0, occluders));
            indices.extend(offset_indices(off, flip));
            off += 4;
        }
    }

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    let instance = instance::Instance {
        position: Vector3 {
            x: chunk_pos[0] as f32 * CHUNK_SIZE as f32,
            y: chunk_pos[1] as f32 * CHUNK_SIZE as f32,
            z: chunk_pos[2] as f32 * CHUNK_SIZE as f32,
        },
        rotation: Quaternion::from_axis_angle(Vector3::unit_z(), Deg(0.0)),
    };
    let instance_data = vec![instance.to_raw()];
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instance_data),
        usage: wgpu::BufferUsages::VERTEX,
    });
    Mesh {
        vertex_buffer,
        index_buffer,
        instance_buffer,
        num_indices: indices.len(),
    }
}
