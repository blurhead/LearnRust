struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> u32 {
        self.width
    }
}

#[test]
fn test_rect() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    assert_eq!(rect1.area(), 1500);
    assert_eq!(rect1.width(), 30);
}

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // using `Self` to fill in the blank
    pub fn show_state(self: &Self) {
        println!("the current state is {}", self.color);
    }

    // fill in the blank, DON'T use any variants of `Self`
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}

impl TrafficLight {
    // 1. implement a associated function `new`,
    // 2. it will return a TrafficLight contains color "red"
    // 3. must use `Self`, DON'T use `TrafficLight`
    pub fn new() -> Self {
        Self {
            color: "red".to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

#[test]
fn test_traffic_light() {
    let mut light = TrafficLight::new();
    // Don't take the ownership of `light` here
    light.show_state();
    light.change_state();
    // otherwise, there will be an error below
    println!("{:?}", light.get_state());
}

#[test]
fn test_enum() {
    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }

    // implement TrafficLightColor with a method
    impl TrafficLightColor {
        fn color(&self) -> String {
            match self {
                TrafficLightColor::Red => "red".to_string(),
                TrafficLightColor::Yellow => "yellow".to_string(),
                TrafficLightColor::Green => "green".to_string(),
            }
        }
    }

    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}
