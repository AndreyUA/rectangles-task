#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 20,
        height: dbg!(30 * scale),
    };

    println!("Area ===>> {}", rect1.area());

    dbg!(&rect1);

    println!("Test of rect1: {:#?}", rect1);
}
