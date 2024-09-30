#[derive(Debug, Copy, Clone)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[test]
fn test_match() {
    // 填空
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            // 在这里匹配 South 或 North
            println!("South or North");
        }
        _ => println!("West"),
    };
}
enum IpAddr {
    Ipv4,
    Ipv6,
}

#[test]
fn test_match_as_exp() {
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    RGB(u16, u16, u16),
}

#[test]
fn test_complicate_pattern_bind() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::RGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::RGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}

#[test]
fn test_use_variable_in_pattern_match() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };

    match dire {
        Direction::South => {
            println!("three");
        }
        _ => {}
    }

    let s = Some(3u8);
    match s {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(x) = s {
        println!("{}", x);
    }

    let v = 2u8;

    if let 2 = v {
        println!("two");
    }
}

enum MyEnum {
    Foo,
    Bar,
}

#[test]
fn test_matches() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    let _ = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    for x in 'A'..='Z' {
        println!("{}", x)
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(i) => Some(i + 1),
        None => None,
    }
}

#[test]
fn test_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}, {:?}", five, six, none);
}

#[test]
fn test_while_let() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    while let Some(x) = v.pop() {
        println!("{:?}", x);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

#[test]
fn test_option_match() {
    let a: Option<u8> = Some(1);
    let b: Option<u8> = None;
    if let None = b {
        println!("{:?}", a)
    }
}

#[test]
fn test_destruct_arr() {
    let arr: &[u16] = &[114, 514];

    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }

    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }

    let arr: &[u16] = &[1];

    assert!(matches!(arr, [..]));
    assert!(matches!(arr, [x, ..]));
    match arr {
        [x, ..] => println!("{}", x),
        [..] => println!("arr: {:?}", arr),
        _ => println!("default"),
    }
}

#[test]
fn test_bind_with_xor() {
    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}
