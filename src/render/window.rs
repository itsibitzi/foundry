use std::path::Path;

use sdl2;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2_ttf::Font;

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

        let window = match sdl_context.window(title, width, height).opengl().build() {
            Ok(window) => window,
            Err(why) => panic!("Failed to create a window: {}", why),
        };

        let mut renderer = match window.renderer().accelerated().present_vsync().build() {
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
                Event::Quit{..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => self.should_close = true,
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

    pub fn write(&mut self, text: &str) {
        let font = Font::from_file(&Path::new("fonts/NotoSans/NotoSans-Regular.ttf"), 28).unwrap();

        let surface = font.render_str_blended(text, Color::RGBA(200, 0, 0, 255)).unwrap();
        let mut texture = self.renderer.create_texture_from_surface(&surface).unwrap();

        self.renderer.set_draw_color(Color::RGBA(195, 217, 255, 255));
        self.renderer.clear();

        let (w, h) = {
            let q = texture.query();
            (q.width, q.height)
        };
        println!("{}, {}", w, h);

        self.renderer.copy(&mut texture, None, Rect::new(((800 - w) / 2) as i32, ((600 - h) / 2) as i32, w, h).unwrap());
        self.present();
    }
}
