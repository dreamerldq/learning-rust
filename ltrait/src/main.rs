use::std::fmt::Display;
fn main() {
    println!("Hello, world!");
}

pub trait Summary {
    fn summary(&self) ->String;
}

struct NewsArtical {
    author: String,
    age:i32, 
    create_time:i32,
}
impl Summary for NewsArtical {
    fn summary(&self) -> String {
        println!("{}", self.author);
        String::from("sss")
    }
}

pub fn notify1(item: impl Summary + Display){
    println!("{}", item.summary());
}
pub fn notify3(item1: impl Summary, item2: impl Summary + Display){
    println!("{}", item1.summary());
}
pub fn notify<T:Summary + Display>(item:T){
    println!("{}", item.summary());
}
pub fn notify4<T:Summary + Display>(item:T, item1:T){
    println!("{}", item.summary());
}

pub fn notify5<T,U>(a:T,b:U)->String
    where T:Summary + Display,
    U:Summary + Display
{
    format!("{}", a.summary())
}

struct Pair<T>{
    x:T,
    y:T,
}

impl<T> Pair<T> {
    fn new(x:T, y: T)->Self{
        Self{x, y}
    }
}
impl<T:Display + PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if(self.x>=self.y){
            println!("{}",self.x);
        }else{
            println!("{}",self.y);
        }
    }
}