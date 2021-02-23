//! Use a floating button to overlay a button over some content
//!
//! *This API requires the following crate features to be activated: floating_button*
use dodrio::bumpalo;
use iced_web::{button, Bus, Button, Css, Element, Widget};

pub use crate::style::button::*;

pub mod anchor;
pub use anchor::Anchor;

pub mod offset;
pub use offset::Offset;

/// A floating button floating over some content.
///
/// # Example
/// ```
/// # use iced_aw::FloatingButton;
/// # use iced_web::{Button, Column, Text, button};
/// #[derive(Debug, Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// let mut button_state = button::State::default();
///
/// let content = Column::new();
/// let floating_button = FloatingButton::new(
///     &mut button_state,
///     content,
///     |state| Button::new(state, Text::new("Press Me!"))
///         .on_press(Message::ButtonPressed)
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct FloatingButton<'a, Message>
where
    Message: Clone,
{
    anchor: Anchor,
    offset: Offset,
    hidden: bool,
    underlay: Element<'a, Message>,
    button: Button<'a, Message>,
}

impl<'a, Message> FloatingButton<'a, Message>
where
    Message: Clone,
{
    /// Creates a new [`FloatingButton`](FloatingButton) over some content,
    /// showing the given [`Button`](iced_native::button::Button).
    pub fn new<U, B>(state: &'a mut button::State, underlay: U, button: B) -> Self
    where
        U: Into<Element<'a, Message>>,
        B: Fn(&mut button::State) -> Button<'_, Message>,
    {
        FloatingButton {
            anchor: Anchor::SouthEast,
            offset: 5.0.into(),
            hidden: false,
            underlay: underlay.into(),
            button: button(state),
        }
    }

    /// Sets the [`Anchor`](Anchor) of the [`FloatingButton`](FloatingButton).
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets the [`Offset`](Offset) of the [`FloatingButton`](FloatingButton).
    pub fn offset<O>(mut self, offset: O) -> Self
    where
        O: Into<Offset>,
    {
        self.offset = offset.into();
        self
    }

    /// Hide or unhide the [`Button`](iced_native::button::Button) on the
    /// [`FloatingButton`](FloatingButton).
    pub fn hide(mut self, hide: bool) -> Self {
        self.hidden = hide;
        self
    }
}

impl<'a, Message> Widget<Message> for FloatingButton<'a, Message>
where
    Message: 'static + Clone,
{
    fn node<'b>(
        &self,
        bump: &'b bumpalo::Bump,
        bus: &Bus<Message>,
        style_sheet: &mut Css<'b>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::*;

        let position = match self.anchor {
            Anchor::NorthWest => format!("top: {}px; left: {}px;", self.offset.y, self.offset.x),
            Anchor::NorthEast => format!("top: {}px; right: {}px;", self.offset.y, self.offset.x),
            Anchor::SouthWest => format!("bottom: {}px; left: {}px;", self.offset.y, self.offset.x),
            Anchor::SouthEast => {
                format!("bottom: {}px; right: {}px;", self.offset.y, self.offset.x)
            }
        };

        let node = div(bump)
            .attr("style", "position: relative; width: 100%; height: 100%;")
            .children(vec![
                self.underlay.node(bump, bus, style_sheet),
                if self.hidden {
                    div(bump).finish()
                } else {
                    div(bump)
                        .attr(
                            "style",
                            bumpalo::format!(
                                in bump,
                                "position: absolute; {}",
                                position
                            )
                            .into_bump_str(),
                        )
                        .children(vec![self.button.node(bump, bus, style_sheet)])
                        .finish()
                },
            ]);

        node.finish()
    }
}

impl<'a, Message> From<FloatingButton<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(floating_button: FloatingButton<'a, Message>) -> Element<'a, Message> {
        Element::new(floating_button)
    }
}
