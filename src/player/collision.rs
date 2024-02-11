use std::cmp::max;

use cgmath::{
    num_traits::{abs, Float, Signed},
    InnerSpace, Point3, Vector3,
};

use crate::terrain::World;

use super::Player;

pub fn handle_collision(player: &mut Player, world: &World) {
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
            ) < player.width / 2.0;
        }
    }

    if hit_ground {
        player.position.y = player.position.y.ceil();
        player.velocity.y = 0.0;
    }
    let above = player.position
        + Vector3 {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        };
    let mut correction = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
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
                if abs(correction.x) < abs(new_correction.x) {
                    correction.x = new_correction.x;
                }
                //println!("{:.2} | {:.2}", new_correction.z, abs(new_correction.z));
                if abs(correction.z) < abs(new_correction.z) {
                    correction.z = new_correction.z;
                }
            }
        }
    }
    player.position += correction;
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
