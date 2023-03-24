use crate::{canvas::color::Color, vector::Vector3};

pub trait Renderable: Color {
  fn intersect_ray(&self, origin: Vector3<f64>, direction: Vector3<f64>) -> Vec<f64>;
}

pub struct Scene {
  objects: Vec<Box<dyn Renderable>>
}

impl Scene {
  pub fn empty() -> Self {
    Scene { objects: vec![] }
  }

  pub fn add<T: Renderable + Sized + 'static>(&mut self, object: T) -> () {
    self.objects.push(Box::new(object))
  }

  pub fn trace_ray(&self, origin: Vector3<f64>, direction: Vector3<f64>, min: Option<f64>, max: Option<f64>) -> impl Color {
    let (min, max) = (min.unwrap_or(f64::NEG_INFINITY), max.unwrap_or(f64::INFINITY));
    let mut closest_obj = None;
    let mut closest_t = max;
    for o in self.objects.iter() {
      for distance in o.intersect_ray(origin, direction) {
        if distance <= closest_t && (min <= distance) {
          closest_t = distance;
          closest_obj = Some(o);
        }
      }
    }

    match closest_obj {
      Some(o) => o.to_color(),
      None => 0
    }
}
}

pub struct Sphere {
  radius: f64,
  position: Vector3<f64>,
  color: u32
}

impl Sphere {
  pub fn new(radius: f64, position: Vector3<f64>, color: u32) -> Self {
    Self { radius, position, color }
  }
}

impl Color for Sphere {
  fn to_color(&self) -> u32 {
      self.color
  }
}

impl Renderable for Sphere {
  fn intersect_ray(&self, o: Vector3<f64>, d: Vector3<f64>) -> Vec<f64> {
    let co = o-self.position;

    let a = d.dot(d);
    let b = 2.0*co.dot(d);
    let c = co.dot(co)-self.radius.powi(2);

    let disc = b.powi(2) - 4.0*a*c;

    if disc < 0.0 {
      vec![]
    } else if disc == 0.0 {
      vec![
        -b/(2.0*a)
      ]
    } else {
      vec![
        (-b+(disc).sqrt())/(2.0*a),
        (-b-(disc).sqrt())/(2.0*a)
      ]
    }
  }
}

pub struct Triangle {
  a: Vector3<f64>,
  b: Vector3<f64>,
  c: Vector3<f64>,
  color: u32,
}

#[allow(dead_code)]
impl Triangle {
  pub fn new(a: Vector3<f64>, b: Vector3<f64>, c: Vector3<f64>, color: u32) -> Self{
    Triangle { a, b, c, color }
  }
}

impl Color for Triangle {
  fn to_color(&self) -> u32 {
      self.color
  }
}

impl Renderable for Triangle {
  fn intersect_ray(&self, origin: Vector3<f64>, direction: Vector3<f64>) -> Vec<f64> {
    let nn = (self.b-self.a).cross(self.c-self.a);
    let n = nn/nn.magnitude();

    let dist = n.dot(self.a);

    let t = (dist-n.dot(origin))/n.dot(direction);

    let q = origin+direction*t;

    let check = vec![(self.b, self.a), (self.c, self.a), (self.a, self.c)]
      .into_iter()
      .fold(
        true,
        |prev, (a, b)| prev && (a-b).cross(q-b).dot(n) >= 0.0
      );

    if check {
      vec![t]
    } else {
      vec![]
    }
  }
}