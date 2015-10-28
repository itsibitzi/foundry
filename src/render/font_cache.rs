pub struct FontCache {
    cache: HashMap<String, FontTexture>,
}

impl FontCache {
    pub fn new() {
        FontCache {
            cache: HashMap::new(),
        }
    }

    pub fn add_font<P: AsRef<Path>>(name: &str, path: P, size: u32) -> Result<(), Error>{

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
