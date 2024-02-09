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

fn get_corners(normal: Vector3<f32>, position: Vector3<f32>) -> [[f32; 3]; 4] {
    let half = normal * 0.5;
    let middle = position + half;
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
    normal: Vector3<f32>,
    texture: Atlas,
    coordinates: [f32; 3],
    occluders: [f32; 8],
) -> [Vertex; 4] {
    let mut rotation: u8 = 0;

    if normal.x == -1.0 {
        rotation = 2;
    }
    if normal.z == -1.0 {
        rotation = 3;
    }
    if normal.z == 1.0 {
        rotation = 3;
    }

    let textures = get_texture_coordinates(texture, rotation);
    let corners = get_corners(
        normal,
        Vector3 {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        },
    );
    let flip = (normal.x + normal.y + normal.z < 0.0) as usize;

    let normal_dir = normal.into();

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

fn get_occluders(position: Vector3<f32>, normal: Vector3<f32>) -> [Vector3<f32>; 8] {
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

fn get_normal(face: u8) -> Vector3<f32> {
    Vector3 {
        x: (face == 0) as u8 as f32 - (face == 1) as u8 as f32,
        y: (face == 2) as u8 as f32 - (face == 3) as u8 as f32,
        z: (face == 4) as u8 as f32 - (face == 5) as u8 as f32,
    }
}

pub struct Mesh {
    //pub vertices: Vec<Vertex>,
    //pub indices: Vec<u32>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub instance_buffer: Buffer,
    pub num_indices: usize,
}
pub fn get_mesh(voxeldata: &Chunk, chunk_pos: Vector3<isize>, device: &Device) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut off: u32 = 0;
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if voxeldata[x][y][z].block_id != 0 {
                    for face in 0..6 {
                        let normal = get_normal(face);

                        let mut occluders: [f32; 8] = [0.33; 8];

                        let x_border = x == (CHUNK_SIZE - 1) * ((normal.x + 1.0) / 2.0) as usize;
                        let y_border = y == (CHUNK_SIZE - 1) * ((normal.y + 1.0) / 2.0) as usize;
                        let z_border = z == (CHUNK_SIZE - 1) * ((normal.z + 1.0) / 2.0) as usize;

                        if x_border || y_border || z_border {
                            continue;
                        }
                        let neighbor_position = Vector3 {
                            x: x as f32,
                            y: y as f32,
                            z: z as f32,
                        } + normal; //hhget_neighbor([x, y, z], face);

                        if voxeldata[neighbor_position.x as usize][neighbor_position.y as usize]
                            [neighbor_position.z as usize]
                            .block_id
                            != 0
                        {
                            continue;
                        }

                        let occluders_pos = get_occluders(neighbor_position, normal);

                        let mut i = 0;
                        for pos in occluders_pos {
                            if pos.x > CHUNK_SIZE as f32 - 1.0
                                || pos.x < 0.0
                                || pos.y > CHUNK_SIZE as f32 - 1.0
                                || pos.y < 0.0
                                || pos.z > CHUNK_SIZE as f32 - 1.0
                                || pos.z < 0.0
                            {
                                continue;
                            };
                            occluders[i] =
                                (voxeldata[pos.x as usize][pos.y as usize][pos.z as usize].block_id
                                    == 0) as u8 as f32
                                    * 0.33;
                            i += 1;
                        }

                        let flip = normal.sum() < 0.0;
                        let texture =
                            block::get_texture(voxeldata[x][y][z].block_id, normal.into());
                        vertices.extend(get_face(
                            normal,
                            texture,
                            [x as f32, y as f32, z as f32],
                            occluders,
                        ));
                        indices.extend(offset_indices(off, flip));
                        off += 4;
                    }
                }
            }
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
            x: chunk_pos.x as f32 * CHUNK_SIZE as f32,
            y: chunk_pos.y as f32 * CHUNK_SIZE as f32,
            z: chunk_pos.z as f32 * CHUNK_SIZE as f32,
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
        // vertices,
        //  indices,
        vertex_buffer,
        index_buffer,
        instance_buffer,
        num_indices: indices.len(),
    }
}
