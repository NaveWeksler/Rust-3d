use minifb::{Key, Window, WindowOptions};
use crate::window::render;
use crate::window::objects;

pub struct Win {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    window: Window,
    objects: Vec<objects::Mesh>
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
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        window.set_position(200, 200);

        return Win {
            buffer: buffer,
            width: width,
            height: height,
            window: window,
            objects: Vec::new()
        }

    
    }

    pub fn start(&mut self) {
        let line = render::Line::new(200, 200, 10, 10, 0xFF0000);
        let line2 = render::Line::new(500, 200, 0, 100, 0xFF00FF);
        
        let tr = render::Triangle::new(render::Point{x: 50, y: 50}, render::Point{x: 100, y: 0}, render::Point{x: 100, y: 100}, 0x00FF00);

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // for i in self.buffer.iter_mut() {
            //     *i = 0x000000;
            // }
            
            // for y in 0..self.height {
            //     self.buffer[self.width*y] = 0xFF0000;
            //     self.buffer[1+self.width*y] = 0xFF0000;
            // }
    
            // for x in 0..self.width {
            //     self.buffer[x] = 0x0000FF;
            //     self.buffer[self.height*self.width - self.width + x] = 0x00FF00
            // }
            line.draw(self);
            line2.draw(self);
            tr.draw(self);
    
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            self.window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap();
        }
    }
}


