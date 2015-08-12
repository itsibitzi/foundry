use sdl2;
use sdl2::Sdl;
use sdl2::render::Renderer;

pub struct Window<'a> {
    context: Sdl,
    renderer: Renderer<'a>,
    should_close: bool,
}

impl<'a> Window<'a> {
    pub fn new(title: &str, width: u32, height: u32) -> Window<'a> {
        let sdl_context = match sdl2::init().video().build() {
            Ok(sdl) => sdl,
            // Can't meaningfully do anything - will replace panics later
            Err(why) => panic!("Failed to create sdl context: {}", why),
        };

        let window = match sdl_context.window(title, width, height).build() {
            Ok(window) => window,
            Err(why) => panic!("Failed to create a window: {}", why),
        };

        let renderer = match window.renderer().present_vsync().build() {
            Ok(renderer) => renderer,
            Err(why) => panic!("Failed to create renderer: {}", why),
        };

        Window {
            context: sdl_context,
            renderer: renderer,
            should_close: false,
        }
    }

    pub fn process_events(&mut self) {
        for event in self.context.event_pump().poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    self.should_close = true;
                }
                _ => {},
            }
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn present(&mut self) {
        self.renderer.clear();
        self.renderer.present();
    }
}
