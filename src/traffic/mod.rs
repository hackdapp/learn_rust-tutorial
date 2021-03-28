pub enum TrafficLight {
  Red,
  Green,
  Yellow
}
pub trait Time {
  fn time(&self) -> u8;
}
impl Time for TrafficLight {
  fn time(&self) -> u8 {
    match &self {
      TrafficLight::Red => { 1 },
      TrafficLight::Green => { 2 },
      TrafficLight::Yellow => { 3 }
    }
  }
}
