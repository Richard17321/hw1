enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 30_u8,
            TrafficLight::Green => 50_u8,
            TrafficLight::Yellow => 3_u8,
        }
    }
}

fn main() {
    let light = TrafficLight::Green;
    println!("light is: {}", light.time());
}
