mod collision;
pub mod controller;

use cgmath::Point3;
use cgmath::Rad;
use cgmath::Vector3;
use instant::Duration;

use crate::terrain::World;

use self::controller::PlayerController;

pub struct Camera {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
}
impl Camera {
    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }
}

pub struct Player {
    pub position: Point3<f32>,
    velocity: Vector3<f32>,
    pub camera: Camera,
    pub height: f32,
    pub width: f32,
}

impl Player {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            position,
            velocity: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            camera: Camera::new(position, Rad(0.0), Rad(0.0)),
            height: 2.8,
            width: 1.8,
        }
    }
    pub fn update(&mut self, controller: &mut PlayerController, dt: Duration, world: &World) {
        let dt = dt.as_secs_f32();
        controller.update_player(self, dt);

        self.velocity.y -= 0.5 * dt;
        self.velocity.x *= 0.9;
        self.velocity.z *= 0.9;
        self.position += self.velocity;

        collision::handle_collision(self, world);
        self.camera.position += ((self.position
            + Vector3 {
                x: 0.0,
                y: self.height - 0.3,
                z: 0.0,
            })
            - self.camera.position);
        controller.update_camera(&mut self.camera, dt);
    }
}
