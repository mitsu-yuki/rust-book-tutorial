struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} squeare pixels.",
        area(&rect1)
    )
}

fn area(rectrangle: &Rectangle) -> u32 {
    rectrangle.width * rectrangle.height
}
