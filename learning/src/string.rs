//1
fn main() {
    let s: &str = "hello, world";
 }

 //2
 fn main() {
    let s: Box<str> =  "hello, world".into();
    greetings(&s)
 }
 
 fn greetings(s: &str) {
     println!("{}",s)
 }
 

 // 3
fn main() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');
 
    assert_eq!(s, "hello, world!");
 }
 
 //4
 // 修复所有错误，并且不要新增代码行
fn main() {
    let mut s =  String::from("hello");
     s.push(',');
     s.push_str(" world");
     s += &"!".to_string();
 
     println!("{}", s)
 }
 //5
 fn main() {
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1 = s.replace("dogs", "cats");
 
    assert_eq!(s1, "I like cats")
 }
 

 //6
 fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; 
    assert_eq!(s3,"hello,world!");
    println!("{}",s3);
}


//7
fn main() {
    let s =  "hello, world".to_string();
    greetings(s)
 }
 
 fn greetings(s: String) {
     println!("{}",s)
 }

 fn main() {
    let s =  String::from("hello, world");
    greetings(s)
 }
 
 fn greetings(s: String) {
     println!("{}",s)
 }

 //8
 fn main() {
    let s =  "hello, world".to_string();
    let s1: &str = &s;
 }

 //11
 fn main() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
    assert_eq!(h, "h");

    let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
    assert_eq!(h1, "中");
}

//12
fn main() {
    // 填空，打印出 "你好，世界" 中的每一个字符
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}
