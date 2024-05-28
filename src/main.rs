pub mod shapes;

use crate::shapes::Shape;
use crate::shapes::Square;
use crate::shapes::Triangle;

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    println!("Hello, shapes!");

    // explicit struct and trait declarations
    let square1: Square = Square {
        name: String::from("square1"),
        side: 2.0,
    };

    let triangle1 = Triangle {
        name: String::from("triangle1"),
        base: 2.0,
        height: 2.0,
    };

    // OOP style invocation
    println!(
        "shape::square: name: {}, area: {}",
        square1.name,
        square1.area()
    );

    println!(
        "shape::triangle: name: {}, area: {}",
        triangle1.name,
        triangle1.area()
    );

    // alternate invocation
    println!(
        "shape::square: name: {}, area: {}",
        square1.name,
        shapes::Shape::area(&square1)
    );

    println!(
        "shape::triangle: name: {}, area: {}",
        triangle1.name,
        shapes::Shape::area(&square1)
    );

    // let mut shapes: Vec<Shape> = vec![square1, triangle1];

    // NOTE:
    // - will not work
    // - i.e., need to use dyn train object types
    // let shapes: Vec<Shape> = vec![square1], triangle1;
    let shapes: Vec<&dyn Shape> = vec![&square1, &triangle1];
    let mut total_area: f64 = 0.0;

    for shape in &shapes {
        println!("shape::{}: area: {}", type_of(shape), shape.area());
        total_area += shape.area();
    }
    println!("total_area: {}", total_area);

    // Boxed types
    let shapes2: Vec<Box<dyn Shape>> = vec![
        Box::new(Square {
            name: String::from("square1"),
            side: 2.0,
        }),
        Box::new(Triangle {
            name: String::from("triangle1"),
            base: 2.0,
            height: 2.0,
        }),
    ];

    let mut total_area2: f64 = 0.0;
    for shape in shapes2.iter() {
        println!("shape::{}: area: {}", type_of(shape), shape.area());
        total_area2 += shape.area();
    }
    println!("total_area2: {}", total_area2);
}
