mod traffic;
mod sumarry;
mod mianji;
//use std::result;
use traffic::TrafficLights;
use mianji::Shape;

use crate::mianji::notify;


// 为交通信号灯定义一个trait,trait中包含一个返回时间的方法，不同的灯的持续时间不同
fn main() {
//交通信号灯持续时间
let current1 =TrafficLights::Red;
let current2 =TrafficLights::Green;
let current3 =TrafficLights::Yellow;
traffic::show(&current1);
traffic::show(&current2);
traffic::show(&current3);
//集合求和
let a=[1,2,3,4,5];
let result= sumarry::sum(&a);
println!("The result of summary is {:?}",result);
//计算面积
let shape1=Shape::Rectangle { width: 30, height: 50 };
notify(&shape1);
let shape2=Shape::Triangle  { width: 30, height: 50 };
notify(&shape2);
let shape3=Shape::Square { width: 30};
notify(&shape3);
 
}



