use std::{f64::consts::PI, io::{Error, ErrorKind, Result}, process::exit};

use camera::{CameraBuilder};
use quaternion::Rotatable;
use scene::{Scene, Sphere, Triangle};
use vector::Vector3;

mod canvas;
mod camera;
mod quaternion;
mod scene;
mod vector;

use crate::{canvas::Canvas, quaternion::Quaternion};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const VIEWPORT_WIDTH: f64 = 1.6;
const VIEWPORT_HEIGHT: f64 = VIEWPORT_WIDTH*(HEIGHT as f64)/(WIDTH as f64);

fn main() -> Result<()> {
  let canvas = Canvas::new(
    "Renderer from Scratch",
    WIDTH,
    HEIGHT
  );

  test_scene(canvas)
}

fn test_scene(mut canvas: Canvas) -> Result<()>  {
  let mut camera = CameraBuilder::default()
    .canvas_size(canvas.get_size())
    .viewport_size((VIEWPORT_WIDTH, VIEWPORT_HEIGHT))
    .position(Vector3::new(3.0, 0.0, 3.0))
    .rotation(Quaternion::y_rot(-PI/2.0))
    .build()
    .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
  let mut scene = Scene::empty();
  scene.add(Sphere::new(1.0, Vector3::new(0.0, 0.0, 4.0), 0xFF0000));
  scene.add(Sphere::new(0.5, Vector3::new(0.0, 0.0, 3.0), 0x00FF00));
  scene.add(Sphere::new(0.2, Vector3::new(0.0, 0.0, 2.0), 0x0000FF));
  scene.add(Triangle::new(
    Vector3::new(0.0, 0.1, 2.2),
    Vector3::new(0.0, 0.0, 2.5),
    Vector3::new(0.0, -0.1, 2.2),
    0xFFFF00)
  );
  while canvas.window.is_open(){
    for key in canvas.window.get_keys() {
      match key {
        // Movement
        minifb::Key::W => camera.move_to(Vector3::new(0.0, 0.0, 0.25)),
        minifb::Key::S => camera.move_to(Vector3::new(0.0, 0.0, -0.25)),
        minifb::Key::A => camera.move_to(Vector3::new(-0.25, 0.0, 0.0)),
        minifb::Key::D => camera.move_to(Vector3::new(0.25, 0.0, 0.0)),
        minifb::Key::Space => camera.move_to(Vector3::new(0.0, -0.25, 0.0)),
        minifb::Key::LeftCtrl => camera.move_to(Vector3::new(0.0, 0.25, 0.0)),

        // Rotation
        minifb::Key::Q => camera.rotate(Quaternion::z_rot(-PI/32.0)),
        minifb::Key::E => camera.rotate(Quaternion::z_rot(PI/32.0)),
        minifb::Key::Down => camera.rotate(Quaternion::x_rot(-PI/32.0)),
        minifb::Key::Up => camera.rotate(Quaternion::x_rot(PI/32.0)),
        minifb::Key::Right => camera.rotate(Quaternion::y_rot(PI/32.0)),
        minifb::Key::Left => camera.rotate(Quaternion::y_rot(-PI/32.0)),

        // Window Control
        minifb::Key::Escape => exit(0),
        minifb::Key::Home => {
          camera.move_to(Vector3::zero());
          camera.set_rotation(Quaternion::identity())
        },

        _ => {}
    }
    }

    for (x, y) in canvas.coord_iter() {
      let ray = camera.get_canvas_ray(x, y);
      let color = scene.trace_ray(camera.get_position(), ray, Some(0.0), None);
      canvas.put_pixel(x, y, color)
    }
    canvas.show();
  }

  Ok(())
}
