// 练习1
fn main() {
    let x = 5;
    // 填写空白处
    let p = &x;
 
    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
 }

// 练习2

fn main() {
    let x = 5;
    let y = &x;

    // 只能修改以下行
    assert_eq!(5, *y);
}


// 练习3
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&mut s)
}

fn borrow_object(s: &String) {}


// 练习4
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s)
}

fn push_str(s: &mut String) {
    s.push_str("world")
}
// 练习5

fn main() {
    let mut s = String::from("hello, ");

    // 填写空白处，让代码工作
    let p = &mut s;
    
    p.push_str("world");
}
// 练习8
fn main() {
    // 通过修改下面一行代码来修复错误
    let mut s = String::from("hello, ");

    borrow_object(&mut s)
}

fn borrow_object(s: &mut String) {}