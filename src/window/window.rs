use minifb::{Key, Window, WindowOptions};
use super::camera::Camera;
use super::camera::mul_mat4;
use super::objects::Triangle;
use super::objects::Mesh;
use super::vec3::Vec3;
use super::render2d;
use std::time::Instant;
use std::thread::sleep;

pub struct Win {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    window: Window,
    
}

impl Win {
    pub fn new(width: usize, height: usize) -> Win{
        let buffer: Vec<u32> = vec![0; width * width];
    
        let mut window = Window::new(
            "Win",
            width,
            width,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // from_micros(16600)  Limit to max ~60 fps update rate
        //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        window.limit_update_rate(Some(std::time::Duration::from_millis(1000/10)));
        window.set_position(200, 200);

        return Win {
            buffer: buffer,
            width: width,
            height: height,
            window: window,
            
        }

    
    }

    pub fn start(&mut self) {
        let cam = Camera::new(self.width as f32 - 1f32, self.height as f32 - 1f32);
        
        let mesh = Mesh::new(
            vec![
                Triangle::new([ Vec3::new(0f32, 0f32, 0f32),    Vec3::new(0f32, 1f32, 0f32),    Vec3::new(1f32, 1f32, 0f32)]),
                Triangle::new([ Vec3::new(0f32, 0f32, 0f32),    Vec3::new(1f32, 1f32, 0f32),    Vec3::new(1f32, 0f32, 0f32)]),
        
                // EAST                                                      
                 Triangle::new([ Vec3::new(1f32, 0f32, 0f32),    Vec3::new(1f32, 1f32, 0f32),    Vec3::new(1f32, 1f32, 1f32)]),
                 Triangle::new([ Vec3::new(1f32, 0f32, 0f32),    Vec3::new(1f32, 1f32, 1f32),    Vec3::new(1f32, 0f32, 1f32)]),
        
                // NORTH                                                     
                 Triangle::new([ Vec3::new(1f32, 0f32, 1f32),    Vec3::new(1f32, 1f32, 1f32),    Vec3::new(0f32, 1f32, 1f32)]),
                 Triangle::new([ Vec3::new(1f32, 0f32, 1f32),    Vec3::new(0f32, 1f32, 1f32),    Vec3::new(0f32, 0f32, 1f32)]),
        
                // WEST                                                      
                 Triangle::new([ Vec3::new(0f32, 0f32, 1f32),    Vec3::new(0f32, 1f32, 1f32),    Vec3::new(0f32, 1f32, 0f32)]),
                 Triangle::new([ Vec3::new(0f32, 0f32, 1f32),    Vec3::new(0f32, 1f32, 0f32),    Vec3::new(0f32, 0f32, 0f32)]),
        
                // TOP                                                       
                Triangle::new([ Vec3::new(0f32, 1f32, 0f32),    Vec3::new(0f32, 1f32, 1f32),    Vec3::new(1f32, 1f32, 1f32)]),
                Triangle::new([ Vec3::new(0f32, 1f32, 0f32),    Vec3::new(1f32, 1f32, 1f32),    Vec3::new(1f32, 1f32, 0f32)]),
        
                // BOTTOM                                                    
                Triangle::new([ Vec3::new(1f32, 0f32, 1f32),     Vec3::new(0f32, 0f32, 1f32),     Vec3::new(0f32, 0f32, 0f32) ]),
                Triangle::new([ Vec3::new(1f32, 0f32, 1f32),    Vec3::new(0f32, 0f32, 0f32),    Vec3::new(1f32, 0f32, 0f32 )])
            ]
            //Triangle::new([Vec3::new(1f32, 0f32, 1f32), Vec3::new(0f32, 0f32, 1f32), Vec3::new(0f32, 0f32, 0f32)]),
            //Triangle::new([Vec3::new(1f32, 0f32, 1f32), Vec3::new(0f32, 0f32, 0f32), Vec3::new(1f32, 0f32, 0f32)]),
        );

        let mut mat_rot_z = [[0f32; 4]; 4];
        let mut mat_rot_x = [[0f32; 4]; 4];

        
        let mut rot_angle = 1f32.to_radians();


        let mut tr_arr: [Vec3; 3];
        //const oneRadian: f32 = std::f32::consts::PI / 180f32;

        let mut timer = Instant::now();
        let mut play = true;

        while self.window.is_open() {
           
            timer = Instant::now();

            // Rotation
            mat_rot_z[0][0] = f32::cos(rot_angle);
            mat_rot_z[0][1] = f32::sin(rot_angle);
            mat_rot_z[1][0] = -f32::sin(rot_angle);
            mat_rot_z[1][1] = f32::cos(rot_angle);
            mat_rot_z[2][2] = 1f32;
            mat_rot_z[3][3] = 1f32;
            
            mat_rot_x[0][0] = 1f32;
            mat_rot_x[1][1] = f32::cos(rot_angle * 0.5);
            mat_rot_x[1][2] = f32::sin(rot_angle * 0.5);
            mat_rot_x[2][1] = -f32::sin(rot_angle * 0.5);
            mat_rot_x[2][2] = f32::cos(rot_angle * 0.5);
            mat_rot_x[3][3] = 1f32;

            for tr in mesh.triangles.iter() {
                let mut translate_tr = Triangle::new([
                    Vec3::new(tr.vertex[0].x, tr.vertex[0].y, tr.vertex[0].z),
                    Vec3::new(tr.vertex[1].x, tr.vertex[1].y, tr.vertex[1].z),
                    Vec3::new(tr.vertex[2].x, tr.vertex[2].y, tr.vertex[2].z),
               ]);

                translate_tr.vertex[0] = mul_mat4(&translate_tr.vertex[0], &mat_rot_z);
                translate_tr.vertex[1] = mul_mat4(&translate_tr.vertex[1], &mat_rot_z);
                translate_tr.vertex[2] = mul_mat4(&translate_tr.vertex[2], &mat_rot_z);

                translate_tr.vertex[0] = mul_mat4(&translate_tr.vertex[0], &mat_rot_x);
                translate_tr.vertex[1] = mul_mat4(&translate_tr.vertex[1], &mat_rot_x);
                translate_tr.vertex[2] = mul_mat4(&translate_tr.vertex[2], &mat_rot_x);

                translate_tr.vertex[0].z += 3f32;
                translate_tr.vertex[1].z += 3f32;
                translate_tr.vertex[2].z += 3f32;

                tr_arr = cam.convert_triangles(translate_tr);
                
                tr_arr[0].x += 1f32; tr_arr[0].y += 1f32;
                tr_arr[1].x += 1f32; tr_arr[1].y += 1f32;
                tr_arr[2].x += 1f32; tr_arr[2].y += 1f32;

                tr_arr[0].x *= 0.5f32 * self.width as f32; tr_arr[0].y *= 0.5f32 * self.height  as f32;
                tr_arr[1].x *= 0.5f32 * self.width as f32; tr_arr[1].y *= 0.5f32 * self.height as f32;
                tr_arr[2].x *= 0.5f32 * self.width as f32; tr_arr[2].y *= 0.5f32 * self.height as f32;
                //println!("{:?}", tr_arr);
                // draw
                render2d::drawTriangle(render2d::Point::new(tr_arr[0].x, tr_arr[0].y), render2d::Point::new(tr_arr[1].x, tr_arr[1].y), render2d::Point::new(tr_arr[2].x, tr_arr[2].y), 0xFFFFFF, self);
            }

            self.window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap();
            
            for px in self.buffer.iter_mut() {
                 *px = 0x000000;
            }

            if self.window.is_key_released(Key::W) {
                play = !play;
            }
            if play {
                rot_angle += timer.elapsed().as_millis() as f32 / 360f32;
            }
            

            sleep(std::time::Duration::from_secs_f32(1f32/20f32))
        }
    }
}