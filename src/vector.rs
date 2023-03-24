use std::ops::{Mul, Add, Sub, Neg, Div};

use num_traits::Float;

#[derive(Clone, Copy)]
pub struct Vector3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

#[allow(dead_code)]
impl<T: Float> Vector3<T> {
  pub fn new(x: T, y: T, z: T) -> Self {
    Vector3 { x, y, z }
  }

  pub fn zero() -> Self {
    Vector3::new(T::zero(), T::zero(), T::zero())
  }

  pub fn x_unit() -> Self {
    Vector3::new(T::one(), T::zero(), T::zero())
  }

  pub fn y_unit() -> Self {
    Vector3::new(T::zero(), T::one(), T::zero())
  }

  pub fn z_unit() -> Self {
    Vector3::new(T::zero(), T::zero(), T::one())
  }

  pub fn dot(self, rhs: Self) -> T {
    self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
  }

  pub fn cross(self, rhs: Self) -> Self {
    Vector3::new(
      self.y*rhs.z - self.z*rhs.y,
      self.z*rhs.x - self.x*rhs.z,
      self.x*rhs.y - self.y*rhs.x,
    )
  }

  pub fn magnitude(&self) -> T {
      (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
  }

  fn test(self) -> Self {
    Vector3::x_unit()-self
  }
}

impl<T: Float> Add for Vector3<T> {
  type Output = Self;
  fn add(self, rhs: Vector3<T>) -> Self::Output {
    Self::Output {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl<T: Float> Neg for Vector3<T> {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Self::Output::new(
      -self.x,
      -self.y,
      -self.z
    )
  }
}

impl<T: Float> Sub for Vector3<T> {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    self+(-rhs)
  }
}

impl<T: Float> Mul<T> for Vector3<T>{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::Output {
      x: rhs*self.x,
      y: rhs*self.y,
      z: rhs*self.z,
    }
  }
}

impl<T: Float> Div<T> for Vector3<T> {
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output{
    self*(rhs.recip())
  }
}

impl<T: Float> Into<Vector3<T>> for (T, T, T) {
  fn into(self) -> Vector3<T> {
    Vector3::new(self.0, self.1, self.2)
  }
}