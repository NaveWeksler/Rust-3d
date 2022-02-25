mod window;


const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() {
    window::win::Win::new(WIDTH, HEIGHT).start();
    
}
