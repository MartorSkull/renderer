use std::{ops::{Mul, Add}};

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
  w: f64,
  x: f64,
  y: f64,
  z: f64
}

impl Quaternion {
  pub fn identity() -> Self {
    Quaternion {
      w: 1.0,
      x: 0.0,
      y: 0.0,
      z: 0.0
    }
  }

  pub fn x_rot(r: f64) -> Self {
    Quaternion {
      w: (r/2.0).cos(),
      x: (r/2.0).sin(),
      y: 0.0,
      z: 0.0,
    }
  }

  pub fn y_rot(r: f64) -> Self {
    Quaternion {
      w: (r/2.0).cos(),
      x: 0.0,
      y: (r/2.0).sin(),
      z: 0.0
    }
  }

  pub fn z_rot(r: f64) -> Self {
    Quaternion {
      w: (r/2.0).cos(),
      x: 0.0,
      y: 0.0,
      z: (r/2.0).sin()
    }
  }

  pub fn invert(&self) -> Self {
    Quaternion {
      w: self.w,
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }

  pub fn apply_quat<T: Into<Quaternion>>(self, vector: T) -> (f64, f64, f64) {
    apply_quat(self, vector)
  }
}

impl Mul for Quaternion {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    let a = self;
    let b = rhs;
    Quaternion {
      w: a.w*b.w - a.x*b.x - a.y*b.y - a.z*b.z,
      x: a.x*b.w + a.w*b.x - a.z*b.y + a.y*b.z,
      y: a.y*b.w + a.w*b.y - a.x*b.z + a.z*b.x,
      z: a.z*b.w + a.w*b.z - a.y*b.x + a.x*b.y
    }
  }
}

impl<T: Into<f64> + Copy> Mul<T> for Quaternion {
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Quaternion {
      w: rhs.into()*self.w,
      x: rhs.into()*self.x,
      y: rhs.into()*self.y,
      z: rhs.into()*self.z
    }
  }
}

impl<T: Into<f64>> Into<Quaternion> for (T, T, T) {
  fn into(self) -> Quaternion {
    Quaternion {
      w: 1.0,
      x: self.0.into(),
      y: self.1.into(),
      z: self.2.into()
    }
  }
}

impl Add for Quaternion {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Quaternion {
      w: self.w+rhs.w,
      x: self.x+rhs.x,
      y: self.y+rhs.y,
      z: self.z+rhs.z
    }
  }
}

impl Default for Quaternion {
  fn default() -> Self {
    Quaternion::identity()
  }
}

pub trait InnerRotatable {
  fn rotate(self, q: Quaternion) -> Self;
}

impl InnerRotatable for (f64, f64, f64) {
  fn rotate(self, q: Quaternion) -> Self {
    apply_quat(q, self)
  }
}

fn apply_quat<T: Into<Quaternion>>(q: Quaternion, vector: T) -> (f64, f64, f64) {
  let a = q*vector.into()*q.invert();
  (a.x, a.y, a.z)
}

pub trait Rotatable {
  fn rotate(&mut self, q: Quaternion);
}
