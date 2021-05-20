mod canvas;
mod rect;
mod util;
mod widget;
mod error;
mod style;
mod buffer;
mod text;

pub mod prelude {
    pub use super::error::DrawError;
    pub use super::error::DrawErrorKind;
    pub use super::canvas::Canvas;
    pub use super::rect::Rect;
    pub use super::widget::Widget;
    pub use super::text::Text;
    pub use super::style::Style;
    pub use super::buffer::Buffer;
}