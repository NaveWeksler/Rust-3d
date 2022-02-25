use crate::window::render::Point;

struct Canvas { // plane from (0,0) to (width, height)
    ex: i32,
    ey: i32
}

impl Canvas {
    fn project(&self, v: Point) {
        
    }
}

pub struct Camera {
    screen: Canvas
}