struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    println!("Area: {}", area(&rect1));

    println!(
        "Test of var. Width: {}, height: {}",
        rect1.width, rect1.height
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
