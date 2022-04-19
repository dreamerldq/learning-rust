use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
//    let f = File::open("hello.txt")
//   let file =  match f {
//        Ok(file) => file,
//        Err(err) => panic!("Error opening file {:?}",err),
//    }
// let file = match f {
//     Ok(file) => file,
//     Err(err) => match err.kind() {
//         ErrorKind::NotFound => match File::crate("hello.txt") {
//             Ok(fc)=> fc,
//             Err(err) => panic!("Error")
//         },
//         _ => panic!("Error opening file {:?}",  _),
//     }
// }
// let f = File::open("hello.txt").unwrap()
// let f = File::open("hello.txt").expect("无法打开文件")


// fn read_username_from_file() -> Result<String, id::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f{
//         Ok(file) => file,
//         Err(err) => return Err(e),
//     }
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => _,
//         Err(err) => return Err(e),
//     }
//     }
// }

// fn read_username_from_file() -> Result<String, id::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
fn read_username_from_file() -> Result<String, id::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
let result = read_username_from_file();
}
