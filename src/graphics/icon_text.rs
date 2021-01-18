//! Text widget for rendering icons.
//!
//! Nearly a complete copy of the iced_native::Text widget, but with the
//! icons font as a default font. Maybe I'll find a better way in the future.
//!
//! //! *This API requires the following crate features to be activated: icon_text*
use iced_graphics::{backend, Backend, Font, Primitive, Rectangle, Renderer};
use iced_native::mouse;

use crate::native::icon_text;

use super::icons::ICON_FONT;

/// Text widget with icon font.
///
/// This is an alias of an `iced_native` IconText with an `iced_wgpu::Renderer`.
pub type IconText<Backend> = crate::native::icon_text::IconText<Renderer<Backend>>;

impl<B> icon_text::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Font = Font;

    fn default_size(&self) -> u16 {
        self.backend().default_size()
    }

    fn default_font(&self) -> Self::Font {
        ICON_FONT
    }

    fn measure(
        &self,
        content: &str,
        size: u16,
        font: Self::Font,
        bounds: iced_graphics::Size,
    ) -> (f32, f32) {
        self.backend()
            .measure(content, f32::from(size), font, bounds)
    }

    fn draw(
        &mut self,
        defaults: &Self::Defaults,
        bounds: iced_graphics::Rectangle,
        content: &str,
        size: u16,
        font: Option<Self::Font>,
        color: Option<iced_graphics::Color>,
        horizontal_alignment: iced_graphics::HorizontalAlignment,
        vertical_alignment: iced_graphics::VerticalAlignment,
    ) -> Self::Output {
        let x = match horizontal_alignment {
            iced_native::HorizontalAlignment::Left => bounds.x,
            iced_native::HorizontalAlignment::Center => bounds.center_x(),
            iced_native::HorizontalAlignment::Right => bounds.x + bounds.width,
        };

        let y = match vertical_alignment {
            iced_native::VerticalAlignment::Top => bounds.y,
            iced_native::VerticalAlignment::Center => bounds.center_y(),
            iced_native::VerticalAlignment::Bottom => bounds.y + bounds.height,
        };

        (
            Primitive::Text {
                content: content.to_string(),
                size: f32::from(size),
                bounds: Rectangle { x, y, ..bounds },
                color: color.unwrap_or(defaults.text.color),
                font: font.unwrap_or_else(|| self.default_font()),
                horizontal_alignment,
                vertical_alignment,
            },
            mouse::Interaction::default(),
        )
    }
}
