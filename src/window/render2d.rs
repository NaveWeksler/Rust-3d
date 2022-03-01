use crate::window::window::Win;

pub struct Point {
     pub x: i32,
     pub y: i32
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        return Point {
            x: x as i32,
            y: y as i32,
        }
    }
}

pub fn drawTriangle(v1: Point, v2: Point, v3: Point, color: u32, win: &mut Win) {
    // let mut temp;
    // if v2.y < v1.y {
    //     temp = v2;
    //     v2 = v1;
    //     v1 = temp;
    // }
    // if v3.y < v1.y {
    //     temp = v3;
    //     v3 = v1;
    //     v1 = temp;
    // }
    // if v3.y < v2.y {
    //     temp = v3;
    //     v3 = v2;
    //     v2 = temp;
    // }
    
    
    // if v2.y == v3.y {
    //     fill_bottom_flat_triangle(&v1, &v2, &v3, color, win);
    // }
    // else if v1.y == v2.y {
    //     fill_top_flat_triangle(&v1, &v2, &v3, color, win);
    // }
    // else {
        
    //     let v4 = Point {
    //         x: (v1.x + ((v2.y - v1.y) / (v3.y - v1.y)) * (v3.x - v1.x)),
    //         y: v2.y
    //     };
        
    //     fill_bottom_flat_triangle(&v1, &v2, &v4, color, win);
    //     fill_top_flat_triangle(&v2, &v4, &v3, color, win);
    // }
    draw_line(v1.x, v1.y, v2.x, v2.y, color, win);
    draw_line(v2.x, v2.y, v3.x, v3.y, color, win);
    draw_line(v3.x, v3.y, v1.x, v1.y, color, win);
}


pub fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, color: u32, win: &mut Win) {
    //Bresenham's line algorithm - unoptimized
    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {plotLineLow(x1, y1, x0, y0, color, win)}
        else{
            plotLineLow(x0, y0, x1, y1, color, win)
        }
    }else {
        if y0 > y1{
            plotLineHigh(x1, y1, x0, y0, color, win)
        }
        else{
            plotLineHigh(x0, y0, x1, y1, color, win)
        }
    }
    

}

fn plotLineLow(x0: i32, y0: i32, x1: i32, y1: i32, color: u32, win: &mut Win){
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }

    let mut D = (2 * dy) - dx;
    let mut y = y0;
    let mut pos: i32;

    for x in x0..x1 {
        pos = (x - 1) + (y - 1) * win.width as i32;
        if pos >= 0 && pos < win.buffer.len() as i32 {
            win.buffer[pos as usize] = color;
        }
       
        if D > 0 {
            y = y + yi;
            D = D + (2 * (dy - dx));
        } else{
            D = D + 2*dy;
        } 
    }  
            
}

fn plotLineHigh(x0: i32, y0: i32, x1: i32, y1: i32, color: u32, win: &mut Win){
    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let mut xi = 1;

    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    
    let mut D = (2 * dx) - dy;
    let mut x = x0;
    let mut pos: i32;

    for y in y0..y1 {
        pos = (x - 1) + (y - 1) * win.width as i32;
        if pos >= 0 && pos < win.buffer.len() as i32 {
            win.buffer[pos as usize] = color;
        }
        
        if D > 0 {
            x = x + xi;
            D = D + (2 * (dx - dy));
        } else {
            D = D + 2*dx;
        }
    }
       
}



// fn fill_bottom_flat_triangle(v1: &Point, v2: &Point, v3: &Point, color: u32, win: &mut Win) {
//     let invs1 = (v2.x - v1.x) as f32 / (v2.y - v1.y) as f32;
//     let invs2 = (v3.x - v1.x) as f32 / (v3.y - v1.y) as f32;
//     let (mut curx1, mut curx2) = (v1.x as f32,  v1.x as f32);
    
//     for y in v1.y..=v2.y {
//         draw_line(curx1 as usize, y as usize, curx2 as usize, y as usize, color, win);
//         curx1 += invs1;
//         curx2 += invs2;
//     }
// }

// fn fill_top_flat_triangle(v1: &Point, v2: &Point, v3: &Point, color: u32, win: &mut Win) {
//     let invs1 = (v3.x - v1.x) as f32 / (v3.y - v1.y) as f32;
//     let invs2 = (v3.x - v2.x) as f32 / (v3.y - v2.y) as f32;
//     let (mut curx1, mut curx2) = (v3.x as f32,  v3.x as f32);
    
//     for y in (v1.y..v3.y).rev() {
//         draw_line(curx1 as usize, y as usize, curx2 as usize, y as usize, color, win);
//         curx1 -= invs1;
//         curx2 -= invs2;
//     }
// }

// fn draw_line_opt(sx: usize, sy: usize, ex: usize, ey: usize, color: u32, win: &mut Win) {
//     //Bresenham's line algorithm
//     let (dx, dy) = (((ex - sx) as i32).abs(),  -((ey - sy) as i32).abs());
//     let mut error = dx + dy;

//     let (mut x, mut y) : (usize, usize) = (sx, sy);
//     let mut e2 : i32;
//     let mut calc: i32;
//     loop {
//         //if ((x as i32 + y as i32 * win.width as i32) as i32) >= 0 {
//             calc = ((x-1) + (y-1) * win.width) as i32;
//             if calc < 0 || calc >= win.buffer.len() as i32 {
//                 break;
//             }
//             //if calc > 0 && calc < (win.width*win.height) as i32 {
//             win.buffer[calc as usize] = color; //calc as usize
//             if x == ex && y == ey{
//                 break;
//             }
//             e2 = 2 * error;
//             if e2 >= dy {
//                 if x == ex {break}
//                 error = error + dy;
//                 x += 1;
//             }
//             if e2 <= dx {
//                 if y == ey {break}
//                 error = error + dx;
//                 y += 1;
//             }
//             //}
            
//         //}

       
//     }
//}