//! A modal for showing elements as an overlay on top of another.
//! 
//! *This API requires the following crate features to be activated: modal*
use iced_web::{css, Background, Bus, Css, Element, Widget};
use dodrio::bumpalo;

use crate::style::modal::{Style, StyleSheet};

/// A modal content as an overlay.
/// 
/// Can be used in combination with the [`Card`](crate::native::card::Card)
/// widget to form dialog elements.
/// 
/// TODO: Example
#[allow(missing_debug_implementations)]
pub struct Modal<'a, Message>
where
    Message: Clone,
{
    show: bool,
    underlay: Element<'a, Message>,
    content: Element<'a, Message>,
    backdrop: Option<Message>,
    esc: Option<Message>,
    style: Box<dyn StyleSheet>,
}

impl<'a, Message> Modal<'a, Message>
where
    Message: Clone,
{
    /// Creates a new [`Modal`](Modal) wrapping the underlying element to
    /// show some content as an overlay.
    /// 
    /// `state` is the content's state, assigned at the creation of the
    /// overlying content.
    pub fn new<S, U, Content>(
        state: &'a mut State<S>,
        underlay: U,
        content: Content,
    ) -> Self
    where
        S: 'a,
        U: Into<Element<'a, Message>>,
        Content: Fn(&mut S) -> Element<'_, Message>,
    {
        let State {
            show,
            state
        } = state;

        Modal {
            show: *show,
            underlay: underlay.into(),
            content: content(state),
            backdrop: None,
            esc: None,
            style: Default::default(),
        }
    }

    /// Sets the message that will be produced when the backdrop of the
    /// [`Modal`](Modal) is clicked.
    pub fn backdrop(mut self, message: Message) -> Self {
        self.backdrop = Some(message);
        self
    }

    /// Sets the message that will be produced when the Escape Key is
    /// pressed when the modal is open.
    /// 
    /// This can be used to close the modal on ESC.
    pub fn on_esc(mut self, message: Message) -> Self {
        self.esc = Some(message);
        self
    }

    /// Sets the style of the [`Modal`](Modal).
    pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style = style.into();
        self
    }
}

/// The state of the modal.
#[derive(Debug)]
pub struct State<S> {
    show: bool,
    state: S,
}

impl <S> State<S> {
    /// Creates a new [`State`](State) containing the given state data.
    pub fn new(s: S) -> Self {
        State {
            show: false,
            state: s,
        }
    }

    /// Setting this to true shows the modal (the modal is open), false means
    /// the modal is hidden (closed).
    pub fn show(&mut self, b: bool) {
        self.show = b;
    }
}

impl<'a, Message> Widget<Message>
    for Modal<'a, Message>
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

        let style = self.style.active();

        let underlay = self.underlay.node(bump, bus, style_sheet);

        let modal = if self.show {
            let event_bus = bus.clone();

            let mut backdrop = div(bump)
                .attr(
                    "style",
                    bumpalo::format!(
                        in bump,
                        "position: absolute; background: {}; width: 100%; height: 100%;",
                        match style.background {
                            Background::Color(color) => css::color(color),
                        }
                    ).into_bump_str(),
                );

            if let Some(on_backdrop) = self.backdrop.clone() {
                backdrop = backdrop.on("click", move |_root, _vdom, _event| {
                    event_bus.publish(on_backdrop.clone());
                });
            }

            let backdrop = backdrop.finish();

            let modal_content = div(bump)
                .attr(
                    "style",
                    "margin: auto; position: absolute; top: 50%;\
                    transform: translate(-50%, -50%); left: 50%;"
                )
                .children(vec![self.content.node(bump, bus, style_sheet)])
                .finish();

            Some(div(bump)
                .attr(
                    "style",
                    "position: absolute; top: 0; bottom: 0; left: 0; right: 0;"
                ).children(
                    vec![ backdrop,  modal_content ]
                )
                .finish()
            )
        } else {
            None
        };

        div(bump)
            .attr(
                "style",
                "position: relative;"
            )
            .children(
                match modal {
                    Some(modal) => vec![underlay, modal],
                    None => vec![underlay],
                }
            )
            .finish()
    }
}

impl<'a, Message> From<Modal<'a, Message>>
    for Element<'a, Message>
where 
    Message: 'static + Clone,
{
    fn from(modal: Modal<'a, Message>) -> Element<'a, Message> {
        Element::new(modal)
    }
}