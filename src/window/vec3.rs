#[derive(Debug)]
pub struct Vec3 {
    pub y: f32,
    pub x: f32,
    pub z: f32
}


impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn add(&self, other: &Vec3) -> Vec3{
        return Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn dot_product(&self, other: &Vec3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn len(&self) -> f32 {
        return (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
    }

    pub fn normalize(&self) -> Vec3{
        let l = self.len();
        return Vec3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l
        }
    }
    
    pub fn sub(&self, other: &Vec3) -> Vec3{
        return Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }

    pub fn cross(&self, other: &Vec3) -> Vec3{
        return Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x
        }
    }
}