fn main() {
    let rect1 = (20, 30);

    println!("Area: {}", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
