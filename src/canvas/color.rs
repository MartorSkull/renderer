use hsl::HSL;

pub trait ConstColor<const T: u32> {
  fn get_color() -> u32{
    T
  }
}

pub trait Color {
  fn to_color(&self) -> u32;
}

impl Color for u32 {
  fn to_color(&self) -> u32 {
    *self
  }
}

impl Color for HSL {
  fn to_color(&self) -> u32 {
    self.to_rgb().to_color()
  }
}

impl<N: Into<u32> + Copy> Color for (N, N, N) {
  fn to_color(&self) -> u32 {
    self.0.into() << 16 | self.1.into() << 8 | self.2.into()
  }
}