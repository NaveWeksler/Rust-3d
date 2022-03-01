<<<<<<< HEAD
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
=======
use super::vec3::Vec3;
use super::objects::Triangle;

pub struct Camera {
    mat: [[f32; 4]; 4]
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Camera{
        let (fNear, fFar, fFov) = (0.1f32, 1000f32, 90f32);
        let aspectRatio = height / width;

        println!("VAL: {}", f32::tan((fFov * 0.5).to_radians()));
        let fFovRad = 1f32 / f32::tan((fFov * 0.5).to_radians());
        let mut mat = [[0f32; 4]; 4];
    
        mat[0][0] = aspectRatio * fFovRad;
        mat[1][1] = fFovRad;
        mat[2][2] = fFar / (fFar - fNear);
        mat[3][2] = (-fFar * fNear) / (fFar - fNear);
        mat[2][3] = 1f32;
        println!("{:?}", mat);

        return Camera {
            mat: mat
        };
    }

    pub fn convert_triangles(&self, tr: Triangle) -> [Vec3; 3]{
        return [
            mul_mat4(&tr.vertex[0], &self.mat),
            mul_mat4(&tr.vertex[1], &self.mat),
            mul_mat4(&tr.vertex[2], &self.mat)
        ];
    }
}

pub fn mul_mat4(vec: &Vec3, mat: &[[f32; 4]; 4]) -> Vec3{
    
    
    let (x, y, z) = (
        (vec.x * mat[0][0] + vec.y * mat[1][0] + vec.z * mat[2][0] + mat[3][0]),
        (vec.x * mat[0][1] + vec.y * mat[1][1] + vec.z * mat[2][1] + mat[3][1]),
        (vec.x * mat[0][2] + vec.y * mat[1][2] + vec.z * mat[2][2] + mat[3][2]),
    );

    let w = vec.x * mat[0][3] + vec.y * mat[1][3] + vec.z * mat[2][3] + mat[3][3];
    
    if w != 0f32 {
        return Vec3 {
            x:  x / w,
            y: y / w,
            z: z / w
        }
    }
    
    return Vec3 {
        x: x,
        y: y,
        z: z
    }
    
>>>>>>> ccbc06b (Finished 3d base + fixed 2d draw)
}