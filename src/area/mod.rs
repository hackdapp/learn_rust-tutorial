use std::f64::consts::PI;

pub struct Circle {
  pub x: f64 //半径
}

pub struct Triangle {
  pub x: f64, // 边长1
  pub y: f64, // 边长2
  pub z: f64  // 边长3
}

pub struct Square {
  pub x: f64 // 边长
}

pub trait Area {
  fn area(&self) -> f64;
}

impl Area for Circle {
  fn area(&self) -> f64 {
    PI * &self.x * self.x
  }
}

impl Area for Triangle {
  fn area(&self) -> f64 {
    let p:f64 = (self.x + self.y + self.z) / 2.0;
    let result:f64 = (p * ( p - self.x) * ( p - self.y) * ( p - self.z)).sqrt();
    result
  }
}

impl Area for Square {
  fn area(&self) -> f64 {
    &self.x * self.x
  }
}

pub fn cal_area<T: Area>(item: T) -> f64 {
  item.area()
}
