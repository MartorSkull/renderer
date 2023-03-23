use std::{f64::consts::PI, io::{Error, ErrorKind, Result}};

use camera::{CameraBuilder};
use minifb::Key;
use hsl::HSL;

mod canvas;
mod camera;
mod quaternion;

use crate::{canvas::Canvas, quaternion::{Quaternion, Rotatable}};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() -> Result<()> {
  let canvas = Canvas::new(
    "Renderer from Scratch",
    WIDTH,
    HEIGHT
  );
  let mut camera = CameraBuilder::default()
    .canvas(&canvas)
    .build()
    .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
  camera.rotate(Quaternion::x_rot(PI/2.0));

  test_canvas(canvas);

  Ok(())
}

fn test_canvas(mut canvas: Canvas) {
  let mut r = 50.0;
  let mut t = 0.0;

  while canvas.window.is_open() && !canvas.window.is_key_down(Key::Escape) {
    for (x,y) in canvas.coord_iter() {
      if ((x.pow(2)+y.pow(2)) as f64).sqrt() <= r {
        let h = (((x*360) as f64)/(2.0*r)+t) % 360.0;
        canvas.put_pixel(x, y, HSL{h, s: 1.0, l: 0.5});
      } else {
        canvas.put_pixel(x, y, 0);
      }
    }

    if canvas.window.is_key_down(Key::Equal) {
      r += 0.2;
    }
    if canvas.window.is_key_down(Key::Minus) {
      r -= 0.2;
    }
    t = (t + 1.0) % 360.0;

    canvas.show();
  }
}