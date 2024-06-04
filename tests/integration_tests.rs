use mylib::square::Square;
use mylib::triangle::Triangle;
use mylib::Shape;

#[cfg(test)]
#[test]
/// Square area test
fn square_get_area() {
    let square1: Square = Square {
        name: String::from("square1"),
        side: 2.0,
    };
    assert_eq!(square1.get_area(), 4.0)
}

#[cfg(test)]
#[test]
/// Triangle area test
fn triangle_get_area() {
    let triangle1: Triangle = Triangle {
        name: String::from("triangle1"),
        base: 2.0,
        height: 2.0,
    };
    assert_eq!(triangle1.get_area(), 2.0)
}

#[cfg(test)]
#[test]
/// Iterate over shapes to compute area
fn iterate_shapes_get_area() {
    let square1: Square = Square {
        name: String::from("square1"),
        side: 2.0,
    };
    let triangle1: Triangle = Triangle {
        name: String::from("triangle1"),
        base: 2.0,
        height: 2.0,
    };
    let shapes: Vec<&dyn Shape> = vec![&square1, &triangle1];
    let mut total_area: f64 = 0.0;

    for shape in &shapes {
        total_area += shape.get_area();
    }
    assert_eq!(total_area, 6.0)
}
