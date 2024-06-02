use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[test]
fn main() {
    let p = Point { x: 1, y: 1 };
}
