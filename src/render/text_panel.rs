use math::{Matrix, Point, Vector2};

pub struct TextPanel {
    origin: Point,
    width: i32,
    height: i32,
    font_size: f32,
}

impl TextPanel {
    pub fn new(origin: Point, width: i32, height: i32, font_size: f32) -> TextPanel {
        TextPanel {
            origin: origin,
            width: width,
            height: height,
            font_size: font_size,
        }
    }

    pub fn to_matrix(&self, fb_w: f32, fb_h: f32) -> Matrix {
        let origin: Vector2 = self.origin.into();

        Matrix::scale(Vector2::new(2.0, 0.2)) * Matrix::translate(origin)
    }
}
