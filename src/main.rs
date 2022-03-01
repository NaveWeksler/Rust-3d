mod window;

<<<<<<< HEAD

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() {
    window::win::Win::new(WIDTH, HEIGHT).start();
    
=======
fn main() {
    window::window::Win::new(500usize, 500usize).start();
>>>>>>> ccbc06b (Finished 3d base + fixed 2d draw)
}
