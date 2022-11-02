#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 20,
        height: dbg!(30 * scale),
    };

    println!("Area: {}", area(&rect1));

    dbg!(&rect1);

    println!("Test of rect1: {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
