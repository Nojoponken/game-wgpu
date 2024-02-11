use cgmath::{num_traits::Float, Vector3};

use crate::terrain::World;

use super::Player;

pub fn three_by_three() -> Vec<Vector3<f32>> {
    vec![
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        },
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Vector3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: -1.0,
            y: 0.0,
            z: 1.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    ]
}

pub fn handle_collision(player: &mut Player, dt: f32, world: &World) {
    let hit_ground = world.block_exists(player.position);

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
    let hit_body = world.block_exists(above);
    if hit_body {}
}
