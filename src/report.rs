use std::fmt::Debug;

pub fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}
