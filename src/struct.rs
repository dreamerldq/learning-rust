//1
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("basketball")
    };
} 

//3
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let v = Color(0,127,255);
    check_color(v);
}   

fn check_color(p: Color) {
    let (x, y, z) = (p.0,p.1,p.2);
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
 }

 //4 
 struct  Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18? 
    p.age = 30;

    // 填空
    p.name = String::from("sunfei");
}

//5
// 填空
struct Person {
    name: String,
    age: u8,
}
fn main() {} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name
    }
}

//6 
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);
} 

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

