enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    println!("Hello, world!");
    let  mut v:Vec<i32> = Vec::new();
    v.push(1);
    let v1 = vec![1,2,3,4];
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(12.3),
        SpreadsheetCell::Text(String::from("Hello, world!"))
    ];
    let mut str1 = "Hello, world!".to_string();
    let str2 = String::from("Hello, world2");
    str1.push_str("aaa");
    str1.push('l');
    str1.push_str(&str2);
    println!("{}",str1); 


    let s1 =String::from("Hello,");
    let s2 = String::from("world");
    let s3 = s1+&s2;
    // + 号运算符其实是调用了一个函数，S1传递进去后，就转换了所有权，所以后面不能再用了，而S2是做了copy
    // println!("{}",s1);
    println!("{}",s2);
    println!("{}",s3);
    let s4 =String::from("beijing");
    let s5 = String::from("chaoyang");
    let s6 = String::from("lishuiqiao"); 
    let s = format!("{}-{}-{}", s4,s5,s6);


    
} 
