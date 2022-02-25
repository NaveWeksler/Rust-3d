use crate::window::win as Win;
//use crate::window::vector::vec2d;

pub struct Point {
    pub x: i32,
    pub y: i32
}

pub struct Triangle {
    v1: Point, 
    v2: Point, 
    v3: Point,
    color: u32
}

fn fill_bottom_flat_triangle(v1: &Point, v2: &Point, v3: &Point, color: u32, win: &mut Win::Win) {
    let invs1 = (v2.x - v1.x) as f32 / (v2.y - v1.y) as f32;
    let invs2 = (v3.x - v1.x) as f32 / (v3.y - v1.y) as f32;
    let (mut curx1, mut curx2) = (v1.x as f32,  v1.x as f32);
    
    for y in v1.y..=v2.y {
        draw_line(curx1 as usize, y as usize, curx2 as usize, y as usize, color, win);
        curx1 += invs1;
        curx2 += invs2;
    }
}

fn fill_top_flat_triangle(v1: &Point, v2: &Point, v3: &Point, color: u32, win: &mut Win::Win) {
    let invs1 = (v3.x - v1.x) as f32 / (v3.y - v1.y) as f32;
    let invs2 = (v3.x - v2.x) as f32 / (v3.y - v2.y) as f32;
    let (mut curx1, mut curx2) = (v3.x as f32,  v3.x as f32);
    
    for y in (v1.y..v3.y).rev() {
        draw_line(curx1 as usize, y as usize, curx2 as usize, y as usize, color, win);
        curx1 -= invs1;
        curx2 -= invs2;
    }
}

impl Triangle {
    pub fn new(mut v1: Point, mut v2: Point, mut v3: Point, color: u32) -> Triangle {
        let mut temp: Point;
        if v2.y < v1.y {
            temp = v2;
            v2 = v1;
            v1 = temp;
        }
        if v3.y < v1.y {
            temp = v3;
            v3 = v1;
            v1 = temp;
        }
        if v3.y < v2.y {
            temp = v3;
            v3 = v2;
            v2 = temp;
        }

        return Triangle {
            v1: v1,
            v2: v2,
            v3: v3,
            color: color,
        }
    }

    

    pub fn draw(&self, win: &mut Win::Win) {
        if self.v2.y == self.v3.y {
            fill_bottom_flat_triangle(&self.v1, &self.v2, &self.v3, self.color, win);
        }
        else if self.v1.y == self.v2.y {
            fill_top_flat_triangle(&self.v1, &self.v2, &self.v3, self.color, win);
        }
        else {
            
            let v4 = Point {
                x: (self.v1.x + ((self.v2.y - self.v1.y) / (self.v3.y - self.v1.y)) * (self.v3.x - self.v1.x)),
                y: self.v2.y
            };
            
            fill_bottom_flat_triangle(&self.v1, &self.v2, &v4, self.color, win);
            fill_top_flat_triangle(&self.v2, &v4, &self.v3, self.color, win);
        }

    }
}

fn draw_line(mut sx: usize, mut sy: usize, mut ex: usize, mut ey: usize, color: u32, win: &mut Win::Win) {
    if ex < sx {
        let temp = sx;
        sx = ex;
        ex = temp;
    }
    if ey < sy {
        let temp = sy;
        sy = ey;
        ey = temp;
    }
    draw_line_opt(sx, sy, ex, ey, color, win);
}

fn draw_line_opt(sx: usize, sy: usize, ex: usize, ey: usize, color: u32, win: &mut Win::Win) {
    //Bresenham's line algorithm
    let (dx, dy) = (((ex - sx) as i32).abs(),  -((ey - sy) as i32).abs());
    let mut error = dx + dy;

    let (mut x, mut y) : (usize, usize) = (sx, sy);
    let mut e2 : i32;

    loop {
        win.buffer[x + y*win.width] = color;
        if x == ex && y == ey{
            break;
        }
        e2 = 2 * error;
        if e2 >= dy {
            if x == ex {break}
            error = error + dy;
            x += 1;
        }
        if e2 <= dx {
            if y == ey {break}
            error = error + dx;
            y += 1;
        }
    }
}


pub struct Line {
    sx: usize,
    sy: usize,
    ex: usize,
    ey: usize,
    color: u32
}

impl Line {
    pub fn new(mut sx: usize, mut sy: usize, mut ex: usize, mut ey: usize, color: u32) -> Line {
        if ex < sx {
            let temp = sx;
            sx = ex;
            ex = temp;
        }
        if ey < sy {
            let temp = sy;
            sy = ey;
            ey = temp;
        }

        return Line {
            sx: sx,
            sy: sy,
            ex: ex,
            ey: ey,
            color: color,
        }
    }

    pub fn draw(&self, win: &mut Win::Win) {
        draw_line_opt(self.sx, self.sy, self.ex, self.ey, self.color, win);
    }
}
    //     //Bresenham's line algorithm
    //     let (dx, dy) = (((self.ex-self.sx)as i32).abs(),  -((self.ey - self.sy) as i32).abs());
    //     let mut error = dx + dy;

    //     let (mut x, mut y) : (usize, usize) = (self.sx, self.sy);
    //     let mut e2 : i32;

    //     loop {
    //         win.buffer[x + y*win.width] = self.color;
    //         if x == self.ex && y == self.ey{
    //             break;
    //         }
    //         e2 = 2 * error;
    //         if e2 >= dy {
    //             if x == self.ex {break}
    //             error = error + dy;
    //             x += 1;
    //         }
    //         if e2 <= dx {
    //             if y == self.ey {break}
    //             error = error + dx;
    //             y += 1;
    //         }
    //     }
    // }

/*
     let (dx, dy): (i32, i32) = ( (self.ex as i32) - (self.sx as i32),(self.ey as i32) - (self.sy as i32));
        let mut d = 2*dy - dx;
        let mut y = self.sy;
        for x in self.sx..self.ex {
            win.buffer[x + y*win.width] = self.color;
            if d > 0 {
                y += 1;
                d = d - 2*dx;
            }
            d = d + 2*dy;
        } 
    */

