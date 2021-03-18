//! A menu bar.
//!
//! *This API requires the following crate features to be activated: `menu`*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color, Vector};
#[cfg(target_arch = "wasm32")]
use iced_web::{Background, Color, Vector};

/// The appearance of a [`Menu`](crate::native::Menu).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The shadow offset of the [`Menu`](crate::native::Menu).
    pub shadow_offset: Vector,
    /// The background of the [`Menu`](crate::native::Menu).
    pub background: Background,
    /// The border radius of the [`Menu`](crate::native::Menu).
    pub border_radius: f32,
    /// The border width of the [`Menu`](crate::native::Menu).
    pub border_width: f32,
    /// The border color of the [`Menu`](crate::native::Menu).
    pub border_color: Color,
    /// The background of the label of the [`Section`](crate::native::menu::Section)s.
    pub label_background: Option<Background>,
    /// The text color of the [`Menu`](crate::native::Menu).
    pub text_color: Color,

    /// The shadow offset of the [`MenuOverlay`](crate::native::overlay::MenuOverlay).
    pub overlay_shadow_offset: Vector,
    /// The background  of the [`MenuOverlay`](crate::native::overlay::MenuOverlay).
    pub overlay_background: Background,
    /// The border radius  of the [`MenuOverlay`](crate::native::overlay::MenuOverlay).
    pub overlay_border_radius: f32,
    /// The border width  of the [`MenuOverlay`](crate::native::overlay::MenuOverlay).
    pub overlay_border_width: f32,
    /// The border color  of the [`MenuOverlay`](crate::native::overlay::MenuOverlay).
    pub overlay_border_color: Color,
    /// The background of the label of the [entries](crate::native::menu::Entry).
    pub overlay_label_background: Option<Background>,
    /// The text color  of the [`MenuOverlay`](crate::native::overlay::MenuOverlay).
    pub overlay_text_color: Color,

    /// The corner radius of the separator.
    pub separator_radius: f32,
    /// The width of the separator.
    pub separator_width: f32,
    /// The color of the separator.
    pub separator_color: Color,
    /// The horizontal marging of the separator.
    pub separator_horizontal_margin: f32,
}

/// The appearance of a [`Menu`](crate::native::Menu).
pub trait StyleSheet {
    /// The normal appearance of a [`Menu`](crate::native::Menu).
    fn active(&self) -> Style;

    /// The appearance when something is selected of the
    /// [`Menu`](crate::native::Menu) (Currently unused).
    fn selected(&self) -> Style;

    /// The appearance when something is hovered of the
    /// [`Menu`](crate::native::Menu).
    fn hovered(&self) -> Style;

    /// The appearance when something is focused of the
    /// [`Menu`](crate::native::Menu).
    fn focused(&self) -> Style;

    /// The appearance when something is disabled of the
    /// [`Menu`](crate::native::Menu).
    fn disabled(&self) -> Style;
}

/// The default appearance of the [`Menu`](crate::native::Menu).
#[derive(Clone, Copy, Debug)]
pub struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            shadow_offset: Vector::new(0.0, 1.0),
            background: Background::Color([0.87, 0.87, 0.87].into()),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            label_background: None,
            text_color: Color::BLACK,

            overlay_shadow_offset: Vector::new(0.0, 1.0),
            overlay_background: Background::Color([0.87, 0.87, 0.87].into()),
            overlay_border_radius: 0.0,
            overlay_border_width: 1.0,
            overlay_border_color: [0.0, 0.0, 0.0, 0.5].into(),
            overlay_label_background: None,
            overlay_text_color: Color::BLACK,

            separator_radius: 5.0,
            separator_width: 1.0,
            separator_color: [0.7, 0.7, 0.7].into(),
            separator_horizontal_margin: 1.0,
        }
    }

    fn selected(&self) -> Style {
        Style { ..self.active() }
    }

    fn hovered(&self) -> Style {
        let active = self.active();

        Style {
            label_background: Some(Background::Color([0.9, 0.9, 0.9].into())),
            overlay_label_background: Some(Background::Color([0.9, 0.9, 0.9].into())),

            ..active
        }
    }

    fn focused(&self) -> Style {
        Style { ..self.active() }
    }

    fn disabled(&self) -> Style {
        Style {
            text_color: [0.0, 0.0, 0.0, 0.75].into(),
            overlay_text_color: [0.0, 0.0, 0.0, 0.75].into(),
            ..self.active()
        }
    }
}

#[allow(clippy::use_self)]
impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

#[allow(clippy::use_self)]
impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
