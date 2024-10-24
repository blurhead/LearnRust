#[allow(unused_imports)]
use std::ops::Add;

fn add<T: Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


#[test]
fn test_generic() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);


    print!("{}", largest(&[1, 2, 3]));
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
#[test]
fn test_point() {
    println!("{}", Point { x: "h", y: "h" }.x())
}

struct Point1<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point1<T, U> {
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

#[test]
fn test_point1() {
    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


fn display_array(arr: &[i32;3]) {
    println!("{:?}", arr);
}

#[test]
fn test_array_with_wrong_lens() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);

    let arr: [i32; 2] = [1, 2];
    display_array(&arr);
}