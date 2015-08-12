//extern crate cargo;
extern crate gl;
extern crate sdl2;

use render::Window;

mod render;
mod session;

fn main() {
    let mut window = Window::new("Foundry", 800, 600);

    loop {
        window.process_events();
        if window.should_close() {
            break;
        }

        window.present();
    }
}
