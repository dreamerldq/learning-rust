use std::env;
use std::path::Path;
fn main() {
    let image_path = env::args().skip(1).next().unwrap();
    let path = Path::new(&image_path);
    let image = image::open(path).unwrap();
    let rotated = image.rotate90();
    rotated.save(path).unwrap();
}
