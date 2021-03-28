mod area;
mod math;
mod traffic;

use traffic::{ TrafficLight, Time};
use math::{ sum };
use area::{ Circle, Triangle, Square, cal_area};

fn main() {
  // 1. 交通灯作业
  println!("// 作业一、打印交通灯时间");
  println!("\tRed-Time: {}", TrafficLight::Red.time());
  println!("\tGreen-Time: {}", TrafficLight::Green.time());
  println!("\tYellow-Time: {}\n", TrafficLight::Yellow.time());

  //2. 计算数组和
  println!("// 作业二、计算数组和");
  let mut nums = [1, 2, 3, 4, 5, 6, 7, 2];
  let mut total_val = sum(&nums);
  println!("\tsum({:?})={:?}\n", nums, total_val);

  nums = [1, 2, 3, 4, 5, 6, 7, 4294967294];
  total_val = sum(&nums);
  println!("\tsum({:?})={:?} (error: overflow)\n", nums, total_val);

  //3. 计算图形面积
  println!("// 作业三、计算不同图形面积");
  let a: Circle = Circle{x: 2.0};
  println!("\tcircle area = {}", cal_area(a));

  let b: Triangle = Triangle{x: 2.0, y: 2.0, z: 2.0};
  println!("\ttriangle area = {}", cal_area(b));

  let c:Square = Square{x: 2.0};
  println!("\tsquare area = {}", cal_area(c));
}
