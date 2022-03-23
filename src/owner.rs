//练习1
fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}

//练习2
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// 只能修改下面的代码!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}


// 练习4
fn main() {
    let mut s = String::from("hello, world");

    s = print_str(s);

    println!("{}", s);
}

fn print_str(s: String)->String  {
    println!("{}",s);
    s
}

// 练习5   由基本类型构成的元组也具备copy属性
// 不要使用 clone，使用 copy 的方式替代
fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}


// 练习6 当所有权转移时，可变性也可以随之改变。
fn main() {
    let s = String::from("hello, ");
    
    // 只修改下面这行代码 !
    let mut s1 = s;

    s1.push_str("world");
    println!("{}", s1);
}
