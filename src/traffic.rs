use std::fmt;
pub trait Time{
    fn duration(&self)->u32;
    
}

// pub trait std::fmt::Display {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
// }    
 
pub enum TrafficLights{
    Green,
    Red,
    Yellow,
}
impl Time for TrafficLights{
    fn duration(&self)->u32{
        match self {
            TrafficLights::Red=>30,
            TrafficLights::Green=>60,
            TrafficLights::Yellow=>5,
        }
    }  
}
impl std::fmt::Display for TrafficLights{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrafficLights::Red=>f.write_str("Red light"),
            TrafficLights::Green=>f.write_str("Green light"),
            TrafficLights::Yellow=>f.write_str("Yellow light"),
            
        }
    }
}
pub fn show<T:Time+std::fmt::Display>(item:&T){
    println!("The Duration time of {}  is {}s",item,item.duration());
}
