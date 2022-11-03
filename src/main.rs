#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another_rectangle: &Rectangle) -> bool {
        self.width > another_rectangle.width && self.height > another_rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 20,
        height: dbg!(30 * scale),
    };
    let rect2 = Rectangle {
        width: 10,
        height: 50,
    };

    println!("Area ===>> {}", rect1.area());

    dbg!(&rect1);

    println!("Test of rect1: {:#?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(60);

    println!("Test of square: {:#?}", square);
}
