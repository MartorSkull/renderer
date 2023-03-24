use derive_builder::Builder;

use crate::{quaternion::{Quaternion, Rotatable, InnerRotatable}, vector::Vector3};

#[derive(Builder)]
pub struct Camera {
  #[builder(default = "(1.6, 0.9)")]
  viewport_size: (f64, f64),
  #[builder(default = "1.0")]
  v_distance: f64,
  #[builder(default = "Vector3::new(0.0, 0.0, 0.0)")]
  position: Vector3<f64>,
  #[builder(default = "Quaternion::identity()")]
  rotation: Quaternion,
  canvas_size: (usize, usize),
}

#[allow(dead_code)]
impl Camera {
  pub fn new(
    viewport_size: (f64, f64),
    v_distance: f64,
    position: Vector3<f64>,
    rotation: Quaternion,
    canvas_size: (usize, usize),
) -> Self {
    Camera {
      viewport_size,
      v_distance,
      position,
      rotation,
      canvas_size,
    }
  }

  pub fn get_canvas_ray(&self, x: isize, y: isize) -> Vector3<f64> {
    let (cw, ch) = self.canvas_size;
    let (vw, vh) = self.viewport_size;
    assert!(y.abs() <= (ch as isize)/2 && x.abs() <= (cw as isize)/2);
    let vx = (x as f64)*(vw/(cw as f64));
    let vy = (y as f64)*(vh/(ch as f64));
    Vector3::new(vx, vy, self.v_distance).rotate(self.rotation)
  }

  pub fn get_position(&self) -> Vector3<f64> {
    self.position
  }
}

impl Rotatable for Camera {
  fn rotate(&mut self, q: Quaternion) {
    self.rotation = self.rotation * q;
  }
}

