//pub use self::window::Window;
pub use self::font_cache::FontCache;
pub use self::text_panel::TextPanel;

mod font_cache;
mod text_panel;

pub struct RenderMessage {
    pub redness: f32,
}
