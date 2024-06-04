#[doc = include_str!("./shapes-doc.md")]
#[deny(missing_docs)]

pub fn some_lib_func(x: i32) -> i32 {
    println!("x: {}", x);
    x
}
pub trait Shape {
    fn get_area(&self) -> f64;
}

pub mod square {
    use super::Shape;
    pub struct Square {
        pub name: String,
        pub side: f64,
    }
    impl Shape for Square {
        fn get_area(&self) -> f64 {
            area(self.side)
        }
    }
    fn area(side: f64) -> f64 {
        side * side
    }
}

pub mod triangle {
    use super::Shape;
    pub struct Triangle {
        pub name: String,
        pub base: f64,
        pub height: f64,
    }
    impl Shape for Triangle {
        fn get_area(&self) -> f64 {
            area(self.base, self.height)
        }
    }
    fn area(base: f64, height: f64) -> f64 {
        0.5 * base * height
    }
}

#[cfg(test)]
mod tests {
    use super::some_lib_func;
    use super::square::Square;
    use super::triangle::Triangle;
    use super::Shape;

    #[test]
    fn call_some_lib_func() {
        let x = some_lib_func(123);
        assert_eq!(x, 123)
    }
    #[test]
    fn square_get_area() {
        let square1 = Square {
            name: String::from("square1"),
            side: 2.0,
        };
        assert_eq!(square1.get_area(), 4.0)
    }
    #[test]
    fn triangle_get_area() {
        let triangle1 = Triangle {
            name: String::from("triangle1"),
            base: 2.0,
            height: 2.0,
        };
        assert_eq!(triangle1.get_area(), 2.0)
    }
}
