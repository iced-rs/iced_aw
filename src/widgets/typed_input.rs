//! Display fields that can only be filled with a specific type.
//!
//! *This API requires the following crate features to be activated: `typed_input`*

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::widget::{
    tree::{State, Tag},
    Operation, Tree, Widget,
};
use iced::advanced::{Clipboard, Shell};
use iced::mouse::{self, Cursor};
use iced::{
    event,
    widget::text_input::{self, TextInput},
    Event, Size,
};
use iced::{Element, Length, Padding, Pixels, Rectangle};

use std::{fmt::Display, str::FromStr};

/// The default padding
const DEFAULT_PADDING: f32 = 5.0;

/// A field that can only be filled with a specific type.
///
/// # Example
/// ```ignore
/// # use iced_aw::TypedInput;
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
///     TypedInputChanged(u32),
/// }
///
/// let value = 12;
///
/// let input = TypedInput::new(
///     value,
///     Message::TypedInputChanged,
/// );
/// ```
pub struct TypedInput<'a, T, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::text::Renderer,
    Theme: text_input::Catalog,
{
    /// The current value of the [`TypedInput`].
    value: T,
    /// The underlying element of the [`TypedInput`].
    text_input: text_input::TextInput<'a, InternalMessage, Theme, Renderer>,
    text: String,
    /// The ``on_change`` event of the [`TypedInput`].
    on_change: Option<Box<dyn 'a + Fn(T) -> Message>>,
    /// The ``on_submit`` event of the [`TypedInput`].
    #[allow(clippy::type_complexity)]
    on_submit: Option<Box<dyn 'a + Fn(Result<T, String>) -> Message>>,
    /// The ``on_paste`` event of the [`TypedInput`]
    on_paste: Option<Box<dyn 'a + Fn(T) -> Message>>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
enum InternalMessage {
    OnChange(String),
    OnSubmit,
    OnPaste(String),
}

impl<'a, T, Message, Theme, Renderer> TypedInput<'a, T, Message, Theme, Renderer>
where
    T: Display + FromStr,
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
    Theme: text_input::Catalog,
{
    /// Creates a new [`TypedInput`].
    ///
    /// It expects:
    /// - the current value
    /// - a function that produces a message when the [`TypedInput`] changes
    #[must_use]
    pub fn new(placeholder: &str, value: &T) -> Self
    where
        T: 'a + Clone,
    {
        let padding = DEFAULT_PADDING;

        Self {
            value: value.clone(),
            text_input: text_input::TextInput::new(placeholder, format!("{value}").as_str())
                .padding(padding)
                .width(Length::Fixed(127.0))
                .class(<Theme as text_input::Catalog>::default()),
            text: value.to_string(),
            on_change: None,
            on_submit: None,
            on_paste: None,
        }
    }

    /// Sets the [Id](text_input::Id) of the internal [`TextInput`]
    #[must_use]
    pub fn id(mut self, id: text_input::Id) -> Self {
        self.text_input = self.text_input.id(id);
        self
    }

    /// Convert the [`TypedInput`] into a secure password input
    #[must_use]
    pub fn secure(mut self, is_secure: bool) -> Self {
        self.text_input = self.text_input.secure(is_secure);
        self
    }

    /// Sets the message that should be produced when some valid text is typed into [`TypedInput`]
    ///
    /// If neither this method nor [`on_submit`](Self::on_submit) is called, the [`TypedInput`] will be disabled
    #[must_use]
    pub fn on_input<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(T) -> Message,
    {
        self.text_input = self.text_input.on_input(InternalMessage::OnChange);
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Sets the message that should be produced when the [`TextInput`] is
    /// focused and the enter key is pressed.
    ///
    /// If neither this method nor [`on_input`](Self::on_input) is called, the [`TypedInput`] will be disabled
    #[must_use]
    pub fn on_submit<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(Result<T, String>) -> Message,
    {
        self.text_input = self
            .text_input
            .on_input(InternalMessage::OnChange)
            .on_submit(InternalMessage::OnSubmit);
        self.on_submit = Some(Box::new(callback));
        self
    }

    /// Sets the message that should be produced when some text is pasted into the [`TypedInput`], resulting in a valid value
    #[must_use]
    pub fn on_paste<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(T) -> Message,
    {
        self.text_input = self.text_input.on_paste(InternalMessage::OnPaste);
        self.on_paste = Some(Box::new(callback));
        self
    }

    /// Sets the [Font](iced::advanced::text::Renderer::Font) of the [`TypedInput`].
    #[must_use]
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.text_input = self.text_input.font(font);
        self
    }

    /// Sets the [Icon](iced::widget::text_input::Icon) of the [`TypedInput`]
    #[must_use]
    pub fn icon(mut self, icon: iced::widget::text_input::Icon<Renderer::Font>) -> Self {
        self.text_input = self.text_input.icon(icon);
        self
    }

    /// Sets the width of the [`TypedInput`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.text_input = self.text_input.width(width);
        self
    }

    /// Sets the padding of the [`TypedInput`].
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.text_input = self.text_input.padding(padding);
        self
    }

    /// Sets the text size of the [`TypedInput`].
    #[must_use]
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_input = self.text_input.size(size);
        self
    }

    /// Sets the [`text::LineHeight`](iced::widget::text::LineHeight) of the [`TypedInput`].
    #[must_use]
    pub fn line_height(mut self, line_height: impl Into<iced::widget::text::LineHeight>) -> Self {
        self.text_input = self.text_input.line_height(line_height);
        self
    }

    /// Sets the style of the input of the [`TypedInput`].
    #[must_use]
    pub fn style(
        mut self,
        style: impl Fn(&Theme, text_input::Status) -> text_input::Style + 'a,
    ) -> Self
    where
        <Theme as text_input::Catalog>::Class<'a>: From<text_input::StyleFn<'a, Theme>>,
    {
        self.text_input = self.text_input.style(style);
        self
    }

    /// Sets the class of the input of the [`TypedInput`].
    #[must_use]
    pub fn class(mut self, class: impl Into<<Theme as text_input::Catalog>::Class<'a>>) -> Self {
        self.text_input = self.text_input.class(class);
        self
    }

    /// Gets the current text of the [`TypedInput`].
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl<'a, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for TypedInput<'a, T, Message, Theme, Renderer>
where
    T: Display + FromStr + Clone + PartialEq,
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer,
    Theme: text_input::Catalog,
{
    fn tag(&self) -> Tag {
        <TextInput<_, _, _> as Widget<_, _, _>>::tag(&self.text_input)
    }
    fn state(&self) -> State {
        <TextInput<_, _, _> as Widget<_, _, _>>::state(&self.text_input)
    }

    fn children(&self) -> Vec<Tree> {
        <TextInput<_, _, _> as Widget<_, _, _>>::children(&self.text_input)
    }

    fn diff(&self, state: &mut Tree) {
        <TextInput<_, _, _> as Widget<_, _, _>>::diff(&self.text_input, state);
    }

    fn size(&self) -> Size<Length> {
        <TextInput<_, _, _> as Widget<_, _, _>>::size(&self.text_input)
    }

    fn layout(&self, state: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        <TextInput<_, _, _> as Widget<_, _, _>>::layout(&self.text_input, state, renderer, limits)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        <TextInput<_, _, _> as Widget<_, _, _>>::draw(
            &self.text_input,
            state,
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        <TextInput<_, _, _> as Widget<_, _, _>>::mouse_interaction(
            &self.text_input,
            state,
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        <TextInput<_, _, _> as Widget<_, _, _>>::operate(
            &self.text_input,
            state,
            layout,
            renderer,
            operation,
        );
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let mut messages = Vec::new();
        let mut sub_shell = Shell::new(&mut messages);
        let status = self.text_input.on_event(
            state,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            &mut sub_shell,
            viewport,
        );

        if let Some(redraw) = sub_shell.redraw_request() {
            shell.request_redraw(redraw);
        }
        if sub_shell.is_layout_invalid() {
            shell.invalidate_layout();
        }
        if sub_shell.are_widgets_invalid() {
            shell.invalidate_widgets();
        }

        for message in messages {
            match message {
                InternalMessage::OnChange(value) => {
                    self.text = value;
                    if let Ok(val) = T::from_str(&self.text) {
                        if self.value != val {
                            self.value = val.clone();
                            if let Some(on_change) = &self.on_change {
                                shell.publish(on_change(val));
                            }
                        }
                    }
                    shell.invalidate_layout();
                }
                InternalMessage::OnSubmit => {
                    if let Some(on_submit) = &self.on_submit {
                        let value = match T::from_str(&self.text) {
                            Ok(v) => Ok(v),
                            Err(_) => Err(self.text.clone()),
                        };
                        shell.publish(on_submit(value));
                    }
                }
                InternalMessage::OnPaste(value) => {
                    self.text = value;
                    if let Ok(val) = T::from_str(&self.text) {
                        if self.value != val {
                            self.value = val.clone();
                            if let Some(on_paste) = &self.on_paste {
                                shell.publish(on_paste(val));
                            }
                        }
                    }
                    shell.invalidate_layout();
                }
            }
        }
        status
    }
}

impl<'a, T, Message, Theme, Renderer> From<TypedInput<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: 'a + Display + FromStr + Clone + PartialEq,
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer,
    Theme: 'a + text_input::Catalog,
{
    fn from(typed_input: TypedInput<'a, T, Message, Theme, Renderer>) -> Self {
        Element::new(typed_input)
    }
}
