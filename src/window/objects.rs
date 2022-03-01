use super::vec3::Vec3;

pub struct Triangle {
    pub vertex: [Vec3; 3]
}


impl Triangle {
    pub fn new(vertex: [Vec3; 3]) -> Triangle {
        Triangle {
            vertex: vertex
        }
    }
}

pub struct Mesh {
    pub triangles: Vec<Triangle>
}

impl Mesh {
    pub fn new(trs: Vec<Triangle>) -> Mesh{
        return Mesh {
            triangles: trs,
        }
    }
}