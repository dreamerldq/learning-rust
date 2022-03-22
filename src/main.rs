fn main() {
    // greet_world()
    increase();
    let (x, y) = (1, 2);
    let s = sum(x, y);
    print();
    never_return();
    assert_eq!(s, 3);
}
// fn greet_world(){
//     let southern_germany = "111";
//     let chinese = "你好 世界";
//     let english = "hello world";
//     let regions = [southern_germany, chinese, english];
//     for region in regions.iter(){
//         println!("{}", region)
//     }
// }
fn increase(){
    let mut a = 5;
    println!("{}", a);
    a = 6;
    println!("{}",a)
}
fn sum(x: i32, y: i32)->i32 {
     x + y
}
fn print() -> () {
    println!("hello,world");
 }


 fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    loop{
        
    }
    
}
// 表达式一定有返回值，而语句没有返回值。表达式后面一定不能加分号