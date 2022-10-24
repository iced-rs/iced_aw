//! Display fields that can only be filled with numeric type.
//!
//! A [`NumberInput`] has some local [`State`].

use iced_native::{
    alignment::{Horizontal, Vertical},
    event, keyboard,
    layout::{Limits, Node},
    mouse, renderer,
    widget::{
        self, container, text,
        text_input::{self, cursor, Value},
        tree::{self, Tree},
        Column, Container, Operation, Row, Text, TextInput,
    },
    Alignment, Background, Clipboard, Color, Element, Event, Layout, Length, Padding, Point,
    Rectangle, Shell, Size, Widget,
};
use num_traits::{Num, NumAssignOps};
use std::{fmt::Display, str::FromStr};

pub use crate::{
    graphics::icons::Icon,
    style::number_input::{self, Appearance, StyleSheet},
};

/// The default padding
const DEFAULT_PADDING: u16 = 5;

/// A field that can only be filled with numeric type.
///
/// # Example
/// ```
/// # use iced_native::renderer::Null;
/// # use iced_aw::number_input;
/// #
/// # pub type NumberInput<'a, T, Message> = number_input::NumberInput<'a, T, Message, Null>;
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
///     max,
///     Message::NumberInputChanged,
/// )
/// .step(2);
/// ```
#[allow(missing_debug_implementations)]
pub struct NumberInput<'a, T, Message, Renderer>
where
    Renderer: iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: number_input::StyleSheet
        + text_input::StyleSheet
        + container::StyleSheet
        + text::StyleSheet,
{
    /// The current value of the [`NumberInput`](NumberInput).
    value: T,
    /// The step for each modify of the [`NumberInput`](NumberInput).
    step: T,
    /// The min and max value of the [`NumberInput`](NumberInput).
    bounds: (T, T),
    /// The content padding of the [`NumberInput`](NumberInput).
    padding: u16,
    /// The text size of the [`NumberInput`](NumberInput).
    size: Option<u16>,
    /// The underlying element of the [`NumberInput`](NumberInput).
    content: TextInput<'a, Message, Renderer>,
    /// The on_change event of the [`NumberInput`](NumberInput).
    on_change: Box<dyn Fn(T) -> Message>,
    /// The style of the [`NumberInput`](NumberInput).
    style: <Renderer::Theme as number_input::StyleSheet>::Style,
    /// The font text of the [`NumberInput`](NumberInput).
    font: Renderer::Font,
}

impl<'a, T, Message, Renderer> NumberInput<'a, T, Message, Renderer>
where
    T: Num + NumAssignOps + PartialOrd + Display + FromStr + Copy,
    Message: Clone,
    Renderer: iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: number_input::StyleSheet
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
    pub fn new<F>(value: T, max: T, on_changed: F) -> Self
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
            bounds: (T::zero(), max),
            padding,
            size: None,
            content: TextInput::new("", format!("{value}").as_str(), convert_to_num)
                .padding(padding)
                .width(Length::Units(127)),
            on_change: Box::new(on_changed),
            style: <Renderer::Theme as number_input::StyleSheet>::Style::default(),
            font: iced_graphics::Font::default(),
        }
    }

    /// Sets the step of the [`NumberInput`].
    #[must_use]
    pub fn step(mut self, step: T) -> Self {
        self.step = step;
        self
    }

    /// Sets the minimum value of the [`NumberInput`].
    #[must_use]
    pub fn min(mut self, min: T) -> Self {
        if min < self.bounds.1 {
            self.bounds.0 = min;
        }
        self
    }

    /// Sets the maximum value of the [`NumberInput`].
    #[must_use]
    pub fn max(mut self, max: T) -> Self {
        if max > self.bounds.0 {
            self.bounds.1 = max;
        }
        self
    }

    /// Sets the minimum & maximum value (bound) of the [`NumberInput`].
    #[must_use]
    pub fn bounds(mut self, bounds: (T, T)) -> Self {
        if bounds.0 < bounds.1 {
            self.bounds = bounds;
        }
        self
    }

    /// Sets the [ `Font`] of the [`Text`].
    ///
    /// [`Font`]: crate::widget::text::Renderer::Font
    /// [`Text`]: crate::widget::Text
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self.content = self.content.font(font);
        self
    }

    /// Sets the width of the [`NumberInput`].
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.content = self.content.width(width);
        self
    }

    /// Sets the padding of the [`NumberInput`].
    #[must_use]
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self.content = self.content.padding(units);
        self
    }

    /// Sets the text size of the [`NumberInput`].
    #[must_use]
    pub fn size(mut self, size: u16) -> Self {
        self.size = Some(size);
        self.content = self.content.size(size);
        self
    }

    /// Sets the message that should be produced when the [`NumberInput`] is
    /// focused and the enter key is pressed.
    #[must_use]
    pub fn on_submit(mut self, message: Message) -> Self {
        self.content = self.content.on_submit(message);
        self
    }

    /// Sets the style of the [`NumberInput`].
    #[must_use]
    pub fn style(
        mut self,
        style: impl Into<<Renderer::Theme as number_input::StyleSheet>::Style>,
    ) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the input style of the [`NumberInput`].
    #[must_use]
    pub fn input_style(
        mut self,
        style: impl Into<<Renderer::Theme as iced_style::text_input::StyleSheet>::Style>,
    ) -> Self {
        self.content = self.content.style(style);
        self
    }

    /// Decrease current value by step of the [`NumberInput`].
    fn decrease_val(&mut self, shell: &mut Shell<Message>) {
        if self.value > self.bounds.0 {
            let new_val = self.value - self.step;
            self.value = if new_val > self.bounds.0 {
                new_val
            } else {
                self.bounds.0
            };
            shell.publish((self.on_change)(self.value));
        }
    }

    /// Increase current value by step of the [`NumberInput`].
    fn increase_val(&mut self, shell: &mut Shell<Message>) {
        if self.value < self.bounds.1 {
            let new_val = self.value + self.step;
            self.value = if new_val < self.bounds.1 {
                new_val
            } else {
                self.bounds.1
            };
            shell.publish((self.on_change)(self.value));
        }
    }
}

impl<'a, T, Message, Renderer> Widget<Message, Renderer> for NumberInput<'a, T, Message, Renderer>
where
    T: Num + NumAssignOps + PartialOrd + Display + FromStr + ToString + Copy,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: number_input::StyleSheet
        + text_input::StyleSheet
        + container::StyleSheet
        + text::StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ModifierState>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(ModifierState::default())
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

    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let padding = Padding::from(self.padding);
        let limits = limits
            .width(self.width())
            .height(Length::Shrink)
            .pad(padding);
        let content = self.content.layout(renderer, &limits.loose());
        let txt_size = self.size.unwrap_or_else(|| renderer.default_size());
        let icon_size = txt_size * 3 / 4;
        let btn_mod = |c| {
            Container::<(), Renderer>::new(Text::new(format!(" {c} ")).size(icon_size))
                .center_y()
                .center_x()
        };
        let mut modifier = if self.padding < DEFAULT_PADDING {
            Row::<(), Renderer>::new()
                .spacing(1)
                .width(Length::Shrink)
                .push(btn_mod('+'))
                .push(btn_mod('-'))
                .layout(renderer, &limits.loose())
        } else {
            Column::<(), Renderer>::new()
                .spacing(1)
                .width(Length::Shrink)
                .push(btn_mod('▲'))
                .push(btn_mod('▼'))
                .layout(renderer, &limits.loose())
        };
        let intrinsic = Size::new(
            content.size().width - 3.0,
            content.size().height.max(modifier.size().height),
        );
        modifier.align(Alignment::End, Alignment::Center, intrinsic);
        let size = limits.resolve(intrinsic);
        Node::with_children(size, vec![content, modifier])
    }

    fn operate(&self, tree: &mut Tree, layout: Layout<'_>, operation: &mut dyn Operation<Message>) {
        operation.container(None, &mut |operation| {
            self.content.operate(
                &mut tree.children[0],
                layout
                    .children()
                    .next()
                    .expect("NumberInput inner child Textbox was not created."),
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
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
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
        let mouse_over_inc = inc_bounds.contains(cursor_position);
        let mouse_over_dec = dec_bounds.contains(cursor_position);
        let modifiers = state.state.downcast_mut::<ModifierState>();
        let child = &mut state.children[0];

        if layout.bounds().contains(cursor_position) {
            if mouse_over_inc || mouse_over_dec {
                let mut event_status = event::Status::Captured;
                match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                        if mouse_over_dec {
                            modifiers.decrease_pressed = true;
                            self.decrease_val(shell);
                        } else if mouse_over_inc {
                            modifiers.increase_pressed = true;
                            self.increase_val(shell);
                        } else {
                            event_status = event::Status::Ignored;
                        }
                    }
                    Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                        if mouse_over_dec {
                            modifiers.decrease_pressed = false;
                        } else if mouse_over_inc {
                            modifiers.increase_pressed = false;
                        } else {
                            event_status = event::Status::Ignored;
                        }
                    }
                    _ => event_status = event::Status::Ignored,
                }
                event_status
            } else {
                match event {
                    Event::Keyboard(keyboard::Event::CharacterReceived(c))
                        if child
                            .state
                            .downcast_mut::<widget::text_input::State>()
                            .is_focused()
                            && c.is_numeric() =>
                    {
                        let mut new_val = self.value.to_string();
                        match child
                            .state
                            .downcast_mut::<widget::text_input::State>()
                            .cursor()
                            .state(&Value::new(&new_val))
                        {
                            cursor::State::Index(idx) => {
                                if T::zero().eq(&self.value) {
                                    new_val = c.to_string();
                                } else {
                                    new_val.insert(idx, c);
                                }
                            }
                            cursor::State::Selection { start, end } => {
                                if (0..new_val.len()).contains(&start)
                                    && (0..new_val.len()).contains(&end)
                                {
                                    new_val.replace_range(
                                        if start > end { end..start } else { start..end },
                                        &c.to_string(),
                                    );
                                }
                            }
                        }

                        match T::from_str(&new_val) {
                            Ok(val) => {
                                if (self.bounds.0..=self.bounds.1).contains(&val) {
                                    self.value = val;
                                    shell.publish((self.on_change)(self.value));
                                    self.content.on_event(
                                        child,
                                        event.clone(),
                                        content,
                                        cursor_position,
                                        renderer,
                                        clipboard,
                                        shell,
                                    )
                                } else {
                                    event::Status::Ignored
                                }
                            }
                            Err(_) => event::Status::Ignored,
                        }
                    }
                    Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. })
                        if child
                            .state
                            .downcast_mut::<widget::text_input::State>()
                            .is_focused() =>
                    {
                        match key_code {
                            keyboard::KeyCode::Up => {
                                self.increase_val(shell);
                                event::Status::Captured
                            }
                            keyboard::KeyCode::Down => {
                                self.decrease_val(shell);
                                event::Status::Captured
                            }
                            keyboard::KeyCode::Backspace => {
                                if T::zero().eq(&self.value) {
                                    event::Status::Ignored
                                } else {
                                    let mut new_val = self.value.to_string();
                                    match child
                                        .state
                                        .downcast_mut::<widget::text_input::State>()
                                        .cursor()
                                        .state(&Value::new(&new_val))
                                    {
                                        cursor::State::Index(idx) => {
                                            if idx >= 1 && idx <= new_val.len() {
                                                if new_val.len() == 1 {
                                                    new_val = if self.bounds.0 > T::zero() {
                                                        self.bounds.0
                                                    } else {
                                                        T::zero()
                                                    }
                                                    .to_string();
                                                } else {
                                                    let _ = new_val.remove(idx - 1);
                                                }
                                            }
                                        }
                                        cursor::State::Selection { start, end } => {
                                            if (0..new_val.len()).contains(&start)
                                                && (0..new_val.len()).contains(&end)
                                            {
                                                new_val.replace_range(
                                                    if start > end {
                                                        end..start
                                                    } else {
                                                        start..end
                                                    },
                                                    "",
                                                );
                                            }
                                        }
                                    }

                                    match T::from_str(&new_val) {
                                        Ok(val) => {
                                            if (self.bounds.0..=self.bounds.1).contains(&val) {
                                                self.value = val;
                                                shell.publish((self.on_change)(self.value));
                                                self.content.on_event(
                                                    child,
                                                    event.clone(),
                                                    content,
                                                    cursor_position,
                                                    renderer,
                                                    clipboard,
                                                    shell,
                                                )
                                            } else {
                                                event::Status::Ignored
                                            }
                                        }
                                        Err(_) => event::Status::Ignored,
                                    }
                                }
                            }
                            _ => self.content.on_event(
                                child,
                                event.clone(),
                                content,
                                cursor_position,
                                renderer,
                                clipboard,
                                shell,
                            ),
                        }
                    }
                    // This section from line 502 to 516 was owned by 13r0ck (https://github.com/13r0ck).
                    Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                        let negative = match delta {
                            mouse::ScrollDelta::Lines { y, .. }
                            | mouse::ScrollDelta::Pixels { y, .. } => y.is_sign_negative(),
                        };
                        if negative {
                            self.increase_val(shell);
                        } else {
                            self.decrease_val(shell);
                        }
                        event::Status::Captured
                    }
                    _ => self.content.on_event(
                        child,
                        event,
                        content,
                        cursor_position,
                        renderer,
                        clipboard,
                        shell,
                    ),
                }
            }
        } else {
            match event {
                Event::Keyboard(_) => event::Status::Ignored,
                _ => self.content.on_event(
                    child,
                    event,
                    content,
                    cursor_position,
                    renderer,
                    clipboard,
                    shell,
                ),
            }
        }
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
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
        let is_mouse_over = bounds.contains(cursor_position);
        let is_decrease_disabled = self.value <= self.bounds.0;
        let is_increase_disabled = self.value >= self.bounds.1;
        let mouse_over_decrease = dec_bounds.contains(cursor_position);
        let mouse_over_increase = inc_bounds.contains(cursor_position);

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
        theme: &Renderer::Theme,
        _style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_graphics::Point,
        _viewport: &iced_graphics::Rectangle,
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
            cursor_position,
            None,
        );
        let is_decrease_disabled = self.value <= self.bounds.0;
        let is_increase_disabled = self.value >= self.bounds.1;

        let decrease_btn_style = if is_decrease_disabled {
            theme.disabled(self.style)
        } else if state.state.downcast_ref::<ModifierState>().decrease_pressed {
            theme.pressed(self.style)
        } else {
            theme.active(self.style)
        };

        let increase_btn_style = if is_increase_disabled {
            theme.disabled(self.style)
        } else if state.state.downcast_ref::<ModifierState>().increase_pressed {
            theme.pressed(self.style)
        } else {
            theme.active(self.style)
        };

        // decrease button section
        renderer.fill_quad(
            renderer::Quad {
                bounds: dec_bounds,
                border_radius: 3.0,
                border_width: 0.,
                border_color: Color::TRANSPARENT,
            },
            decrease_btn_style
                .button_background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );

        let mut buffer = [0; 4];

        renderer.fill_text(iced_native::text::Text {
            content: char::from(Icon::CaretDownFill).encode_utf8(&mut buffer),
            bounds: Rectangle {
                x: dec_bounds.center_x(),
                y: dec_bounds.center_y(),
                ..dec_bounds
            },
            size: dec_bounds.height,
            color: decrease_btn_style.icon_color,
            font: crate::graphics::icons::ICON_FONT,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
        });

        // increase button section
        renderer.fill_quad(
            renderer::Quad {
                bounds: inc_bounds,
                border_radius: 3.0,
                border_width: 0.,
                border_color: Color::TRANSPARENT,
            },
            increase_btn_style
                .button_background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );

        renderer.fill_text(iced_native::text::Text {
            content: char::from(Icon::CaretUpFill).encode_utf8(&mut buffer),
            bounds: Rectangle {
                x: inc_bounds.center_x(),
                y: inc_bounds.center_y(),
                ..inc_bounds
            },
            size: inc_bounds.height,
            color: increase_btn_style.icon_color,
            font: crate::graphics::icons::ICON_FONT,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
        });
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

impl<'a, T, Message, Renderer> From<NumberInput<'a, T, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    T: 'a + Num + NumAssignOps + PartialOrd + Display + FromStr + Copy,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: number_input::StyleSheet
        + text_input::StyleSheet
        + container::StyleSheet
        + text::StyleSheet,
{
    fn from(num_input: NumberInput<'a, T, Message, Renderer>) -> Self {
        Element::new(num_input)
    }
}
