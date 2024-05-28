pub trait Shape {
    fn area(&self) -> f64;
}
pub struct Square {
    pub name: String,
    pub side: f64,
}

pub struct Triangle {
    pub name: String,
    pub base: f64,
    pub height: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}
