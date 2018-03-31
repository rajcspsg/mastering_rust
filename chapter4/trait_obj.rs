use std::fmt::Debug;

#[derive(Debug)]
struct Point {
    x: i8,
    y: i8
}

#[derive(Debug)]
struct ThreeDimPoint {
    x: i8,
    y: i8,
    z: i8
}

fn main() {
    let point: Point = Point{x: 1, y: 3};
    let threed_point = ThreeDimPoint{x: 3, y: 6, z: 9};

    let mut x: &Debug = &point as &Debug;
    println!("1: {:?}", x);

    x = &threed_point ;
    println!("2: {:?}", x);
}