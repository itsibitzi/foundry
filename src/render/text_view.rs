pub struct TextView {
    top: f32,
    left: f32,
    width: f32,
    height: f32,
}

impl TextView {
    pub fn new() -> TextView {
        TextView {
            top: 0.0,
            left: 0.0,
            width: 300.0,
            height: 300.0
        }
    }
}
