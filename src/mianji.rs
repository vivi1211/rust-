//实现一个打印面积的函数，接收一个可以计算面积的类型为参数
//圆形、三角形、正方形  泛型、泛型约束
pub trait Count{
    fn area(&self)->u32;
}

// pub struct Rectangle{
//     width:u32,
//     height:u32,
// }
// struct Triangle{
//     width:u32,
//     height:u32,
// }
// struct Square{
//     width:u32,
// }
pub enum Shape{
    Rectangle{width:u32,height:u32},
    Triangle{width:u32,height:u32},
    Square{width:u32},
}
impl Count for Shape{
    fn area(&self)->u32{
        match self{
            Shape::Rectangle{width,height}=>width*height,
            Shape::Triangle{width,height}=>(width*height)/2,
            Shape::Square{width}=>width*width,
        }
    }

}

//定义一个函数notify用来打印结果,使用了泛型，trait bound 是实现了summary的trait
pub fn notify<T:Count>(para:&T){
    println!("The area of the shape is {}",para.area());
}