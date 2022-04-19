struct Point<T,U>{
    x:T,
    y:U
}
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
impl <T,U> Point<T, U>{
    fn mixup<V,W>(self, other:Point<V,W>) -> Point<T,W>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}
enum Option<T>{
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
fn main() {
    let p = Point{x:0, y:1};
    // let c = p.x();
    let p1 = Point{x:"hello", y:'c'};
    let p2 = p.mixup(p1);
    println!("p2.x ={}, p2.y={}",p2.x,p2.y);
    let float = Point{x:1.1,y:2.1};
    // let a = Result::Err(21)
    // let b = Result::Ok(11)
    // let list = vec![1,2,3,5,6];
    // let largest = largest(&list);
    // println!("{}", largest);
}

// fn largest<T>(list:&[T]) ->T {
//     let mut largest = list[0];
//     for &num in list {
//         if num>largest {
//             largest = num;
//         }
//     }
//     largest
// }