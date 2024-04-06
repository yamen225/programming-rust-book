use std::iter;
use std::vec::IntoIter;

fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) ->
    iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

// could be rewritten as
fn cyclical_zip_dyn_dispatch(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item = u8>> {
    Box::new(v.into_iter().chain(u.into_iter()).cycle())
}

// without the overhead of the dyn dispatch
fn cyclical_zip_impl(v: Vec<u8>, u: Vec<u8>) -> impl Iterator < Item = u8 > {
    v.into_iter().chain(u.into_iter()).cycle()
}

trait Shape {
    fn new() -> Self;
    fn area(&self) -> f64;
}

struct Circle{
    radius: f64,
}

impl Shape for Circle {
    fn new() -> Circle {
        Circle { radius: 1.0 }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn new() -> Rectangle {
        Rectangle { width: 1.0, height: 1.0 }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Triangle {
    length: f64,
    height: f64,
}

impl Shape for Triangle {
    fn new() -> Triangle {
        Triangle { length: 1.0, height: 1.0 }
    }

    fn area(&self) -> f64 {
        0.5 * self.length * self.height
    }
}

/*
/// can't use it as a factory pattern return type
fn make_shape(shape: str) -> impl Shape {
    match shape {
        "circle" => Circle::new(),
        "rectangle" => Rectangle::new(), // error[E0308]: mismatched types
        "triangle" => Triangle::new(),
        _ => panic!("Unknown shape"),
    }
}

*/

// These functions are the same:

fn print_area(s: impl Shape) {
    println!("Area is {}", s.area());
}

fn print_area_t<T: Shape>(s: T) {
    println!("Area is {}", s.area());
}
