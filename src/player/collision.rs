use std::cmp::max;

use cgmath::{
    num_traits::{abs, Float, Signed},
    InnerSpace, Point3, Vector3,
};

use crate::terrain::World;

use super::Player;

pub fn handle_collision(player: &mut Player, world: &World, dt: f32) {
    let mut hit_ground = world.block_exists(player.position);

    for voxel in three_by_three() {
        if hit_ground {
            break;
        }
        if world.block_exists(player.position + voxel) {
            let closest_point = closest_point_square(player.position, player.position + voxel);
            hit_ground = pyth(
                closest_point.x - player.position.x,
                closest_point.z - player.position.z,
            ) < (player.width) / 2.0 - player.velocity.magnitude() - dt;
        }
    }

    let head_position = player.position
        + Vector3 {
            x: 0.0,
            y: player.height,
            z: 0.0,
        };
    if hit_ground {
        player.position.y = player.position.y.ceil();
        player.velocity.y = 0.0;
    }
    let mut hit_head = world.block_exists(head_position);

    for voxel in three_by_three() {
        if hit_head {
            break;
        }
        if world.block_exists(head_position + voxel) {
            let closest_point = closest_point_square(head_position, head_position + voxel);
            hit_head = pyth(
                closest_point.x - head_position.x,
                closest_point.z - head_position.z,
            ) < (player.width) / 2.0 - player.velocity.magnitude() - dt;
        }
    }

    if hit_head {
        player.position.y = head_position.y.floor() - player.height;
        player.velocity.y = -0.00000000000000000001;
    }

    for height in 0..=player.height as usize {
        let above = player.position
            + Vector3 {
                x: 0.0,
                y: 0.5 + height as f32,
                z: 0.0,
            };

        let mut correction = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        for voxel in orthagonal() {
            if world.block_exists(above + voxel) {
                let closest_point = closest_point_square(player.position, above + voxel);
                let diff: Vector3<f32> = Vector3 {
                    x: closest_point.x - player.position.x,
                    y: 0.0,
                    z: closest_point.z - player.position.z,
                };
                let new_correction = diff - diff.normalize() * (player.width / 2.0);
                if diff.magnitude() < player.width / 2.0 {
                    correction += new_correction;
                }
            }
        }
        player.position += correction;
        player.velocity += correction;

        correction.x = 0.0;
        correction.z = 0.0;
        for voxel in three_by_three() {
            if world.block_exists(above + voxel) {
                let closest_point = closest_point_square(player.position, above + voxel);
                let diff: Vector3<f32> = Vector3 {
                    x: closest_point.x - player.position.x,
                    y: 0.0,
                    z: closest_point.z - player.position.z,
                };
                let new_correction = diff - diff.normalize() * (player.width / 2.0);
                if diff.magnitude() < player.width / 2.0 {
                    correction += new_correction;
                }
            }
        }

        player.position += correction;
        player.velocity += correction;
    }
}
fn closest_point_square(circle_pos: Point3<f32>, square_pos_fix: Point3<f32>) -> Vector3<f32> {
    let mut test_x = circle_pos.x;
    let mut test_z = circle_pos.z;

    let square_pos: Point3<f32> = [square_pos_fix.x.floor(), 0.0, square_pos_fix.z.floor()].into();
    if circle_pos.x < square_pos.x {
        test_x = square_pos.x;
    } else if circle_pos.x > square_pos.x + 1.0 {
        test_x = square_pos.x + 1.0;
    }

    if circle_pos.z < square_pos.z {
        test_z = square_pos.z;
    } else if circle_pos.z > square_pos.z + 1.0 {
        test_z = square_pos.z + 1.0;
    }
    Vector3 {
        x: test_x,
        y: 0.0,
        z: test_z,
    }
}

fn pyth(a: f32, b: f32) -> f32 {
    (a * a + b * b).sqrt()
}

fn three_by_three() -> Vec<Vector3<f32>> {
    let mut positions = Vec::new();
    for x in -1..=1 {
        for z in -1..=1 {
            if z == 0 && x == 0 {
                continue;
            }
            positions.push(Vector3 {
                x: x as f32,
                y: 0.0,
                z: z as f32,
            })
        }
    }
    positions
}

fn orthagonal() -> Vec<Vector3<f32>> {
    let mut ret = Vec::new();

    for xz in -1..1 {
        ret.push(Vector3 {
            x: xz as f32 * 2.0 + 1.0,
            y: 0.0,
            z: 0.0,
        });
        ret.push(Vector3 {
            x: 0.0,
            y: 0.0,
            z: xz as f32 * 2.0 + 1.0,
        });
    }
    ret
}

fn diagonal() -> Vec<Vector3<f32>> {
    let mut ret = Vec::new();

    for xz in -1..1 {
        ret.push(Vector3 {
            x: xz as f32 * 2.0 + 1.0,
            y: 0.0,
            z: xz as f32 * 2.0 + 1.0,
        });
        ret.push(Vector3 {
            x: -xz as f32 * 2.0 + 1.0,
            y: 0.0,
            z: xz as f32 * 2.0 + 1.0,
        });
    }
    ret
}
