use std::fmt::Debug;
use std::ops::Add;

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
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

#[test]
fn test_point() {
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
    println!("{}", Point { x: "h", y: "h" }.x())
}

#[test]
fn test_point_with_two_types() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

#[test]
fn test_array_with_wrong_lens() {
    fn display_array(arr: &[i32; 3]) {
        println!("{:?}", arr);
    }

    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);

    // let arr: [i32; 2] = [1, 2];
    // display_array(&arr);
}

#[test]
fn test_array_with_any_lens() {
    fn display_array<T: Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }

    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

#[test]
fn test_const_fn() {
    struct Buffer<const N: usize> {
        data: [u8; N],
    }

    const fn compute_buffer_size(factor: usize) -> usize {
        factor * 1024
    }

    const SIZE: usize = compute_buffer_size(4);
    let buffer = Buffer::<SIZE> { data: [255; SIZE] };
    println!("Buffer size: {} bytes", buffer.data.len());
    println!("first element: {}", buffer.data[0])
}

#[test]
fn test_case_1() {
    // 填空
    struct A; // 具体的类型 `A`.
    struct S(A); // 具体的类型 `S`.
    struct SGen<T>(T); // 泛型 `SGen`.

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}

    // 使用非泛型函数
    reg_fn(S(A {})); // 具体的类型
    gen_spec_t(SGen(A {})); // 隐式地指定类型参数  `A`.
    gen_spec_i32(SGen(6)); // 隐式地指定类型参数`i32`.

    // 显式地指定类型参数 `char`
    generic::<char>(SGen('a'));

    // 隐式地指定类型参数 `char`.
    generic(SGen('a'));
}

#[test]
fn test_case_2() {
    fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
}

#[test]
fn test_case_3() {
    struct Point<T> {
        x: T,
        y: T,
    }
    // 实现一个结构体 Point 让代码工作
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
}

#[test]
fn test_case_4() {
    // 修改以下结构体让代码工作
    struct Point<T, U> {
        x: T,
        y: U,
    }

    // 不要修改这行代码！
    let _p = Point {
        x: 5,
        y: "hello".to_string(),
    };
}

#[test]
fn test_case_5() {
    // 为 Val 增加泛型参数，不要修改 `main` 中的代码
    struct Val<T> {
        val: T,
    }

    impl<T> Val<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }

    let x = Val { val: 3.0 };
    let y = Val {
        val: "hello".to_string(),
    };
    println!("{}, {}", x.value(), y.value());
}

#[test]
fn test_case_6() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        // 实现 mixup，不要修改其它代码！
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point {
        x: "Hello",
        y: '中',
    };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
}

#[test]
fn test_case_7() {
    // 修复错误，让代码工作
    struct Point<T> {
        x: T,
        y: T,
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point {
        x: 5_f32,
        y: 10_f32,
    };
    println!("{}", p.distance_from_origin())
}

#[test]
fn test_case_8() {
    fn using_generic() {
        trait Draw {
            fn draw(&self) -> String;
        }

        impl Draw for u8 {
            fn draw(&self) -> String {
                format!("u8: {}", *self)
            }
        }

        impl Draw for f64 {
            fn draw(&self) -> String {
                format!("f64: {}", *self)
            }
        }
        pub struct Screen<T: Draw> {
            pub components: Vec<T>,
        }

        impl<T> Screen<T>
        where
            T: Draw,
        {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }
    }
}
