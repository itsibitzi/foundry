//pub use self::window::Window;
pub use self::text_panel::TextPanel;

mod text_panel;

pub struct RenderMessage {
    pub redness: f32,
}
