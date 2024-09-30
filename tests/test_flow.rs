#[test]
fn test_flow() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

#[test]
fn test_loop() {
    let mut container = [1, 2, 3, 4, 5];
    for i in &container {
        println!("{}", i);
    }

    for (i, v) in container.iter_mut().enumerate() {
        println!("{}", i);
        *v += 1;
    }

    for i in &container {
        println!("{}", i);
    }
}

#[test]
fn test_if() {
    let number = 3;
    // if当作语句来用
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if表达式
    let num = 1;
    let number = if num == 1 {
        5
    } else if num == 2 {
        6
    } else {
        7
    };

    println!("The value of number is: {}", number);
}

#[test]
fn test_for_str_array() {
    let arr = [
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
    ];
    // 没有实现copy trait的数组多次迭代需要借用
    for i in arr.iter() {
        println!("{}", i);
    }
    for i in &arr {
        println!("{}", i);
    }
}
#[test]
fn test_for_in() {
    for i in 1..=5 {
        println!("{}", i);
    }
}

#[test]
fn examination() {
    // 填空
    fn case1() {
        let n = 5;

        if n < 0 {
            println!("{} is negative", n);
        } else if n > 0 {
            println!("{} is positive", n);
        } else {
            println!("{} is zero", n);
        }
    }

    // 修复错误
    fn case2() {
        let n = 5;

        let big_n = if n < 10 && n > -10 {
            println!(" 数字太小，先增加 10 倍再说");

            10 * n
        } else {
            println!("数字太大，我们得让它减半");
            n / 2
        };

        println!("{} -> {}", n, big_n);
    }

    fn case3() {
        for n in 1..=99 {
            // 修改此行，让代码工作
            if n == 100 {
                panic!("NEVER LET THIS RUN")
            }
        }
    }

    // 修复错误，不要新增或删除代码行
    fn case4() {
        let names = [String::from("liming"), String::from("han mei mei")];
        for _ in &names {
            // do something with name...
        }

        println!("{:?}", names);

        let numbers = [1, 2, 3];
        // numbers中的元素实现了 Copy，因此无需转移所有权
        for _ in numbers {
            // do something with name...
        }

        println!("{:?}", numbers);
    }

    fn case5() {
        let a = [4, 3, 2, 1];

        // 通过索引和值的方式迭代数组 `a`
        for (i, v) in a.iter().enumerate() {
            println!("第{}个元素是{}", i + 1, v);
        }
    }

    // 填空，让最后一行的  println! 工作 !
    fn case6() {
        // 一个计数值
        let mut n = 1;

        // 当条件为真时，不停的循环
        while n < 10 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }

            n += 1;
        }

        println!("n 的值是 {}, 循环结束", n);
    }

    // 填空，不要修改其它代码
    fn case7() {
        let mut n = 0;
        for _ in 0..=100 {
            if n == 66 {
                break;
            }
            n += 1;
        }

        assert_eq!(n, 66);
    }

    // 填空，不要修改其它代码
    fn case8() {
        let mut n = 0;
        for _ in 0..=100 {
            if n != 66 {
                n += 1;
                continue;
            }

            break;
        }

        assert_eq!(n, 66);
    }

    // 填空，不要修改其它代码
    fn case9() {
        let mut count = 0u32;

        println!("Let's count until infinity!");

        // 无限循环
        loop {
            count += 1;

            if count == 3 {
                println!("three");

                // 跳过当此循环的剩余代码
                continue;
            }

            println!("{}", count);

            if count == 5 {
                println!("OK, that's enough");

                break;
            }
        }

        assert_eq!(count, 5);
    }

    // 填空
    fn case10() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        assert_eq!(result, 20);
    }

    // 填空
    fn case11() {
        let mut count = 0;
        'outer: loop {
            'inner1: loop {
                if count >= 20 {
                    // 这只会跳出 inner1 循环
                    break 'inner1; // 这里使用 `break` 也是一样的
                }
                count += 2;
            }

            count += 5;

            loop {
                if count >= 30 {
                    break 'outer;
                }

                continue 'outer;
            }
        }

        assert_eq!(count, 30)
    }

    case1();
    case2();
    case3();
    case4();
    case5();
    case6();
    case7();
    case8();
    case9();
    case10();
    case11();
}
