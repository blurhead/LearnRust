use std::mem::size_of_val;
use crate::report::report;

#[test]
fn test_str_() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions {
        println!("{}", region.to_string());
    }
    let mut c = 32;
    let x = "abc啊沙发舒服";
    println!("{}", size_of_val(&x));
    let _d = "hello".to_string();
    c += 1;
    println!("{}", c)
}


#[test]
fn test_str_del() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    string_remove.truncate(3);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(&string_remove);
    assert_eq!(&string_remove, &String::from("试"))
}

#[test]
fn main() {}


#[test]
fn test_str() {
    let a = String::from("abcdefghijklmnopqrstuvw");
    report(size_of_val(&a));
    report(String::len(&a));
    let b: u32 = 4294967295;

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    report(s[0..5].to_string());
    report(s[6..11].to_string());
}

#[test]
fn test_escape() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals can span multiple lines. The linebreak and indentation here -> <- can be escaped too!";
    println!("{}", long_string);


    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r##"And then I said: "There is no escape!""##;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    for (i, c) in "hello我是中国人".chars().enumerate() {
        println!("No {}: {}, size: {}", i, c, size_of_val(&c));
    }

    for c in "我是中国人".bytes() {
        println!("{}", c);
    }
}

#[test]
fn practice_slice() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // 修改数字 `8` 让代码工作
    // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
    println!("{}", size_of_val(slice));
    let ref1 = &slice;
    assert_eq!(size_of_val(ref1), 16);
}


#[test]
fn practice_utf8_slice()
{
    let mut s = String::new();
    s.push_str("hello");

    let v = vec![104, 101, 108, 108, 111];

    // 将字节数组转换成 String
    let s1 = String::from_utf8(v).unwrap();


    assert_eq!(s, s1);

    println!("Success!")
}
