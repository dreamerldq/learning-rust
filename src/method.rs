struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 完成 area 方法，返回矩形 Rectangle 的面积
    fn area(&self) ->u32 {
        &self.width * &self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);
}

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self)  {
        println!("the current state is {}", &self.color);
    }
}
fn main() {
    let light = TrafficLight{
        color: "red".to_owned(),
    };
    // 不要拿走 `light` 的所有权
    light.show_state();
    // 否则下面代码会报错
    println!("{:?}", light);
}



struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 使用 `Self` 填空
    pub fn show_state(&self)  {
        println!("the current state is {}", &self.color);
    }

    // 填空，不要使用 `Self` 或其变体
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
        
    }
}
fn main() {}



struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 1. 实现下面的关联函数 `new`,
    // 2. 该函数返回一个 TrafficLight 实例，包含 `color` "red"
    // 3. 该函数必须使用 `Self` 作为类型，不能在签名或者函数体中使用 `TrafficLight`
    pub fn new(color: String) ->TrafficLight{
        TrafficLight{color}
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light = TrafficLight::new("red".to_string());
    assert_eq!(light.get_state(), "red");
}