use crate::window::render::Triangle;
use crate::window::camera::Camera;

pub struct Mesh {
    triangels: Vec<Triangle>
}

impl Mesh {
    fn new(arr: Vec<Triangle>) -> Mesh {
        return Mesh {
            triangels: arr
        }
    }

    fn render(&self, camera: Camera) {

    }
}