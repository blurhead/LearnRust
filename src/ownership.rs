#![allow(unused, unused_variables)]

mod ptr;
mod report;
mod serialize;
mod slice;
mod str;

use std::fmt::Debug;
use std::mem::{size_of, size_of_val};
use utf8_slice;

#[test]
fn test_ownership() {
    fn run() {
        let s = String::from("hello"); // s 进入作用域

        {
            println!("{}", &s);
        }
        {
            takes_ownership(&s);
        }
        // ... 所以到这里不再有效

        let x = 5; // x 进入作用域

        makes_copy(x); // x 应该移动函数里，
                       // 但 i32 是 Copy 的，所以在后面可继续使用 x
    } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
      // 所以不会有特殊操作

    fn takes_ownership(some_string: &String) {
        // some_string 进入作用域
        println!("{}", *some_string);
    } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

    fn makes_copy(some_integer: i32) {
        // some_integer 进入作用域
        println!("{}", some_integer);
    } // 这里，some_integer 移出作用域。不会有特殊操作

    run()
}

#[allow(unused, unused_variables)]
fn test_struct() {
    type File = String;

    fn open(f: &mut File) -> bool {
        true
    }

    #[allow(unused)]
    fn close(f: &mut File) -> bool {
        true
    }

    #[allow(dead_code)]
    fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
        unimplemented!()
    }

    fn main() {
        let mut f1 = File::from("f1.txt");
        let f2 = &mut f1;
        open(f2);
        report::report(&f1);
        report::report(&f1);
        //read(&mut f1, &mut vec![]);
    }

    main()
}

fn first_word(s: &str) -> &str {
    &s[..1]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    let char = 'a';
    println!("the first word is: {}", first_word(&s).to_string());

    let character = '我'; // 这里使用了一个Unicode字符作为示例
    let length = size_of_val(&character); // 获取字符的长度
}
