use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use glium::backend::Facade;
use glium::backend::glutin_backend::GlutinFacade;
use glium::DisplayBuild;
use glium::glutin::{Event, WindowBuilder};
use glium::Surface;
use glium_text;
use glium_text::{FontTexture, TextDisplay, TextSystem};

use super::RenderMessage;
use super::matrix;

pub struct Window {
    display: GlutinFacade,
    text_system: TextSystem,
    font: FontTexture,
    tx: Sender<RenderMessage>,
    rx: Receiver<RenderMessage>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        let display = WindowBuilder::new()
            .with_title(title.to_string())
            .with_dimensions(width, height)
            .with_vsync()
            .build_glium()
            .unwrap();

        let text_system = TextSystem::new(&display);

        let font = FontTexture::new(&display, File::open("fonts/NotoSans/NotoSans-Regular.ttf").unwrap(), 24).unwrap();

        let (tx, rx) = mpsc::channel();

        Window {
            display: display,
            tx: tx,
            rx: rx,
        }
    }

    pub fn new_sender(&mut self) -> Sender<RenderMessage> {
        self.tx.clone()
    }

    pub fn run(&mut self) {
        loop {
            let mut target = self.display.draw();
            target.clear_color(1.0, 0.0, 0.0, 1.0);

            match self.rx.try_recv() {
                Ok(msg) => target.clear_color(msg.redness, 0.0, 0.0, 1.0),
                Err(why) => {},
            }

            let text = TextDisplay::new(&self.text_system, &self.font, "This is a tribute!");

            glium_text::draw(&text, &self.text_system, &mut target, matrix::IDENTITY, (1.0, 1.0, 1.0, 1.0));

            target.finish().unwrap();

            for ev in self.display.poll_events() {
                match ev {
                    Event::Closed => return,
                    _ => {},
                }
            }
        }
    }
}
