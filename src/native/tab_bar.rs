//! Displays a [`TabBar`](TabBar) to select the content to be displayed.
//! 
//! You have to manage the logic to show the contend by yourself or you may want
//! to use the [`Tabs`](super::tabs::Tabs) widget instead.
use iced_native::Element;
use std::hash::Hash;

use iced_native::{Align, Clipboard, Column, Event, Hasher, Layout, Point, Row, Text, Widget, column, layout, mouse, row, text};
use iced::{Font, Length};

pub mod tab_label;
pub use tab_label::TabLabel;

/// A tab bar to show tabs.
/// 
/// # Example
/// ```
/// # use iced_aw::{TabLabel};
/// # use iced_native::{renderer::Null};
/// #
/// # pub type TabBar<Message> = iced_aw::native::TabBar<Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     TabSelected(usize),
/// }
/// 
/// let active_tab = 0;
/// 
/// let tab_bar = TabBar::new(
///     active_tab,
///     Message::TabSelected,
/// )
/// .push(TabLabel::Text(String::from("One")))
/// .push(TabLabel::Text(String::from("Two")))
/// .push(TabLabel::Text(String::from("Three")));
/// ```
#[allow(missing_debug_implementations)]
pub struct TabBar<Message, Renderer: self::Renderer> {
    active_tab: usize,
    tab_labels: Vec<TabLabel>,
    on_select: Box<dyn Fn(usize) -> Message>,
    on_close: Option<Box<dyn Fn(usize) -> Message>>,
    width: Length,
    tab_width: Length,
    height: Length,
    max_height: u32,
    icon_size: u16,
    text_size: u16,
    close_size: u16,
    padding: u16,
    spacing: u16,
    icon_font: Option<Font>,
    text_font: Option<Font>,
    style: Renderer::Style,
}

impl<Message, Renderer> TabBar<Message, Renderer>
where
    Renderer: self::Renderer
{
    /// Creates a new [`TabBar`](TabBar) with the index of the selected tab and a
    /// specified message which will be send when a tab is selected by the user.
    pub fn new<F>(active_tab: usize, on_select: F) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        Self::width_tab_labels(active_tab, Vec::new(), on_select)
    }

    /// Similar to `new` but with a given Vector of the
    /// [`TabLabel`](tab_label::TabLabel)s.Align
    pub fn width_tab_labels<F>(
        active_tab: usize,
        tab_labels: Vec<TabLabel>,
        on_select: F
    ) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        TabBar {
            active_tab,
            tab_labels,
            on_select: Box::new(on_select),
            on_close: None,
            width: Length::Fill,
            tab_width: Length::Fill,
            height: Length::Shrink,
            max_height: u32::MAX,
            icon_size: <Renderer as self::Renderer>::DEFAULT_ICON_SIZE,
            text_size: <Renderer as self::Renderer>::DEFAULT_TEXT_SIZE,
            close_size: <Renderer as self::Renderer>::DEFAULT_CLOSE_SIZE,
            padding: <Renderer as self::Renderer>::DEFAULT_PADDING,
            spacing: <Renderer as self::Renderer>::DEFAULT_SPACING,
            icon_font: None,
            text_font: None,
            style: Renderer::Style::default(),
        }
    }

    /// Gets the index of the currently active tab on the [`TabBar`](TabBar).
    pub fn get_active_tab(&self) -> usize {
        self.active_tab
    }

    /// Sets the message that will be produced when the close icon of a tab
    /// on the [`TabBar`](TabBar) is pressed.
    /// 
    /// Setting this enables the drawing of a close icon on the tabs.
    pub fn on_close<F>(mut self, on_close: F) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        self.on_close = Some(Box::new(on_close));
        self
    }

    /// Sets the width of the [`TabBar`](TabBar).
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Gets the width of the [`TabBar`](TabBar).
    pub fn get_width(&self) -> Length {
        self.width
    }

    /// Sets the width of a tab on the [`TabBar`](TabBar).
    pub fn tab_width(mut self, width: Length) -> Self {
        self.tab_width = width;
        self
    }

    /// Sets the height of the [`TabBar`](TabBar).
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Gets the width of the [`TabBar`](TabBar).
    pub fn get_height(&self) -> Length {
        self.height
    }

    /// Sets the maximum height of the [`TabBar`](TabBar).
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    /// Sets the icon size of the [`TabLabel`](tab_label::TabLabel)s of
    /// the [`TabBar`](TabBar).
    pub fn icon_size(mut self, icon_size: u16) -> Self {
        self.icon_size = icon_size;
        self
    }

    /// Sets the text size of the [`TabLabel`](tab_label::TabLabel)s of the
    /// [`TabBar`](TabBar).
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = text_size;
        self
    }

    /// Sets the size of the close icon of the
    /// [`TabLabel`](tab_label::TabLabel)s of the [`TabBar`](TabBar).
    pub fn close_size(mut self, close_size: u16) -> Self {
        self.close_size = close_size;
        self
    }

    /// Sets the padding of the tabs of the [`TabBar`](TabBar).
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the spacing between the tabs of the [`TabBar`](TabBar).
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the font of the icons of the
    /// [`TabLabel`](tab_label::TabLabel)s of the [`TabBar`](TabBar).
    pub fn icon_font(mut self, icon_font: Font) -> Self {
        self.icon_font = Some(icon_font);
        self
    }

    /// Sets the font of the text of the
    /// [`TabLabel`](tab_label::TabLabel)s of the [`TabBar`](TabBar).
    pub fn text_font(mut self, text_font: Font) -> Self {
        self.text_font = Some(text_font);
        self
    }

    /// Sets the style of the [`TabBar`](TabBar).
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Pushes a [`TabLabel`](tab_label::TabLabel) to the [`TabBar`](TabBar).
    pub fn push<'a>(mut self, tab_label: TabLabel) -> Self {
        self.tab_labels.push(tab_label);
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for TabBar<Message, Renderer>
where
    Renderer: self::Renderer + column::Renderer + text::Renderer + row::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.tab_labels.iter()
            .fold(Row::<Message, Renderer>::new(), |row, tab_label| {
                let label = match tab_label {
                        TabLabel::Icon(_icon) => {
                            Column::new()
                                .align_items(Align::Center)
                                .push(
                                    Row::new()
                                        .width(Length::Units(self.icon_size))
                                        .height(Length::Units(self.icon_size))
                                )
                        },
                        TabLabel::Text(text) => {
                            Column::new()
                                .align_items(Align::Center)
                                .push(
                                    Text::new(text)
                                        .size(self.text_size)
                                        .width(self.tab_width)
                                )
                        },
                        TabLabel::IconText(_icon, text) => {
                            Column::new()
                                .align_items(Align::Center)
                                .push(
                                    Row::new()
                                        .width(Length::Units(self.icon_size))
                                        .height(Length::Units(self.icon_size))
                                )
                                .push(
                                    Text::new(text)
                                        .size(self.text_size)
                                        .width(self.tab_width)
                                )
                        },
                    }
                    .width(self.tab_width)
                    .height(self.height);

                let mut label_row = Row::new()
                    .align_items(Align::Center)
                    .padding(self.padding)
                    .width(self.tab_width)
                    .push(label);

                if self.on_close.is_some() {
                    label_row = label_row
                        .push(
                            Row::new()
                                .width(Length::Units(self.close_size))
                                .height(Length::Units(self.close_size))
                                .align_items(Align::Center)
                        );
                }

                row.push(label_row)
            })
            .width(self.width)
            .height(self.height)
            .spacing(self.spacing)
            .layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>
    ) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if layout.bounds().contains(cursor_position) {
                    let tabs_map: Vec<bool> = layout.children()
                        .map(|layout| layout.bounds().contains(cursor_position))
                        .collect();

                    if let Some(new_selected) = tabs_map.iter().position(|b| *b) {
                        messages.push(
                            self.on_close.as_ref().filter(|_on_close| {
                                let tab_layout = layout.children().nth(new_selected).unwrap();
                                let cross_layout = tab_layout.children().nth(1).unwrap();
    
                                cross_layout.bounds().contains(cursor_position)
                            })
                            .map(|on_close| (on_close)(new_selected))
                            .unwrap_or((self.on_select)(new_selected))
                        );
                    }
                }
            },
            _ => {}
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        self::Renderer::draw(
            renderer,
            defaults,
            self.active_tab,
            &self.tab_labels,
            layout,
            cursor_position,
            self.icon_font,
            self.text_font,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.active_tab.hash(state);
        self.tab_labels.iter().for_each(|tab| tab.hash(state));
        self.width.hash(state);
        self.height.hash(state);
        self.max_height.hash(state);
        self.icon_size.hash(state);
        self.text_size.hash(state);
        self.close_size.hash(state);
    }
}


/// The renderer of a [`TabBar`](TabBar).
/// 
/// Your renderer will need to implement this trait before being
/// able to use a [`TabBar`](TabBar) in your user interface.
pub trait Renderer: iced_native::Renderer {

    /// The style supported by this renderer.
    type Style: Default;

    /// The default icon size of a [`TabBar`](TabBar).
    const DEFAULT_ICON_SIZE: u16;

    /// The default text size of a [`TabBar`](TabBar).
    const DEFAULT_TEXT_SIZE: u16;

    /// The default close size of a [`TabBar`](TabBar).
    const DEFAULT_CLOSE_SIZE: u16;

    /// The default padding of a [`TabBar`](TabBar).
    const DEFAULT_PADDING: u16;

    /// The default spacing of a [`TabBar`](TabBar).
    const DEFAULT_SPACING: u16;

    /// Draws a [`TabBar`](TabBar).
    fn draw(
        &mut self,
        defaults: &Self::Defaults,
        active_tab: usize,
        tab_labels: &[TabLabel],
        layout: Layout<'_>,
        cursor_position: Point,
        icon_font: Option<Font>,
        text_font: Option<Font>,
        style_sheet: &Self::Style,
    ) -> Self::Output;
}

impl Renderer for iced_native::renderer::Null {
    type Style = ();

    const DEFAULT_ICON_SIZE: u16 = 0;

    const DEFAULT_TEXT_SIZE: u16 = 0;

    const DEFAULT_CLOSE_SIZE: u16 = 0;

    const DEFAULT_PADDING: u16 = 0;

    const DEFAULT_SPACING: u16 = 0;

    fn draw(
        &mut self,
        _defaults: &Self::Defaults,
        _active_tab: usize,
        _tab_labels: &[TabLabel],
        _layout: Layout<'_>,
        _cursor_position: Point,
        _icon_font: Option<Font>,
        _text_font: Option<Font>,
        _style_sheet: &Self::Style,
    ) -> Self::Output {}
}


impl<'a, Message, Renderer> From<TabBar<Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + column::Renderer + text::Renderer
        + row::Renderer,
    Message: 'a,
{
    fn from(tab_bar: TabBar<Message, Renderer>) -> Self {
        Element::new(tab_bar)
    }
}