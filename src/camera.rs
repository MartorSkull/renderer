use derive_builder::Builder;

use crate::{quaternion::{Quaternion, Rotatable, InnerRotatable}, canvas::{Canvas}};

#[derive(Builder)]
pub struct Camera<'a> {
  #[builder(default = "(1.0, 1.0)")]
  viewport_size: (f64, f64),
  #[builder(default = "1.0")]
  v_distance: f64,
  #[builder(default = "(1.0, 1.0, 1.0)")]
  position: (f64, f64, f64),
  #[builder(default = "Quaternion::identity()")]
  rotation: Quaternion,
  canvas: &'a Canvas,
}

#[allow(dead_code)]
impl<'a> Camera<'a> {
  pub fn new(
    viewport_size: (f64, f64),
    v_distance: f64,
    position: (f64, f64, f64),
    rotation: Quaternion,
    canvas: &'a Canvas,
) -> Self {
    Camera {
      viewport_size,
      v_distance,
      position,
      rotation,
      canvas,
    }
  }

  pub fn get_canvas_ray(&self, x: isize, y: isize) -> (f64, f64, f64) {
    let (cw, ch) = self.canvas.get_size();
    let (vw, vh) = self.viewport_size;
    assert!(y.abs() <= (ch as isize)/2 && x.abs() <= (cw as isize)/2);
    let vx = (x as f64)*(vw/(cw as f64));
    let vy = (y as f64)*(vh/(ch as f64));
    (vx, vy, self.v_distance).rotate(self.rotation)
  }

  pub fn get_ray_iter(&self) -> impl Iterator<Item = (f64, f64, f64)> + '_ {
    self.canvas.coord_iter().map(|(x,y)| self.get_canvas_ray(x, y))
  }

  pub fn get_position(&self) -> (f64, f64, f64) {
    self.position
  }
}

impl Rotatable for Camera<'_> {
  fn rotate(&mut self, q: Quaternion) {
    self.rotation = self.rotation * q;
  }
}

