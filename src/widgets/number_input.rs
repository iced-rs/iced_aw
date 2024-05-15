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
    event, keyboard,
    mouse::{self, Cursor},
    widget::{
        container, text,
        text::LineHeight,
        text_input::{self, cursor, Value},
        Column, Container, Row, Text, TextInput,
    },
    Alignment, Background, Border, Color, Element, Event, Length, Padding, Pixels, Point,
    Rectangle, Shadow, Size,
};
use num_traits::{bounds::Bounded, Num, NumAssignOps};
use std::{
    fmt::Display,
    ops::{Bound, RangeBounds},
    str::FromStr,
};

use crate::style;
pub use crate::{
    core::icons::{bootstrap::icon_to_string, Bootstrap, BOOTSTRAP_FONT},
    style::number_input::{self, Appearance, StyleSheet},
};

/// The default padding
const DEFAULT_PADDING: f32 = 5.0;

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
    Theme: number_input::StyleSheet
        + text_input::StyleSheet
        + container::StyleSheet
        + text::StyleSheet,
{
    /// The current value of the [`NumberInput`].
    value: T,
    /// The step for each modify of the [`NumberInput`].
    step: T,
    /// The min value of the [`NumberInput`].
    min: T,
    /// The max value of the [`NumberInput`].
    max: T,
    /// The content padding of the [`NumberInput`].
    padding: f32,
    /// The text size of the [`NumberInput`].
    size: Option<f32>,
    /// The underlying element of the [`NumberInput`].
    content: TextInput<'a, Message, Theme, Renderer>,
    /// The ``on_change`` event of the [`NumberInput`].
    on_change: Box<dyn Fn(T) -> Message>,
    /// The style of the [`NumberInput`].
    style: <Theme as number_input::StyleSheet>::Style,
    /// The font text of the [`NumberInput`].
    font: Renderer::Font,
    /// The Width to use for the ``NumberBox`` Default is ``Length::Fill``
    width: Length,
    /// Ignore mouse scroll events for the [`NumberInput`] Default is ``false``.
    ignore_scroll_events: bool,
}

impl<'a, T, Message, Theme, Renderer> NumberInput<'a, T, Message, Theme, Renderer>
where
    T: Num + NumAssignOps + PartialOrd + Display + FromStr + Copy + Bounded,
    Message: Clone,
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: number_input::StyleSheet
        + text_input::StyleSheet
        + container::StyleSheet
        + text::StyleSheet,
{
    /// Creates a new [`NumberInput`].
    ///
    /// It expects:
    /// - some [`State`]
    /// - the current value
    /// - the max value
    /// - a function that produces a message when the [`NumberInput`] changes
    pub fn new<F>(value: T, bounds: impl RangeBounds<T>, on_changed: F) -> Self
    where
        F: 'static + Fn(T) -> Message + Copy,
        T: 'static,
    {
        let padding = DEFAULT_PADDING;
        let convert_to_num = move |s: String| {
            on_changed(T::from_str(&s).unwrap_or(if s.is_empty() { T::zero() } else { value }))
        };

        Self {
            value,
            step: T::one(),
            min: Self::set_min(bounds.start_bound()),
            max: Self::set_max(bounds.end_bound()),
            padding,
            size: None,
            content: TextInput::new("", format!("{value}").as_str())
                .on_input(convert_to_num)
                .padding(padding)
                .width(Length::Fixed(127.0)),
            on_change: Box::new(on_changed),
            style: <Theme as number_input::StyleSheet>::Style::default(),
            font: Renderer::Font::default(),
            width: Length::Shrink,
            ignore_scroll_events: false,
        }
    }

    /// Sets the minimum & maximum value (bound) of the [`NumberInput`].
    /// # Example
    /// ```
    /// // Creates a range from -5 till 5.
    /// number_input(my_value, my_message).bounds(-5..=5)
    /// ```
    #[must_use]
    pub fn bounds(mut self, bounds: impl RangeBounds<T>) -> Self {
        self.min = Self::set_min(bounds.start_bound());
        self.max = Self::set_max(bounds.end_bound());

        self
    }

    /// Sets the content width of the [`NumberInput`].
    #[must_use]
    pub fn content_width(mut self, width: Length) -> Self {
        self.content = self.content.width(width);
        self
    }

    /// Sets the [`Font`] of the [`Text`].
    ///
    /// [`Font`]: core::Font
    /// [`Text`]: core::widget::Text
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self.content = self.content.font(font);
        self
    }

    /// Sets the message that should be produced when the [`NumberInput`] is
    /// focused and the enter key is pressed.
    #[must_use]
    pub fn on_submit(mut self, message: Message) -> Self {
        self.content = self.content.on_submit(message);
        self
    }

    /// Sets the padding of the [`NumberInput`].
    #[must_use]
    pub fn padding(mut self, units: f32) -> Self {
        self.padding = units;
        self.content = self.content.padding(units);
        self
    }

    /// Sets the text size of the [`NumberInput`].
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self.content = self.content.size(size);
        self
    }

    /// Sets the step of the [`NumberInput`].
    #[must_use]
    pub fn step(mut self, step: T) -> Self {
        self.step = step;
        self
    }

    /// Sets the style of the [`NumberInput`].
    #[must_use]
    pub fn style(mut self, style: impl Into<<Theme as number_input::StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the width of the [`NumberInput`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
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
        if self.value < self.min + self.step {
            self.value = self.min;
        } else {
            self.value -= self.step;
        }

        shell.publish((self.on_change)(self.value));
    }

    /// Increase current value by step of the [`NumberInput`].
    fn increase_value(&mut self, shell: &mut Shell<Message>) {
        if self.value > self.max - self.step {
            self.value = self.max;
        } else {
            self.value += self.step;
        }
        shell.publish((self.on_change)(self.value));
    }

    fn set_min(min: Bound<&T>) -> T {
        match min {
            Bound::Included(n) | Bound::Excluded(n) => *n,
            Bound::Unbounded => T::min_value(),
        }
    }

    fn set_max(max: Bound<&T>) -> T {
        match max {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => *n - T::one(),
            Bound::Unbounded => T::max_value(),
        }
    }
}

impl<'a, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for NumberInput<'a, T, Message, Theme, Renderer>
where
    T: Num + NumAssignOps + PartialOrd + Display + FromStr + ToString + Copy + Bounded,
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: number_input::StyleSheet
        + text_input::StyleSheet
        + container::StyleSheet
        + text::StyleSheet,
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
        Size::new(self.width, Length::Shrink)
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let padding = Padding::from(self.padding);
        let num_size = self.size();
        let limits = limits
            .width(num_size.width)
            .height(Length::Shrink)
            .shrink(padding);
        let content = self
            .content
            .layout(&mut tree.children[0], renderer, &limits, None);
        let limits2 = Limits::new(Size::new(0.0, 0.0), content.size());
        let txt_size = self.size.unwrap_or_else(|| renderer.default_size().0);

        let icon_size = txt_size * 2.5 / 4.0;
        let btn_mod = |c| {
            Container::<Message, Theme, Renderer>::new(Text::new(format!(" {c} ")).size(icon_size))
                .center_y()
                .center_x()
        };

        let element = if self.padding < DEFAULT_PADDING {
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
        operation: &mut dyn Operation<Message>,
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
            .expect("fail to get decreate mod layout")
            .bounds();

        if self.min == self.max {
            return event::Status::Ignored;
        }

        let cursor_position = cursor.position().unwrap_or_default();
        let mouse_over_widget = layout.bounds().contains(cursor_position);
        let mouse_over_inc = inc_bounds.contains(cursor_position);
        let mouse_over_dec = dec_bounds.contains(cursor_position);
        let mouse_over_button = mouse_over_inc || mouse_over_dec;

        let child = state.children.get_mut(0).expect("fail to get child");
        let text_input = child
            .state
            .downcast_mut::<text_input::State<Renderer::Paragraph>>();
        let modifiers = state.state.downcast_mut::<ModifierState>();

        let mut forward_to_text = |event, shell, child, clipboard| {
            self.content.on_event(
                child, event, content, cursor, renderer, clipboard, shell, viewport,
            )
        };

        match &event {
            Event::Keyboard(ke) => {
                if !text_input.is_focused() {
                    return event::Status::Ignored;
                }
                let (key, modifiers) = match ke {
                    keyboard::Event::KeyPressed { key, modifiers, .. } => (key, modifiers),
                    keyboard::Event::ModifiersChanged(_) => {
                        return forward_to_text(event, shell, child, clipboard)
                    }
                    keyboard::Event::KeyReleased { .. } => return event::Status::Ignored,
                };

                match key {
                    keyboard::Key::Named(keyboard::key::Named::ArrowDown) => {
                        self.decrease_value(shell);
                        event::Status::Captured
                    }
                    keyboard::Key::Named(keyboard::key::Named::ArrowUp) => {
                        self.increase_value(shell);
                        event::Status::Captured
                    }
                    keyboard::Key::Named(
                        keyboard::key::Named::ArrowLeft | keyboard::key::Named::ArrowRight,
                    ) => forward_to_text(event, shell, child, clipboard),
                    keyboard::Key::Named(keyboard::key::Named::Backspace)
                        if !T::zero().eq(&self.value) =>
                    {
                        let mut new_val = self.value.to_string();
                        match text_input.cursor().state(&Value::new(&new_val)) {
                            cursor::State::Index(idx) if idx >= 1 && idx <= new_val.len() => {
                                _ = new_val.remove(idx - 1);
                            }
                            cursor::State::Selection { start, end }
                                if start <= new_val.len() && end <= new_val.len() =>
                            {
                                new_val.replace_range(start.min(end)..start.max(end), "");
                            }
                            _ => return event::Status::Ignored,
                        }

                        if new_val.is_empty() {
                            new_val = T::zero().to_string();
                        }

                        match T::from_str(&new_val) {
                            Ok(val) if (self.min..self.max).contains(&val) && val != self.value => {
                                self.value = val;
                                forward_to_text(event, shell, child, clipboard)
                            }
                            Ok(_) => event::Status::Captured,
                            _ => event::Status::Ignored,
                        }
                    }
                    keyboard::Key::Named(_) | keyboard::Key::Unidentified => event::Status::Ignored,
                    keyboard::Key::Character(c) => {
                        if modifiers.command() && matches!(c.as_ref(), "a" | "c") {
                            return forward_to_text(event, shell, child, clipboard);
                        }

                        let input = if modifiers.command() && c.as_ref() == "v" {
                            match clipboard.read(iced::advanced::clipboard::Kind::Standard) {
                                Some(paste) => paste,
                                None => return event::Status::Ignored,
                            }
                        } else if c.parse::<i64>().is_err() && c != "-" {
                            return event::Status::Ignored;
                        } else {
                            c.to_string()
                        };
                        let input = input.trim();

                        let mut new_val = self.value.to_string();
                        match text_input.cursor().state(&Value::new(&new_val)) {
                            cursor::State::Index(idx) if idx <= new_val.len() => {
                                new_val.insert_str(idx, input);
                            }
                            cursor::State::Selection { start, end }
                                if start <= new_val.len() && end <= new_val.len() =>
                            {
                                new_val.replace_range(start.min(end)..end.max(start), input);
                            }
                            _ => return event::Status::Ignored,
                        }

                        match T::from_str(&new_val) {
                            Ok(val) if (self.min..self.max).contains(&val) && val != self.value => {
                                self.value = val;
                                forward_to_text(event, shell, child, clipboard)
                            }
                            Ok(_) => event::Status::Captured,
                            _ => event::Status::Ignored,
                        }
                    }
                }
            }
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
                event::Status::Captured
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if mouse_over_button => {
                if mouse_over_dec {
                    modifiers.decrease_pressed = true;
                    self.decrease_value(shell);
                } else {
                    modifiers.increase_pressed = true;
                    self.increase_value(shell);
                }
                event::Status::Captured
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if mouse_over_button =>
            {
                if mouse_over_dec {
                    modifiers.decrease_pressed = false;
                } else {
                    modifiers.increase_pressed = false;
                }
                event::Status::Captured
            }
            _ => forward_to_text(event, shell, child, clipboard),
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
            .expect("fail to get decreate mod layout")
            .bounds();
        let is_mouse_over = bounds.contains(cursor.position().unwrap_or_default());
        let is_decrease_disabled = self.value <= self.min || self.min == self.max;
        let is_increase_disabled = self.value >= self.max || self.min == self.max;
        let mouse_over_decrease = dec_bounds.contains(cursor.position().unwrap_or_default());
        let mouse_over_increase = inc_bounds.contains(cursor.position().unwrap_or_default());

        if (mouse_over_decrease && !is_decrease_disabled)
            || (mouse_over_increase && !is_increase_disabled)
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
        _style: &renderer::Style,
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
            .expect("fail to get decreate mod layout")
            .bounds();
        self.content.draw(
            &state.children[0],
            renderer,
            theme,
            content_layout,
            cursor,
            None,
            viewport,
        );
        let is_decrease_disabled = self.value <= self.min || self.min == self.max;
        let is_increase_disabled = self.value >= self.max || self.min == self.max;

        let decrease_btn_style = if is_decrease_disabled {
            style::number_input::StyleSheet::disabled(theme, &self.style)
            //theme.disabled(&self.style)
        } else if state.state.downcast_ref::<ModifierState>().decrease_pressed {
            style::number_input::StyleSheet::pressed(theme, &self.style)
        } else {
            style::number_input::StyleSheet::active(theme, &self.style)
        };

        let increase_btn_style = if is_increase_disabled {
            style::number_input::StyleSheet::disabled(theme, &self.style)
        } else if state.state.downcast_ref::<ModifierState>().increase_pressed {
            style::number_input::StyleSheet::pressed(theme, &self.style)
        } else {
            style::number_input::StyleSheet::active(theme, &self.style)
        };

        let txt_size = self.size.unwrap_or_else(|| renderer.default_size().0);

        let icon_size = Pixels(txt_size * 2.5 / 4.0);

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
                    shadow: Shadow::default(),
                },
                decrease_btn_style
                    .button_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        renderer.fill_text(
            iced::advanced::text::Text {
                content: &icon_to_string(Bootstrap::CaretDownFill),
                bounds: Size::new(dec_bounds.width, dec_bounds.height),
                size: icon_size,
                font: BOOTSTRAP_FONT,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: LineHeight::Relative(1.3),
                shaping: iced::advanced::text::Shaping::Advanced,
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
                    shadow: Shadow::default(),
                },
                increase_btn_style
                    .button_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        renderer.fill_text(
            iced::advanced::text::Text {
                content: &icon_to_string(Bootstrap::CaretUpFill),
                bounds: Size::new(inc_bounds.width, inc_bounds.height),
                size: icon_size,
                font: BOOTSTRAP_FONT,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: LineHeight::Relative(1.3),
                shaping: iced::advanced::text::Shaping::Advanced,
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
    T: 'a + Num + NumAssignOps + PartialOrd + Display + FromStr + Copy + Bounded,
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: 'a
        + number_input::StyleSheet
        + text_input::StyleSheet
        + container::StyleSheet
        + text::StyleSheet,
{
    fn from(num_input: NumberInput<'a, T, Message, Theme, Renderer>) -> Self {
        Element::new(num_input)
    }
}
