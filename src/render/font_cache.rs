use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;

use glium::backend::Facade;
use glium_text::FontTexture;

pub struct FontCache {
    cache: HashMap<String, FontTexture>,
}

impl FontCache {
    pub fn new() -> FontCache {
        FontCache {
            cache: HashMap::new(),
        }
    }

    pub fn add_font<P, F>(&mut self, name: &str, path: P, size: u32, display: &F)
        -> Result<(), Error> where P: AsRef<Path>, F: Facade {
            let file = match File::open(path) {
                Ok(file) => file,
                Err(why) => return Err(Error::Io(why)),
            };

            let texture = match FontTexture::new(display, file, size) {
                Ok(t) => t,
                Err(_) => return Err(Error::UnknownFontTexture),
            };

            self.cache.insert(name.to_string(), texture);

            Ok(())
        }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
        UnknownFontTexture {
            description("unknown font texture error")
            display("Failed to create texture for unknown reasons")
        }
    }
}
