// use std::fs;
#[derive(Debug)]

struct Rectangle{
  width: u32,
  height: u32
}
impl Rectangle {
  fn area(&self) ->u32{
    &self.width * &self.height
  }
  fn squre(size:u32) ->Rectangle{
    Rectangle{
      width: size,
      height:size
    }
  }
}
enum Coin{
  Penny,
  Nickel,
  Dime,
  Quarter(UsState)
}

#[derive(Debug)]
enum IpAddressType{
  ipv4,
  ipv6,
}
#[derive(Debug)]
enum UsState{
  Allabama,
  Alaska,
}

#[derive(Debug)]
enum Message{ 
  Address(String),
  Age(String),
  Name(String),
  School{title:String, location:String}
}
fn value_in_cents(coin:Coin) -> u32 {
  match coin {
        Coin::Penny=>1,
        Coin::Nickel=>{
          println!("nicker");
          5
        },
        Coin::Dime=>10,
        Coin::Quarter(state)=>{
          println!("State quarter from {:?}!", state);
          25
        }

  }
}
fn main() {
  // let url = "https://www.rust-lang.org/";
  // let output = "rust.md";
  
  // println!("Fetching url: {}", url);
  // let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  // println!("Converting html to markdown...");
  // let md = html2md::parse_html(&body);

  // fs::write(output, md.as_bytes()).unwrap();
  // println!("Converted markdown has been saved in {}.", output);
  let message1 = Message::School { title: String::from("清华大学"), location: String::from("北京") };
  let message2 = Message::Name(String::from("lee"));

  let ipv6 = IpAddressType::ipv6;
  let ipv4 = IpAddressType::ipv4;

  let rect1 = Rectangle { width: 100, height: 100 };
  let area = rect1.area();
  let squre1 = Rectangle::squre(100);
  let a:Option<i32> = Some(1);
  let b = Some(20);
  let c:Option<String> = None;
  let coin = Coin::Penny;
  let coin2 = Coin::Quarter(UsState::Alaska);

  let coin1 = value_in_cents(coin2);
  println!("{}", area);
  println!("{:?}",squre1);
  println!("{:?}",message1);
  println!("{:?}",message2);
  println!("{:?}",ipv4);
  println!("{:?}",a);
  println!("{:?}",b);
  println!("{:?}",c);
  println!("{:?}",coin1)



}


// fn computed_area(rect: &Rectangle) ->u32{
//   rect.width * rect.height
// }

use std::option;
