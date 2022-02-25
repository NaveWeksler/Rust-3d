
pub struct Vec2d {
    x: f32,
    y: f32
}

impl Vec2d {
    pub fn new(x: f32, y: f32) -> Vec2d {
        return Vec2d {
            x: x,
            y: y
        }
    }

    pub fn dot_prudoct(&self, other: &Vec2d) -> f32{
        return self.x * other.x + self.y * other.y;
    }

    pub fn normalize(&self) -> Vec2d{
        let l = self.len();
        return Vec2d {
            x: self.x as f32 / l,
            y: self.x as f32 / l
        }
    }

    pub fn len(&self) -> f32{
        return (self.x*self.x + self.y*self.y).sqrt();
    }
}