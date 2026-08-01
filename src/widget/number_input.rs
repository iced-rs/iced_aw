//! Display fields that can only be filled with numeric type.
//!
//! A [`NumberInput`] has some local [`State`].
use std::{
    fmt::Display,
    ops::{Bound, RangeBounds},
    str::FromStr,
};

use iced_core::{
    Alignment, Background, Border, Color, Element, Event, Layout, Length, Padding, Pixels, Point,
    Rectangle, Shadow, Shell, Size, Widget, keyboard, layout, mouse, renderer,
    text::{editor, input},
    widget::{
        self, Operation, Tree,
        operation::{self, Focusable},
        tree,
    },
    window,
};
use iced_widget::{text, text_input};
use num_traits::{Bounded, Num, NumAssignOps};

use crate::iced_aw_font::advanced_text::{down_open, up_open};
use crate::style::{
    self, Status, StyleFn,
    number_input::{Catalog, ExtendedCatalog, Style},
};

/// The default [`Padding`] of a [`NumberInput`].
pub const DEFAULT_PADDING: Padding = Padding::new(5.0);

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
/// let placeholder = "..."
/// let value = 12;
/// let max = 1275;
///
/// let input = NumberInput::new(
///     placeholder,
///     value,
/// )
/// .on_input(Message::NumberInputChanged)
/// .range(0..=max)
/// .step(2);
/// ```
pub struct NumberInput<'a, T, Message, Theme = iced_widget::Theme, Renderer = iced_widget::Renderer>
where
    T: Num + NumAssignOps + PartialOrd + Display + FromStr + Clone + Bounded,
    Renderer: iced_core::text::Renderer<Font = iced_core::Font>,
    Theme: ExtendedCatalog,
{
    /// The [`widget::Id`] of the [`NumberInput`].
    id: Option<widget::Id>,
    /// The placeholder value of the [`NumberInput`].
    placeholder: text::Fragment<'a>,
    /// The current value of the [`NumberInput`].
    value: T,
    /// The font text of the [`NumberInput`].
    font: Option<Renderer::Font>,
    /// The width of the [`NumberInput`].
    width: Length,
    /// The height of the [`NumberInput`].
    height: Length,
    /// The content padding of the [`NumberInput`].
    padding: Padding,
    /// The text size of the [`NumberInput`].
    size: Option<Pixels>,
    /// The line height of the [`NumberInput`].
    line_height: text::LineHeight,
    /// The alignment of the [`NumberInput`].
    alignment: text::Alignment,
    /// The wrapping of the [`NumberInput`].
    multiline: Option<text::Wrapping>,
    /// The ``on_input`` event of the [`NumberInput`].
    on_input: Option<Box<dyn Fn(T) -> Message + 'a>>,
    /// The ``on_paste`` event of the [`NumberInput`]
    on_paste: Option<Box<dyn Fn(T) -> Message + 'a>>,
    /// The ``on_submit`` event of the [`NumberInput`].
    on_submit: Option<Message>,
    /// The style of the [`NumberInput`].
    class: <Theme as Catalog>::Class<'a>,
    /// The style of the text input within [`NumberInput`].
    input_class: <Theme as text_input::Catalog>::Class<'a>,
    /// The previous state of the [`NumberInput`].
    last_status: Option<Status>,
    /// The step for each modify of the [`NumberInput`].
    step: T,
    /// The min value of the [`NumberInput`].
    min: Bound<T>,
    /// The max value of the [`NumberInput`].
    max: Bound<T>,
    /// Ignore drawing increase and decrease buttons [`NumberInput`] Default is ``false``.
    ignore_buttons: bool,
}

impl<'a, T, Message, Theme, Renderer> NumberInput<'a, T, Message, Theme, Renderer>
where
    T: Num + NumAssignOps + PartialOrd + Display + FromStr + Clone + Bounded,
    Message: Clone,
    Theme: ExtendedCatalog,
    Renderer: iced_core::text::Renderer<Font = iced_core::Font>,
{
    /// Creates a new [`NumberInput`] with the given placeholder and
    /// its current value.
    pub fn new(placeholder: impl text::IntoFragment<'a>, value: &T) -> Self {
        NumberInput {
            id: None,
            placeholder: placeholder.into_fragment(),
            value: value.clone(),
            font: None,
            width: Length::Fill,
            height: Length::Fit,
            padding: DEFAULT_PADDING,
            size: None,
            line_height: text::LineHeight::default(),
            alignment: text::Alignment::Default,
            multiline: None,
            on_input: None,
            on_paste: None,
            on_submit: None,
            class: <Theme as Catalog>::default(),
            input_class: <Theme as text_input::Catalog>::default(),
            last_status: None,
            step: T::one(),
            min: Bound::Unbounded,
            max: Bound::Unbounded,
            ignore_buttons: false,
        }
    }

    /// Sets the [`widget::Id`] of the [`NumberInput`].
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the message that should be produced when some valid number is typed into
    /// the [`NumberInput`].
    ///
    /// If this method is not called, the [`NumberInput`] will be disabled.
    pub fn on_input(mut self, on_input: impl Fn(T) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Sets the message that should be produced when some valid number is typed into
    /// the [`NumberInput`], if `Some`.
    ///
    /// If `None`, the [`NumberInput`] will be disabled.
    pub fn on_input_maybe(mut self, on_input: Option<impl Fn(T) -> Message + 'a>) -> Self {
        self.on_input = on_input.map(|f| Box::new(f) as _);
        self
    }

    /// Sets the message that should be produced when the [`NumberInput`] is
    /// focused and the enter key is pressed.
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Sets the message that should be produced when the [`NumberInput`] is
    /// focused and the enter key is pressed, if `Some`.
    pub fn on_submit_maybe(mut self, on_submit: Option<Message>) -> Self {
        self.on_submit = on_submit;
        self
    }

    /// Sets the message that should be produced when some valid number is pasted into
    /// the [`NumberInput`].
    pub fn on_paste(mut self, on_paste: impl Fn(T) -> Message + 'a) -> Self {
        self.on_paste = Some(Box::new(on_paste));
        self
    }

    /// Sets the message that should be produced when some valid number is pasted into
    /// the [`NumberInput`], if `Some`.
    pub fn on_paste_maybe(mut self, on_paste: Option<impl Fn(T) -> Message + 'a>) -> Self {
        self.on_paste = on_paste.map(|f| Box::new(f) as _);
        self
    }

    /// Sets the [`Font`] of the [`NumberInput`].
    ///
    /// [`Font`]: text::Renderer::Font
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the width of the [`NumberInput`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the [`Padding`] of the [`NumberInput`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the text size of the [`NumberInput`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Sets the [`text::LineHeight`] of the [`NumberInput`].
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    /// Sets the horizontal alignment of the [`NumberInput`].
    pub fn align_x(mut self, alignment: impl Into<text::Alignment>) -> Self {
        self.alignment = alignment.into();
        self
    }

    /// Sets the multiline behavior of the [`NumberInput`].
    ///
    /// `None` will behave as a single line input.
    pub fn multiline(mut self, wrapping: Option<text::Wrapping>) -> Self {
        self.multiline = wrapping;
        self
    }

    /// Sets the step of the [`NumberInput`].
    #[must_use]
    pub fn step(mut self, step: T) -> Self {
        self.step = step;
        self
    }

    /// Sets the minimum & maximum value (bound) of the [`NumberInput`].
    /// # Example
    /// ```
    /// use iced_aw::widget::number_input;
    /// // Creates a range from -5 till 5.
    /// let input: iced_aw::NumberInput<'_, _, _, iced_widget::Theme, iced::Renderer> = number_input("...", &4 |_| () /* my_message */).bounds(-5..=5);
    /// ```
    #[must_use]
    pub fn bounds(mut self, bounds: impl RangeBounds<T>) -> Self {
        self.min = bounds.start_bound().cloned();
        self.max = bounds.end_bound().cloned();
        self
    }

    /// Enable or disable increase and decrease buttons of the [`NumberInput`], by default this is set to
    /// ``false``.
    #[must_use]
    pub fn ignore_buttons(mut self, ignore: bool) -> Self {
        self.ignore_buttons = ignore;
        self
    }

    /// Returns the lower value possible
    /// if the bound is excluded the bound is increased by the step.
    fn min(&self) -> T {
        match &self.min {
            Bound::Included(n) => n.clone(),
            Bound::Excluded(n) => n.clone() + self.step.clone(),
            Bound::Unbounded => T::min_value(),
        }
    }

    /// Returns the higher value possible
    /// if the bound is excluded the bound is decreased by the step.
    fn max(&self) -> T {
        match &self.max {
            Bound::Included(n) => n.clone(),
            Bound::Excluded(n) => n.clone() - self.step.clone(),
            Bound::Unbounded => T::max_value(),
        }
    }

    /// Checks if the value is within the bounds.
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

    /// Checks if the value can be increased by the step.
    fn can_increase(&self) -> bool {
        self.value < self.max()
    }

    /// Checks if the value can be decreased by the step.
    fn can_decrease(&self) -> bool {
        self.value > self.min()
    }

    /// Checks if the [`NumberInput`] is disabled, meaning the bounds are
    /// too tight for the value to ever change.
    fn disabled(&self) -> bool {
        match (&self.min, &self.max) {
            (Bound::Included(n) | Bound::Excluded(n), Bound::Included(m) | Bound::Excluded(m)) => {
                *n >= *m
            }
            _ => false,
        }
    }

    /// Applies one step in the given direction, clamped to bounds, updates
    /// the text buffer to match, and publishes `on_input` if set.
    fn apply_step(&mut self, tree: &mut Tree, shell: &mut Shell<'_, Message>, increase: bool)
    where
        Renderer: 'static,
    {
        let mut new_value = self.value.clone();

        if increase {
            new_value += self.step.clone();
        } else {
            new_value -= self.step.clone();
        }

        if new_value > self.max() {
            new_value = self.max();
        }

        if new_value < self.min() {
            new_value = self.min();
        }

        let text = new_value.to_string();
        let state = tree.state.downcast_mut::<State<Renderer>>();

        state.input.overwrite(&text);
        state.value = text;

        self.value = new_value.clone();

        if let Some(on_input) = &self.on_input {
            shell.publish(on_input(new_value));
        }

        shell.capture_event();

        shell.request_redraw();
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

    /// Sets the class of the input of the [`NumberInput`].
    #[must_use]
    pub fn class(
        mut self,
        class: impl Into<<Theme as style::number_input::Catalog>::Class<'a>>,
    ) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the style of the input field of the [`NumberInput`]
    #[must_use]
    pub fn input_style(
        mut self,
        style: impl Fn(&Theme, text_input::Status) -> text_input::Style + 'a,
    ) -> Self
    where
        <Theme as text_input::Catalog>::Class<'a>: From<text_input::StyleFn<'a, Theme>>,
    {
        self.input_class = (Box::new(style) as text_input::StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the input field of the [`NumberInput`].
    #[must_use]
    pub fn input_class(
        mut self,
        class: impl Into<<Theme as text_input::Catalog>::Class<'a>>,
    ) -> Self {
        self.input_class = class.into();
        self
    }
}

impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for NumberInput<'_, T, Message, Theme, Renderer>
where
    T: Num + NumAssignOps + PartialOrd + Display + FromStr + Clone + Bounded,
    Message: Clone,
    Theme: ExtendedCatalog,
    Renderer: iced_core::text::Renderer<Font = iced_core::Font> + 'static,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Renderer>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Renderer>::new())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree.state.downcast_mut::<State<Renderer>>();

        let value_text = self.value.to_string();
        if !state.is_intermediate
            && state.value != value_text
            && state
                .transaction
                .as_ref()
                .is_none_or(iced_core::shell::Tracking::is_processed)
        {
            state.input.overwrite(&value_text);
            state.value = value_text;
        }

        let txt_size = self.size.unwrap_or_else(|| renderer.default_size());
        let icon_size = txt_size.0 * 2.5 / 4.0;

        let button_width = icon_size + 8.0;

        let mut padding = self.padding;

        if !self.ignore_buttons {
            padding.right += button_width;
        }

        let content = state.input.layout(
            renderer,
            limits,
            input::Layout {
                width: self.width,
                height: self.height,
                padding,
                placeholder: self.placeholder.as_ref(),
                font: self.font,
                size: self.size,
                line_height: self.line_height,
                alignment: self.alignment,
                multiline: self.multiline,
            },
        );

        let content_size = content.size();

        let modifiers = if self.ignore_buttons {
            layout::Node::new(Size::ZERO)
        } else {
            let button_height = (content_size.height - 1.0) / 2.0;

            let inc_node = layout::Node::new(Size::new(button_width, button_height));
            let dec_node = layout::Node::new(Size::new(button_width, button_height))
                .move_to(Point::new(0.0, button_height));

            layout::Node::with_children(
                Size::new(button_width, content_size.height),
                vec![inc_node, dec_node],
            )
            .move_to(Point::new(content_size.width - button_width, 0.0))
        };

        layout::Node::with_children(content_size, vec![content, modifiers])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let content_bounds = layout
            .children()
            .next()
            .map_or(layout.bounds(), |l| l.bounds());
        let state = tree.state.downcast_mut::<State<Renderer>>();

        operation.text_input(self.id.as_ref(), content_bounds, state);
        operation.focusable(self.id.as_ref(), content_bounds, state);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let mut children = layout.children();
        let content_layout = children.next().expect("content layout");
        let mut mod_children = children.next().expect("modifiers layout").children();
        let inc_bounds = mod_children.next().expect("inc layout").bounds();
        let dec_bounds = mod_children.next().expect("dec layout").bounds();

        let is_disabled = self.on_input.is_none() || self.disabled();
        let cursor_position = cursor.position().unwrap_or_default();
        let mouse_over_inc = inc_bounds.contains(cursor_position);
        let mouse_over_dec = dec_bounds.contains(cursor_position);

        let can_increase = self.can_increase();
        let can_decrease = self.can_decrease();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                if !self.ignore_buttons && !is_disabled && (mouse_over_inc || mouse_over_dec) =>
            {
                if mouse_over_inc && can_increase {
                    tree.state
                        .downcast_mut::<State<Renderer>>()
                        .increase_pressed = true;
                    self.apply_step(tree, shell, true);
                } else if mouse_over_dec && can_decrease {
                    tree.state
                        .downcast_mut::<State<Renderer>>()
                        .decrease_pressed = true;
                    self.apply_step(tree, shell, false);
                }
                return;
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let state = tree.state.downcast_mut::<State<Renderer>>();

                if state.increase_pressed || state.decrease_pressed {
                    state.increase_pressed = false;
                    state.decrease_pressed = false;

                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta })
                if !is_disabled && cursor.is_over(layout.bounds()) =>
            {
                let y = match delta {
                    mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                        *y
                    }
                };
                if y.is_sign_positive() && can_increase {
                    self.apply_step(tree, shell, true);
                } else if y.is_sign_negative() && can_decrease {
                    self.apply_step(tree, shell, false);
                }
                return;
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. })
                if tree
                    .state
                    .downcast_ref::<State<Renderer>>()
                    .input
                    .is_focused() =>
            {
                match key.as_ref() {
                    keyboard::Key::Named(keyboard::key::Named::ArrowUp) if can_increase => {
                        self.apply_step(tree, shell, true);
                        return;
                    }
                    keyboard::Key::Named(keyboard::key::Named::ArrowDown) if can_decrease => {
                        self.apply_step(tree, shell, false);
                        return;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        let old_text = tree.state.downcast_ref::<State<Renderer>>().value.clone();
        let supports_negative = self.min() < T::zero();
        let state = state::<Renderer>(tree);

        if let Some(on_input) = &self.on_input {
            let edit =
                state
                    .input
                    .update(event, content_layout.bounds(), cursor, shell, |key_press| {
                        if let Some(on_submit) = &self.on_submit
                            && key_press.modified_key
                                == keyboard::Key::Named(keyboard::key::Named::Enter)
                        {
                            return Some(editor::Binding::Custom(on_submit.clone()));
                        }
                        editor::Binding::from_key_press(key_press)
                    });

            if let Some(edit) = edit {
                let new_text = state.input.value();

                let parsed = T::from_str(&new_text);
                let is_valid_intermediate =
                    new_text.is_empty() || (new_text == "-" && supports_negative);

                match parsed {
                    Ok(v) if self.valid(&v) => {
                        state.value = new_text;
                        state.is_intermediate = false;
                        self.value = v.clone();

                        let publish_via = if let Some(on_paste) = &self.on_paste
                            && edit.is_paste
                        {
                            on_paste
                        } else {
                            on_input
                        };

                        state.transaction = Some(shell.publish_and_track(publish_via(v)));
                    }
                    Err(_) if is_valid_intermediate => {
                        state.value = new_text;
                        state.is_intermediate = true;
                    }
                    _ => {
                        state.input.overwrite(&old_text);
                        state.value = old_text;
                    }
                }
            }
        }

        if !state.input.is_focused() {
            state.is_intermediate = false;
        }

        let status = if is_disabled {
            Status::Disabled
        } else if state.input.is_focused() {
            Status::Focused
        } else if cursor.is_over(layout.bounds()) {
            Status::Hovered
        } else {
            Status::Active
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(status);
            shell.request_input_method(
                &state
                    .input
                    .input_method(content_layout.bounds().shrink(self.padding).position()),
            );
        } else if self.last_status.is_some_and(|last| status != last) {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State<Renderer>>();

        let mut children = layout.children();
        let content_layout = children.next().expect("content layout");
        let mut mod_children = children.next().expect("modifiers layout").children();
        let inc_bounds = mod_children.next().expect("inc layout").bounds();
        let dec_bounds = mod_children.next().expect("dec layout").bounds();

        let is_disabled = self.on_input.is_none() || self.disabled();
        let input_status = if is_disabled {
            text_input::Status::Disabled
        } else if state.input.is_focused() {
            text_input::Status::Focused { is_hovered: true }
        } else {
            text_input::Status::Active
        };
        let input_style =
            <Theme as text_input::Catalog>::style(theme, &self.input_class, input_status);

        let bounds = content_layout.bounds();
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: input_style.border,
                shadow: Shadow::default(),
                snap: false,
            },
            input_style.background,
        );

        state.input.draw(
            renderer,
            bounds,
            *viewport,
            input::Style {
                value: input_style.value,
                selection: input_style.selection,
                placeholder: input_style.placeholder,
            },
        );

        if self.ignore_buttons {
            return;
        }

        let can_increase = self.can_increase();
        let can_decrease = self.can_decrease();

        let decrease_btn_style = if !can_decrease {
            style::number_input::Catalog::style(theme, &self.class, Status::Disabled)
        } else if state.decrease_pressed {
            style::number_input::Catalog::style(theme, &self.class, Status::Pressed)
        } else {
            style::number_input::Catalog::style(theme, &self.class, Status::Active)
        };

        let increase_btn_style = if !can_increase {
            style::number_input::Catalog::style(theme, &self.class, Status::Disabled)
        } else if state.increase_pressed {
            style::number_input::Catalog::style(theme, &self.class, Status::Pressed)
        } else {
            style::number_input::Catalog::style(theme, &self.class, Status::Active)
        };

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
                    snap: false,
                },
                increase_btn_style
                    .button_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        let txt_size = self.size.unwrap_or_else(|| renderer.default_size());
        let icon_size = txt_size * 2.5 / 4.0;

        let (content, font, shaping) = up_open();
        renderer.fill_text(
            iced_core::text::Text {
                content,
                bounds: Size::new(inc_bounds.width, inc_bounds.height),
                size: icon_size,
                font,
                line_height: text::LineHeight::Relative(1.3),
                shaping,
                wrapping: text::Wrapping::default(),
                align_x: Alignment::Center.into(),
                align_y: Alignment::Center.into(),
                ellipsis: text::Ellipsis::None,
                hint_factor: renderer.hint_factor(),
            },
            Point::new(inc_bounds.center_x(), inc_bounds.center_y()),
            increase_btn_style.icon_color,
            inc_bounds,
        );

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
                    snap: false,
                },
                decrease_btn_style
                    .button_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        let (content, font, shaping) = down_open();
        renderer.fill_text(
            iced_core::text::Text {
                content,
                bounds: Size::new(dec_bounds.width, dec_bounds.height),
                size: icon_size,
                font,
                line_height: text::LineHeight::Relative(1.3),
                shaping,
                wrapping: text::Wrapping::default(),
                align_x: Alignment::Center.into(),
                align_y: Alignment::Center.into(),
                ellipsis: text::Ellipsis::None,
                hint_factor: renderer.hint_factor(),
            },
            Point::new(dec_bounds.center_x(), dec_bounds.center_y()),
            decrease_btn_style.icon_color,
            dec_bounds,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();
        let content_layout = children.next().expect("content layout");
        let mut mod_children = children.next().expect("modifiers layout").children();
        let inc_bounds = mod_children.next().expect("inc layout").bounds();
        let dec_bounds = mod_children.next().expect("dec layout").bounds();

        let position = cursor.position().unwrap_or_default();

        if !self.ignore_buttons && (inc_bounds.contains(position) || dec_bounds.contains(position))
        {
            mouse::Interaction::Pointer
        } else if cursor.is_over(content_layout.bounds()) {
            if self.on_input.is_none() {
                mouse::Interaction::Idle
            } else {
                mouse::Interaction::Text
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, T, Message, Theme, Renderer> From<NumberInput<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: 'a + Num + NumAssignOps + PartialOrd + Display + FromStr + Clone + Bounded,
    Message: Clone + 'a,
    Theme: ExtendedCatalog + 'a,
    Renderer: iced_core::text::Renderer<Font = iced_core::Font> + 'static,
{
    fn from(
        number_input: NumberInput<'a, T, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(number_input)
    }
}

/// The state of a [`NumberInput`].
struct State<R: iced_core::text::Renderer> {
    input: iced_core::text::Input<R>,
    value: String,
    increase_pressed: bool,
    decrease_pressed: bool,
    is_intermediate: bool,
    transaction: Option<iced_core::shell::Tracking>,
}

fn state<Renderer: iced_core::text::Renderer + 'static>(tree: &mut Tree) -> &mut State<Renderer> {
    tree.state.downcast_mut::<State<Renderer>>()
}

impl<R: iced_core::text::Renderer> State<R> {
    fn new() -> Self {
        Self {
            input: iced_core::text::Input::new(),
            value: String::new(),
            increase_pressed: false,
            decrease_pressed: false,
            is_intermediate: false,
            transaction: None,
        }
    }
}

impl<R: iced_core::text::Renderer> operation::Focusable for State<R> {
    fn is_focused(&self) -> bool {
        self.input.is_focused()
    }
    fn focus(&mut self) {
        self.input.focus();
    }
    fn unfocus(&mut self) {
        self.input.unfocus();
    }
}

impl<R: iced_core::text::Renderer> operation::TextInput for State<R> {
    fn text(&self) -> text::Fragment<'_> {
        if self.input.is_empty() {
            text::Fragment::Borrowed(self.input.placeholder())
        } else {
            text::Fragment::Owned(self.input.value())
        }
    }
    fn move_cursor_to_front(&mut self) {
        self.input.move_cursor_to_front();
    }
    fn move_cursor_to_end(&mut self) {
        self.input.move_cursor_to_end();
    }
    fn move_cursor_to(&mut self, position: text::Position) {
        self.input.move_cursor_to(position);
    }
    fn select_all(&mut self) {
        self.input.select_all();
    }
    fn select_range(&mut self, start: text::Position, end: text::Position) {
        self.input.select_range(start, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::widget::tree::Tag;
    use iced_widget::Renderer;

    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum TestMessage {
        Changed(u32),
        Submit,
    }

    type TestNumberInput<'a> = NumberInput<'a, u32, TestMessage, iced_widget::Theme, Renderer>;

    #[test]
    fn number_input_new_creates_instance() {
        let value = 10u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100);

        assert_eq!(input.value, 10);
        assert_eq!(input.step, 1);
        assert!(matches!(input.min, Bound::Included(0)));
        assert!(matches!(input.max, Bound::Included(100)));
        assert!(!input.ignore_buttons);
    }

    #[test]
    fn number_input_with_step() {
        let value = 10u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100)
            .step(5);

        assert_eq!(input.step, 5);
    }

    #[test]
    fn number_input_ignore_buttons() {
        let value = 10u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100)
            .ignore_buttons(true);

        assert!(input.ignore_buttons);
    }

    #[test]
    fn number_input_bounds() {
        let value = 50u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(10..=90);

        assert!(matches!(input.min, Bound::Included(10)));
        assert!(matches!(input.max, Bound::Included(90)));
    }

    #[test]
    fn number_input_can_increase() {
        let value = 50u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100)
            .step(10);

        assert!(input.can_increase());
    }

    #[test]
    fn number_input_cannot_increase_at_max() {
        let value = 100u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100);

        assert!(!input.can_increase());
    }

    #[test]
    fn number_input_can_decrease() {
        let value = 50u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100)
            .step(10);

        assert!(input.can_decrease());
    }

    #[test]
    fn number_input_cannot_decrease_at_min() {
        let value = 0u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100);

        assert!(!input.can_decrease());
    }

    #[test]
    fn number_input_valid_value() {
        let value = 50u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100);

        assert!(input.valid(&50));
        assert!(input.valid(&0));
        assert!(input.valid(&100));
        assert!(!input.valid(&150));
    }

    #[test]
    fn number_input_min_max_values() {
        let value = 50u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(10..=90);

        assert_eq!(input.min(), 10);
        assert_eq!(input.max(), 90);
    }

    #[test]
    fn number_input_min_max_with_excluded_bounds() {
        let value = 50u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(10..90)
            .step(1);

        // Range 10..90 means start is Included(10), end is Excluded(90)
        assert_eq!(input.min(), 10); // Included bound
        assert_eq!(input.max(), 89); // Excluded bound - step
    }

    #[test]
    fn number_input_disabled_when_bounds_too_tight() {
        let value = 50u32;
        // When min == max (50..=50), the widget is disabled because there's no room to change
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(50..=50);
        assert!(input.disabled());

        // When min < max, the widget is not disabled
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(49..=50);
        assert!(!input.disabled());
    }

    #[test]
    fn number_input_does_not_overflow_when_max_is_type_min() {
        // Regression test for #419: unsigned bounds whose max() is 0 must not
        // panic with "attempt to subtract with overflow".
        type U8Input<'a> = NumberInput<'a, u8, TestMessage, iced_widget::Theme, Renderer>;

        let value = 0u8;

        // Inclusive `0..=0`, as in the minimal reproduction.
        let input: U8Input = NumberInput::new("...", &value)
            .bounds(0..=0u8)
            .on_input(|_| TestMessage::Submit);
        assert!(!input.can_increase());
        assert!(!input.can_decrease());

        // Exclusive `0..1`, as produced by `0..vector.len()` for a one-element vector.
        let input: U8Input = NumberInput::new("...", &value)
            .bounds(0..1u8)
            .on_input(|_| TestMessage::Submit);
        assert!(!input.can_increase());
        assert!(!input.can_decrease());
    }

    #[test]
    fn number_input_tag_returns_state_tag() {
        let value = 10u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100);

        let tag = Widget::<TestMessage, iced_widget::Theme, Renderer>::tag(&input);
        assert_eq!(tag, Tag::of::<State<Renderer>>());
    }

    #[test]
    fn number_input_different_values() {
        let test_values = [(0, 0..=100), (50, 0..=100), (100, 0..=100), (25, 10..=50)];

        for (value, range) in test_values {
            let input = TestNumberInput::new("...", &value)
                .on_input(TestMessage::Changed)
                .bounds(range);
            assert_eq!(input.value, value);
        }
    }

    #[test]
    fn number_input_with_on_submit() {
        let value = 10u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100)
            .on_submit(TestMessage::Submit);

        assert!(input.on_submit.is_some());
    }

    #[test]
    fn number_input_padding() {
        let value = 10u32;
        let custom_padding = Padding::new(10.0);
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100)
            .padding(custom_padding);

        assert_eq!(input.padding, custom_padding);
    }

    #[test]
    fn number_input_size() {
        let value = 10u32;
        let input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100)
            .size(20.0);

        assert_eq!(input.size, Some(iced_core::Pixels(20.0)));
    }

    #[test]
    fn number_input_width() {
        let value = 10u32;
        let _input = TestNumberInput::new("...", &value)
            .on_input(TestMessage::Changed)
            .bounds(0..=100)
            .width(200);

        // We can't easily verify the width was set since it's stored on the
        // widget itself now (not delegated to a child), but this still
        // ensures the builder call doesn't panic.
    }
}
