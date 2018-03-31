#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

fn main() {

    let origin: Point = Point {x: 4.0, y: 5.0};
    println!("Hello, world!");
    println!("origin is {:?}", origin);
    println!("origin.x is {}", origin.x);
}
