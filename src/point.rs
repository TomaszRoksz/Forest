#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
}

impl Point{
    pub fn new(x: i32) -> Self{
        Point { x }
    }
}
