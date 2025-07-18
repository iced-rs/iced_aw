//! Display fields that can only be filled with numeric type.
//!
//! A [`NumberInput`] has some local [`State`].
use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer,
        widget::{
            tree::{State, Tag},
            Operation, Tree,
        },
        Clipboard, Layout, Shell, Widget,
    },
    alignment::{Horizontal, Vertical},
    keyboard,
    mouse::{self, Cursor},
    widget::{
        text::{LineHeight, Wrapping},
        text_input::{self, cursor, Value},
        Column, Container, Row, Text,
    },
    window::RedrawRequest,
    Alignment, Background, Border, Color, Element, Event, Length, Padding, Point, Rectangle, Size,
};
use num_traits::{bounds::Bounded, Num, NumAssignOps};
use std::{
    fmt::Display,
    ops::{Bound, RangeBounds},
    str::FromStr,
};

use crate::style::{self, Status};
pub use crate::style::{
    number_input::{self, Catalog, Style},
    StyleFn,
};
use crate::widget::typed_input::TypedInput;
use iced_fonts::{
    required::{icon_to_string, RequiredIcons},
    REQUIRED_FONT,
};

/// The default padding
const DEFAULT_PADDING: Padding = Padding::new(5.0);

/// A field that can only be filled with numeric type.
///
/// # Example
/// ```ignore
/// # use iced_aw::NumberInput;
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
///     NumberInputChanged(u32),
/// }
///
/// let value = 12;
/// let max = 1275;
///
/// let input = NumberInput::new(
///     value,
///     0..=max,
///     Message::NumberInputChanged,
/// )
/// .step(2);
/// ```
#[allow(missing_debug_implementations)]
pub struct NumberInput<'a, T, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: number_input::ExtendedCatalog,
{
    /// The current value of the [`NumberInput`].
    value: T,
    /// The step for each modify of the [`NumberInput`].
    step: T,
    /// The min value of the [`NumberInput`].
    min: Bound<T>,
    /// The max value of the [`NumberInput`].
    max: Bound<T>,
    /// The content padding of the [`NumberInput`].
    padding: iced::Padding,
    /// The text size of the [`NumberInput`].
    size: Option<iced::Pixels>,
    /// The underlying element of the [`NumberInput`].
    content: TypedInput<'a, T, InternalMessage<T>, Theme, Renderer>,
    /// The ``on_change`` event of the [`NumberInput`].
    on_change: Option<Box<dyn 'a + Fn(T) -> Message>>,
    /// The ``on_submit`` event of the [`NumberInput`].
    #[allow(clippy::type_complexity)]
    on_submit: Option<Message>,
    /// The ``on_paste`` event of the [`NumberInput`]
    on_paste: Option<Box<dyn 'a + Fn(T) -> Message>>,
    /// The style of the [`NumberInput`].
    class: <Theme as style::number_input::Catalog>::Class<'a>,
    /// The font text of the [`NumberInput`].
    font: Renderer::Font,
    // /// The Width to use for the ``NumberBox`` Default is ``Length::Fill``
    // width: Length,
    /// Ignore mouse scroll events for the [`NumberInput`] Default is ``false``.
    ignore_scroll_events: bool,
    /// Ignore drawing increase and decrease buttons [`NumberInput`] Default is ``false``.
    ignore_buttons: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
enum InternalMessage<T> {
    OnChange(T),
    OnSubmit(Result<T, String>),
    OnPaste(T),
}

impl<'a, T, Message, Theme, Renderer> NumberInput<'a, T, Message, Theme, Renderer>
where
    T: Num + NumAssignOps + PartialOrd + Display + FromStr + Clone + Bounded + 'a,
    Message: Clone + 'a,
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: number_input::ExtendedCatalog,
{
    /// Creates a new [`NumberInput`].
    ///
    /// It expects:
    /// - the current value
    /// - the bound values
    /// - a function that produces a message when the [`NumberInput`] changes
    pub fn new<F>(value: &T, bounds: impl RangeBounds<T>, on_change: F) -> Self
    where
        F: 'static + Fn(T) -> Message + Clone,
        T: 'static,
    {
        let padding = DEFAULT_PADDING;

        Self {
            value: value.clone(),
            step: T::one(),
            min: bounds.start_bound().cloned(),
            max: bounds.end_bound().cloned(),
            padding,
            size: None,
            content: TypedInput::new("", value)
                .on_input(InternalMessage::OnChange)
                .padding(padding)
                .width(Length::Fixed(127.0))
                .class(Theme::default_input()),
            on_change: Some(Box::new(on_change)),
            on_submit: None,
            on_paste: None,
            class: <Theme as style::number_input::Catalog>::default(),
            font: Renderer::Font::default(),
            // width: Length::Shrink,
            ignore_scroll_events: false,
            ignore_buttons: false,
        }
    }

    /// Sets the [`Id`](text_input::Id) of the underlying [`TextInput`](iced::widget::TextInput).
    #[must_use]
    pub fn id(mut self, id: impl Into<text_input::Id>) -> Self {
        self.content = self.content.id(id.into());
        self
    }

    /// Sets the message that should be produced when some valid text is typed into [`NumberInput`]
    ///
    /// If neither this method nor [`on_submit`](Self::on_submit) is called, the [`NumberInput`] will be disabled
    #[must_use]
    pub fn on_input<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(T) -> Message,
    {
        self.content = self.content.on_input(InternalMessage::OnChange);
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Sets the message that should be produced when some text is typed into the [`NumberInput`], if `Some`.
    ///
    /// If this is `None`, and there is no [`on_submit`](Self::on_submit) callback, the [`NumberInput`] will be disabled.
    #[must_use]
    pub fn on_input_maybe<F>(mut self, callback: Option<F>) -> Self
    where
        F: 'a + Fn(T) -> Message,
    {
        if let Some(callback) = callback {
            self.content = self.content.on_input(InternalMessage::OnChange);
            self.on_change = Some(Box::new(callback));
        } else {
            if self.on_submit.is_none() {
                // Used to give a proper type to None, maybe someone can find a better way
                #[allow(unused_assignments)]
                let mut f = Some(InternalMessage::OnChange);
                f = None;
                self.content = self.content.on_input_maybe(f);
            }
            self.on_change = None;
        }
        self
    }

    /// Sets the message that should be produced when the [`NumberInput`] is
    /// focused and the enter key is pressed.
    #[must_use]
    pub fn on_submit(mut self, message: Message) -> Self {
        self.content = self.content.on_submit(InternalMessage::OnSubmit);
        self.on_submit = Some(message);
        self
    }

    /// Sets the message that should be produced when the [`NumbertInput`] is
    /// focused and the enter key is pressed, if `Some`.
    ///
    /// If this is `None`, and there is no [`on_change`](Self::on_input) callback, the [`NumberInput`] will be disabled.
    #[must_use]
    pub fn on_submit_maybe(mut self, message: Option<Message>) -> Self {
        if let Some(message) = message {
            self.content = self.content.on_submit(InternalMessage::OnSubmit);
            self.on_submit = Some(message);
        } else {
            if self.on_change.is_none() {
                // Used to give a proper type to None, maybe someone can find a better way
                #[allow(unused_assignments)]
                let mut f = Some(InternalMessage::OnChange);
                f = None;
                self.content = self.content.on_input_maybe(f);
            }
            // Used to give a proper type to None, maybe someone can find a better way
            #[allow(unused_assignments)]
            let mut f = Some(InternalMessage::OnSubmit);
            f = None;
            self.content = self.content.on_submit_maybe(f);
            self.on_change = None;
        }
        self
    }

    /// Sets the message that should be produced when some text is pasted into the [`NumberInput`], resulting in a valid value
    #[must_use]
    pub fn on_paste<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(T) -> Message,
    {
        self.content = self.content.on_paste(InternalMessage::OnPaste);
        self.on_paste = Some(Box::new(callback));
        self
    }

    /// Sets the message that should be produced when some text is pasted into the [`NumberInput`], resulting in a valid value, if `Some`
    #[must_use]
    pub fn on_paste_maybe<F>(mut self, callback: Option<F>) -> Self
    where
        F: 'a + Fn(T) -> Message,
    {
        if let Some(callback) = callback {
            self.content = self.content.on_paste(InternalMessage::OnPaste);
            self.on_paste = Some(Box::new(callback));
        } else {
            // Used to give a proper type to None, maybe someone can find a better way
            #[allow(unused_assignments)]
            let mut f = Some(InternalMessage::OnPaste);
            f = None;
            self.content = self.content.on_paste_maybe(f);
            self.on_paste = None;
        }
        self
    }

    /// Sets the [`Font`] of the [`Text`].
    ///
    /// [`Font`]: iced::Font
    /// [`Text`]: iced::widget::Text
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self.content = self.content.font(font);
        self
    }

    /// Sets the [Icon](iced::widget::text_input::Icon) of the [`NumberInput`]
    #[must_use]
    pub fn icon(mut self, icon: iced::widget::text_input::Icon<Renderer::Font>) -> Self {
        self.content = self.content.icon(icon);
        self
    }

    /// Sets the width of the [`NumberInput`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.content = self.content.width(width);
        self
    }

    /// Sets the width of the [`NumberInput`].
    #[deprecated(since = "0.11.1", note = "use `width` instead")]
    #[must_use]
    pub fn content_width(self, width: impl Into<Length>) -> Self {
        self.width(width)
    }

    /// Sets the padding of the [`NumberInput`].
    #[must_use]
    pub fn padding(mut self, padding: impl Into<iced::Padding>) -> Self {
        let padding = padding.into();
        self.padding = padding;
        self.content = self.content.padding(padding);
        self
    }

    /// Sets the text size of the [`NumberInput`].
    #[must_use]
    pub fn size(mut self, size: impl Into<iced::Pixels>) -> Self {
        let size = size.into();
        self.size = Some(size);
        self.content = self.content.size(size);
        self
    }

    /// Sets the [`text::LineHeight`](iced::widget::text::LineHeight) of the [`NumberInput`].
    #[must_use]
    pub fn line_height(mut self, line_height: impl Into<iced::widget::text::LineHeight>) -> Self {
        self.content = self.content.line_height(line_height);
        self
    }

    /// Sets the horizontal alignment of the [`NumberInput`].
    #[must_use]
    pub fn align_x(mut self, alignment: impl Into<iced::alignment::Horizontal>) -> Self {
        self.content = self.content.align_x(alignment);
        self
    }

    /// Sets the style of the [`NumberInput`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as style::number_input::Catalog>::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }
    /// Sets the style of the input of the [`NumberInput`].
    #[must_use]
    pub fn input_style(
        mut self,
        style: impl Fn(&Theme, text_input::Status) -> text_input::Style + 'a,
    ) -> Self
    where
        <Theme as text_input::Catalog>::Class<'a>: From<text_input::StyleFn<'a, Theme>>,
    {
        self.content = self.content.style(style);
        self
    }

    /// Sets the class of the input of the [`NumberInput`].
    #[must_use]
    pub fn class(
        mut self,
        class: impl Into<<Theme as style::number_input::Catalog>::Class<'a>>,
    ) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the minimum & maximum value (bound) of the [`NumberInput`].
    /// # Example
    /// ```
    /// use iced_aw::widget::number_input;
    /// // Creates a range from -5 till 5.
    /// let input: iced_aw::NumberInput<'_, _, _, iced::Theme, iced::Renderer> = number_input(&4 /* my_value */, 0..=4, |_| () /* my_message */).bounds(-5..=5);
    /// ```
    #[must_use]
    pub fn bounds(mut self, bounds: impl RangeBounds<T>) -> Self {
        self.min = bounds.start_bound().cloned();
        self.max = bounds.end_bound().cloned();

        self
    }

    /// Sets the step of the [`NumberInput`].
    #[must_use]
    pub fn step(mut self, step: T) -> Self {
        self.step = step;
        self
    }

    /// Enable or disable increase and decrease buttons of the [`NumberInput`], by default this is set to
    /// ``false``.
    #[must_use]
    pub fn ignore_buttons(mut self, ignore: bool) -> Self {
        self.ignore_buttons = ignore;
        self
    }

    /// Enable or disable mouse scrolling events of the [`NumberInput`], by default this is set to
    /// ``false``.
    #[must_use]
    pub fn ignore_scroll(mut self, ignore: bool) -> Self {
        self.ignore_scroll_events = ignore;
        self
    }

    /// Decrease current value by step of the [`NumberInput`].
    fn decrease_value(&mut self, shell: &mut Shell<Message>) {
        if self.value.clone() > self.min() + self.step.clone()
            && self.valid(&(self.value.clone() - self.step.clone()))
        {
            self.value -= self.step.clone();
        } else if self.value > self.min() {
            self.value = self.min();
        } else {
            return;
        }
        if let Some(on_change) = &self.on_change {
            shell.publish(on_change(self.value.clone()));
        }
    }

    /// Increase current value by step of the [`NumberInput`].
    fn increase_value(&mut self, shell: &mut Shell<Message>) {
        if self.value < self.max() - self.step.clone()
            && self.valid(&(self.value.clone() + self.step.clone()))
        {
            self.value += self.step.clone();
        } else if self.value < self.max() {
            self.value = self.max();
        } else {
            return;
        }
        if let Some(on_change) = &self.on_change {
            shell.publish(on_change(self.value.clone()));
        }
    }

    /// Returns the lower value possible
    /// if the bound is excluded the bound is increased by the step
    fn min(&self) -> T {
        match &self.min {
            Bound::Included(n) => n.clone(),
            Bound::Excluded(n) => n.clone() + self.step.clone(),
            Bound::Unbounded => T::min_value(),
        }
    }

    /// Returns the higher value possible
    /// if the bound is excluded the bound is decreased by the step
    fn max(&self) -> T {
        match &self.max {
            Bound::Included(n) => n.clone(),
            Bound::Excluded(n) => n.clone() - self.step.clone(),
            Bound::Unbounded => T::max_value(),
        }
    }

    /// Checks if the value is within the bounds
    fn valid(&self, value: &T) -> bool {
        (match &self.min {
            Bound::Included(n) if *n > *value => false,
            Bound::Excluded(n) if *n >= *value => false,
            _ => true,
        }) && (match &self.max {
            Bound::Included(n) if *n < *value => false,
            Bound::Excluded(n) if *n <= *value => false,
            _ => true,
        })
    }

    /// Checks if the value can be increased by the step
    fn can_increase(&self) -> bool {
        (self.value < self.max() - self.step.clone()
            && self.valid(&(self.value.clone() + self.step.clone())))
            || self.value < self.max()
    }

    /// Checks if the value can be decreased by the step
    fn can_decrease(&self) -> bool {
        (self.value.clone() > self.min() + self.step.clone()
            && self.valid(&(self.value.clone() - self.step.clone())))
            || self.value > self.min()
    }

    /// Checks if the [`NumberInput`] is disabled
    /// Meaning that the bounds are too tight for the value to change
    fn disabled(&self) -> bool {
        match (&self.min, &self.max) {
            (Bound::Included(n) | Bound::Excluded(n), Bound::Included(m) | Bound::Excluded(m)) => {
                *n >= *m
            }
            _ => false,
        }
    }
}

impl<'a, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for NumberInput<'a, T, Message, Theme, Renderer>
where
    T: Num + NumAssignOps + PartialOrd + Display + FromStr + ToString + Clone + Bounded + 'a,
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: number_input::ExtendedCatalog,
{
    fn tag(&self) -> Tag {
        Tag::of::<ModifierState>()
    }
    fn state(&self) -> State {
        State::new(ModifierState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree {
            tag: self.content.tag(),
            state: self.content.state(),
            children: self.content.children(),
        }]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children_custom(
            &[&self.content],
            |state, content| content.diff(state),
            |&content| Tree {
                tag: content.tag(),
                state: content.state(),
                children: content.children(),
            },
        );
    }

    fn size(&self) -> Size<Length> {
        Widget::size(&self.content)
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let num_size = self.size();
        let limits = limits.width(num_size.width).height(Length::Shrink);
        let content = self
            .content
            .layout(&mut tree.children[0], renderer, &limits);
        let limits2 = Limits::new(Size::new(0.0, 0.0), content.size());
        let txt_size = self.size.unwrap_or_else(|| renderer.default_size());

        let icon_size = txt_size * 2.5 / 4.0;
        let btn_mod = |c| {
            Container::<Message, Theme, Renderer>::new(Text::new(format!(" {c} ")).size(icon_size))
                .center_y(Length::Shrink)
                .center_x(Length::Shrink)
        };

        let default_padding = DEFAULT_PADDING;

        let element = if self.padding.top < default_padding.top
            || self.padding.bottom < default_padding.bottom
            || self.padding.right < default_padding.right
        {
            Element::new(
                Row::<Message, Theme, Renderer>::new()
                    .spacing(1)
                    .width(Length::Shrink)
                    .push(btn_mod('+'))
                    .push(btn_mod('-')),
            )
        } else {
            Element::new(
                Column::<Message, Theme, Renderer>::new()
                    .spacing(1)
                    .width(Length::Shrink)
                    .push(btn_mod('▲'))
                    .push(btn_mod('▼')),
            )
        };

        let input_tree = if let Some(child_tree) = tree.children.get_mut(1) {
            child_tree.diff(element.as_widget());
            child_tree
        } else {
            let child_tree = Tree::new(element.as_widget());
            tree.children.insert(1, child_tree);
            &mut tree.children[1]
        };

        let mut modifier = element
            .as_widget()
            .layout(input_tree, renderer, &limits2.loose());
        let intrinsic = Size::new(
            content.size().width - 1.0,
            content.size().height.max(modifier.size().height),
        );
        modifier = modifier.align(Alignment::End, Alignment::Center, intrinsic);

        let size = limits.resolve(num_size.width, Length::Shrink, intrinsic);
        Node::with_children(size, vec![content, modifier])
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.operate(
                &mut tree.children[0],
                layout
                    .children()
                    .next()
                    .expect("NumberInput inner child Textbox was not created."),
                renderer,
                operation,
            );
        });
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn update(
        &mut self,
        state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();
        let content = children.next().expect("fail to get content layout");
        let mut mod_children = children
            .next()
            .expect("fail to get modifiers layout")
            .children();
        let inc_bounds = mod_children
            .next()
            .expect("fail to get increase mod layout")
            .bounds();
        let dec_bounds = mod_children
            .next()
            .expect("fail to get decrease mod layout")
            .bounds();

        if self.disabled() {
            return;
        }
        let can_decrease = self.can_decrease();
        let can_increase = self.can_increase();

        let cursor_position = cursor.position().unwrap_or_default();
        let mouse_over_widget = layout.bounds().contains(cursor_position);
        let mouse_over_inc = inc_bounds.contains(cursor_position);
        let mouse_over_dec = dec_bounds.contains(cursor_position);
        let mouse_over_button = mouse_over_inc || mouse_over_dec;

        let modifiers = state.state.downcast_mut::<ModifierState>();
        let mut value = self.content.text().to_owned();

        let child = state.children.get_mut(0).expect("fail to get child");
        let text_input = child
            .state
            .downcast_mut::<text_input::State<Renderer::Paragraph>>();

        // We use a secondary shell to select handle the event of the underlying [`TypedInput`]
        let mut messages = Vec::new();
        let mut sub_shell = Shell::new(&mut messages);

        // Function to forward the event to the underlying [`TypedInput`]
        let mut forward_to_text = |widget: &mut Self, child, clipboard| {
            widget.content.update(
                child,
                &event,
                content,
                cursor,
                renderer,
                clipboard,
                &mut sub_shell,
                viewport,
            )
        };

        // Check if the value that would result from the input is valid and within bound
        let supports_negative = self.min() < T::zero();
        let mut check_value = |value: &str| {
            if let Ok(value) = T::from_str(value) {
                self.valid(&value)
            } else if value.is_empty() || value == "-" && supports_negative {
                self.value = T::zero();
                true
            } else {
                false
            }
        };

        match event {
            Event::Keyboard(key) => {
                if !text_input.is_focused() {
                    return;
                }

                match key {
                    keyboard::Event::ModifiersChanged(_) => forward_to_text(self, child, clipboard),
                    keyboard::Event::KeyReleased { .. } => return,
                    keyboard::Event::KeyPressed {
                        key,
                        text,
                        modifiers,
                        ..
                    } => {
                        let cursor = text_input.cursor();

                        // If true, ignore Arrow/Home/End keys - they are coming from numpad and are just
                        // mislabeled. See the core PR:
                        // https://github.com/iced-rs/iced/pull/2278
                        let has_value = !modifiers.command()
                            && text
                                .as_ref()
                                .is_some_and(|t| t.chars().any(|c| !c.is_control()));

                        match key.as_ref() {
                            // Enter
                            keyboard::Key::Named(keyboard::key::Named::Enter) => {
                                forward_to_text(self, child, clipboard)
                            }
                            // Copy and selecting all
                            keyboard::Key::Character("c" | "a") if modifiers.command() => {
                                forward_to_text(self, child, clipboard)
                            }
                            // Cut
                            keyboard::Key::Character("x") if modifiers.command() => {
                                // We need a selection to cut
                                if let Some((start, end)) = cursor.selection(&Value::new(&value)) {
                                    let _ = value.drain(start..end);
                                    // We check that once this part is cut, it's still a number
                                    if check_value(&value) {
                                        forward_to_text(self, child, clipboard)
                                    } else {
                                        return;
                                    }
                                } else {
                                    return;
                                }
                            }
                            // Paste
                            keyboard::Key::Character("v") if modifiers.command() => {
                                // We need something to paste
                                let Some(paste) =
                                    clipboard.read(iced::advanced::clipboard::Kind::Standard)
                                else {
                                    return;
                                };
                                // We replace the selection or paste the text at the cursor
                                match cursor.state(&Value::new(&value)) {
                                    cursor::State::Index(idx) => {
                                        value.insert_str(idx, &paste);
                                    }
                                    cursor::State::Selection { start, end } => {
                                        value.replace_range(sorted_range(start, end), &paste);
                                    }
                                }

                                // We check if it's now a valid number
                                if check_value(&value) {
                                    forward_to_text(self, child, clipboard)
                                } else {
                                    return;
                                }
                            }
                            // Backspace
                            keyboard::Key::Named(keyboard::key::Named::Backspace) => {
                                // We remove either the selection or the character before the cursor
                                match cursor.state(&Value::new(&value)) {
                                    cursor::State::Selection { start, end } => {
                                        let _ = value.drain(sorted_range(start, end));
                                    }
                                    // We need the cursor not at the start
                                    cursor::State::Index(idx) if idx > 0 => {
                                        if modifiers.command() {
                                            // ctrl+backspace erases to the left,
                                            // including decimal separator but not including
                                            // minus sign.
                                            let _ =
                                                value.drain((value.starts_with('-').into())..idx);
                                        } else {
                                            let _ = value.remove(idx - 1);
                                        }
                                    }
                                    cursor::State::Index(_) => return,
                                }

                                // We check if it's now a valid number
                                if check_value(&value) {
                                    forward_to_text(self, child, clipboard)
                                } else {
                                    return;
                                }
                            }
                            // Delete
                            keyboard::Key::Named(keyboard::key::Named::Delete) => {
                                // We remove either the selection or the character after the cursor
                                match cursor.state(&Value::new(&value)) {
                                    cursor::State::Selection { start, end } => {
                                        let _ = value.drain(sorted_range(start, end));
                                    }
                                    // We need the cursor not at the end
                                    cursor::State::Index(idx) if idx < value.len() => {
                                        if idx == 0 && value.starts_with('-') {
                                            let _ = value.remove(0);
                                        } else if modifiers.command() {
                                            // ctrl+del erases to the right,
                                            // including decimal separator but not including
                                            // minus sign.
                                            let _ = value.drain(idx..);
                                        } else {
                                            let _ = value.remove(idx);
                                        }
                                    }
                                    cursor::State::Index(_) => return,
                                }

                                // We check if it's now a valid number
                                if check_value(&value) {
                                    forward_to_text(self, child, clipboard)
                                } else {
                                    return;
                                }
                            }
                            // Arrow Down, decrease by step
                            keyboard::Key::Named(keyboard::key::Named::ArrowDown)
                                if can_decrease && !has_value =>
                            {
                                self.decrease_value(shell);
                            }
                            // Arrow Up, increase by step
                            keyboard::Key::Named(keyboard::key::Named::ArrowUp)
                                if can_increase && !has_value =>
                            {
                                self.increase_value(shell);
                            }
                            // Movement of the cursor
                            keyboard::Key::Named(
                                keyboard::key::Named::ArrowLeft
                                | keyboard::key::Named::ArrowRight
                                | keyboard::key::Named::Home
                                | keyboard::key::Named::End,
                            ) if !has_value => forward_to_text(self, child, clipboard),
                            // Everything else
                            _ => match text {
                                // If we are trying to input text
                                Some(text) => {
                                    // We replace the selection or insert the text at the cursor
                                    match cursor.state(&Value::new(&value)) {
                                        cursor::State::Index(idx) => {
                                            value.insert_str(idx, text);
                                        }
                                        cursor::State::Selection { start, end } => {
                                            value.replace_range(sorted_range(start, end), text);
                                        }
                                    }

                                    // We check if it's now a valid number
                                    if check_value(&value) {
                                        forward_to_text(self, child, clipboard)
                                    } else {
                                        return;
                                    }
                                }
                                // If we are not trying to input text
                                None => return,
                            },
                        }
                    }
                }
            }
            // Mouse scroll event
            Event::Mouse(mouse::Event::WheelScrolled { delta })
                if mouse_over_widget && !self.ignore_scroll_events =>
            {
                match delta {
                    mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                        if y.is_sign_positive() {
                            self.increase_value(shell);
                        } else {
                            self.decrease_value(shell);
                        }
                    }
                }
            }
            // Clicking on the buttons up or down
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                if mouse_over_button && !self.ignore_buttons =>
            {
                if mouse_over_dec {
                    modifiers.decrease_pressed = true;
                    self.decrease_value(shell);
                } else {
                    modifiers.increase_pressed = true;
                    self.increase_value(shell);
                }
            }
            // Releasing the buttons
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if mouse_over_button =>
            {
                if mouse_over_dec {
                    modifiers.decrease_pressed = false;
                } else {
                    modifiers.increase_pressed = false;
                }
            }
            // Any other event are just forwarded
            _ => forward_to_text(self, child, clipboard),
        };

        // We forward the shell of the [`TypedInput`] to the application
        if sub_shell.redraw_request() == RedrawRequest::NextFrame {
            shell.request_redraw();
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
                    if self.value != value || self.value.is_zero() {
                        self.value = value.clone();
                        if let Some(on_change) = &self.on_change {
                            shell.publish(on_change(value));
                        }
                    }
                    shell.invalidate_layout();
                }
                InternalMessage::OnSubmit(result) => {
                    if let Err(text) = result {
                        assert!(
                            text.is_empty(),
                            "We shouldn't be able to submit a number input with an invalid value"
                        );
                    }
                    if let Some(on_submit) = &self.on_submit {
                        shell.publish(on_submit.clone());
                    }
                    shell.invalidate_layout();
                }
                InternalMessage::OnPaste(value) => {
                    if self.value != value {
                        self.value = value.clone();
                        if let Some(on_paste) = &self.on_paste {
                            shell.publish(on_paste(value));
                        }
                    }
                    shell.invalidate_layout();
                }
            }
        }
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let mut children = layout.children();
        let _content_layout = children.next().expect("fail to get content layout");
        let mut mod_children = children
            .next()
            .expect("fail to get modifiers layout")
            .children();
        let inc_bounds = mod_children
            .next()
            .expect("fail to get increase mod layout")
            .bounds();
        let dec_bounds = mod_children
            .next()
            .expect("fail to get decrease mod layout")
            .bounds();
        let is_mouse_over = bounds.contains(cursor.position().unwrap_or_default());
        let is_decrease_disabled = !self.can_decrease();
        let is_increase_disabled = !self.can_increase();
        let mouse_over_decrease = dec_bounds.contains(cursor.position().unwrap_or_default());
        let mouse_over_increase = inc_bounds.contains(cursor.position().unwrap_or_default());

        if ((mouse_over_decrease && !is_decrease_disabled)
            || (mouse_over_increase && !is_increase_disabled))
            && !self.ignore_buttons
        {
            mouse::Interaction::Pointer
        } else if is_mouse_over {
            mouse::Interaction::Text
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();
        let content_layout = children.next().expect("fail to get content layout");
        let mut mod_children = children
            .next()
            .expect("fail to get modifiers layout")
            .children();
        let inc_bounds = mod_children
            .next()
            .expect("fail to get increase mod layout")
            .bounds();
        let dec_bounds = mod_children
            .next()
            .expect("fail to get decrease mod layout")
            .bounds();
        self.content.draw(
            &state.children[0],
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            viewport,
        );
        let is_decrease_disabled = !self.can_decrease();
        let is_increase_disabled = !self.can_increase();

        let decrease_btn_style = if is_decrease_disabled {
            style::number_input::Catalog::style(theme, &self.class, Status::Disabled)
        } else if state.state.downcast_ref::<ModifierState>().decrease_pressed {
            style::number_input::Catalog::style(theme, &self.class, Status::Pressed)
        } else {
            style::number_input::Catalog::style(theme, &self.class, Status::Active)
        };

        let increase_btn_style = if is_increase_disabled {
            style::number_input::Catalog::style(theme, &self.class, Status::Disabled)
        } else if state.state.downcast_ref::<ModifierState>().increase_pressed {
            style::number_input::Catalog::style(theme, &self.class, Status::Pressed)
        } else {
            style::number_input::Catalog::style(theme, &self.class, Status::Active)
        };

        let txt_size = self.size.unwrap_or_else(|| renderer.default_size());

        let icon_size = txt_size * 2.5 / 4.0;

        if self.ignore_buttons {
            return;
        }
        // decrease button section
        if dec_bounds.intersects(viewport) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: dec_bounds,
                    border: Border {
                        radius: (3.0).into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    ..renderer::Quad::default()
                },
                decrease_btn_style
                    .button_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        renderer.fill_text(
            iced::advanced::text::Text {
                content: icon_to_string(RequiredIcons::CaretDownFill),
                bounds: Size::new(dec_bounds.width, dec_bounds.height),
                size: icon_size,
                font: REQUIRED_FONT,
                align_x: Horizontal::Center.into(),
                align_y: Vertical::Center.into(),
                line_height: LineHeight::Relative(1.3),
                shaping: iced::advanced::text::Shaping::Advanced,
                wrapping: Wrapping::default(),
            },
            Point::new(dec_bounds.center_x(), dec_bounds.center_y()),
            decrease_btn_style.icon_color,
            dec_bounds,
        );

        // increase button section
        if inc_bounds.intersects(viewport) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: inc_bounds,
                    border: Border {
                        radius: (3.0).into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    ..renderer::Quad::default()
                },
                increase_btn_style
                    .button_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        renderer.fill_text(
            iced::advanced::text::Text {
                content: icon_to_string(RequiredIcons::CaretUpFill),
                bounds: Size::new(inc_bounds.width, inc_bounds.height),
                size: icon_size,
                font: REQUIRED_FONT,
                align_x: Horizontal::Center.into(),
                align_y: Vertical::Center.into(),
                line_height: LineHeight::Relative(1.3),
                shaping: iced::advanced::text::Shaping::Advanced,
                wrapping: Wrapping::default(),
            },
            Point::new(inc_bounds.center_x(), inc_bounds.center_y()),
            increase_btn_style.icon_color,
            inc_bounds,
        );
    }
}

/// The modifier state of a [`NumberInput`].
#[derive(Default, Clone, Debug)]
pub struct ModifierState {
    /// The state of decrease button on a [`NumberInput`].
    pub decrease_pressed: bool,
    /// The state of increase button on a [`NumberInput`].
    pub increase_pressed: bool,
}

impl<'a, T, Message, Theme, Renderer> From<NumberInput<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: 'a + Num + NumAssignOps + PartialOrd + Display + FromStr + Clone + Bounded,
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: 'a + number_input::ExtendedCatalog,
{
    fn from(num_input: NumberInput<'a, T, Message, Theme, Renderer>) -> Self {
        Element::new(num_input)
    }
}

fn sorted_range<T: PartialOrd>(a: T, b: T) -> std::ops::Range<T> {
    if a >= b {
        b..a
    } else {
        a..b
    }
}
