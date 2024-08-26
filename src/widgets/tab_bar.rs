//! Displays a [`TabBar`] to select the content to be displayed.
//!
//! You have to manage the logic to show the contend by yourself or you may want
//! to use the [`Tabs`](super::tabs::Tabs) widget instead.
//!
//! *This API requires the following crate features to be activated: `tab_bar`*

pub mod tab_label;
use crate::core::icons::{bootstrap::icon_to_string, Bootstrap, BOOTSTRAP_FONT};

use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer,
        widget::Tree,
        Clipboard, Layout, Shell, Widget,
    },
    alignment::{self, Horizontal, Vertical},
    event,
    mouse::{self, Cursor},
    touch,
    widget::{
        text::{self, LineHeight},
        Column, Row, Text,
    },
    Alignment, Background, Border, Color, Element, Event, Font, Length, Padding, Pixels, Point,
    Rectangle, Shadow, Size,
};

use std::marker::PhantomData;

pub use crate::style::{
    tab_bar::{self, Catalog, Style},
    Status, StyleFn,
};
pub use tab_label::TabLabel;

/// The default icon size.
const DEFAULT_ICON_SIZE: f32 = 16.0;
/// The default text size.
const DEFAULT_TEXT_SIZE: f32 = 16.0;
/// The default size of the close icon.
const DEFAULT_CLOSE_SIZE: f32 = 16.0;
/// The default padding between the tabs.
const DEFAULT_PADDING: Padding = Padding::new(5.0);
/// The default spacing around the tabs.
const DEFAULT_SPACING: Pixels = Pixels::ZERO;

/// A tab bar to show tabs.
///
/// # Example
/// ```ignore
/// # use iced_aw::{TabLabel, TabBar};
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
///     TabSelected(TabId),
/// }
///
/// #[derive(PartialEq, Hash)]
/// enum TabId {
///    One,
///    Two,
///    Three,
/// }
///
/// let tab_bar = TabBar::new(
///     Message::TabSelected,
/// )
/// .push(TabId::One, TabLabel::Text(String::from("One")))
/// .push(TabId::Two, TabLabel::Text(String::from("Two")))
/// .push(TabId::Three, TabLabel::Text(String::from("Three")))
/// .set_active_tab(&TabId::One);
/// ```
#[allow(missing_debug_implementations)]
pub struct TabBar<'a, Message, TabId, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer,
    Theme: Catalog,
    TabId: Eq + Clone,
{
    /// The index of the currently active tab.
    active_tab: usize,
    /// The vector containing the labels of the tabs.
    tab_labels: Vec<TabLabel>,
    /// The vector containing the indices of the tabs.
    tab_indices: Vec<TabId>,
    /// The function that produces the message when a tab is selected.
    on_select: Box<dyn Fn(TabId) -> Message>,
    /// The function that produces the message when the close icon was pressed.
    on_close: Option<Box<dyn Fn(TabId) -> Message>>,
    /// The width of the [`TabBar`].
    width: Length,
    /// The width of the tabs of the [`TabBar`].
    tab_width: Length,
    /// The width of the [`TabBar`].
    height: Length,
    /// The maximum height of the [`TabBar`].
    max_height: f32,
    /// The icon size.
    icon_size: f32,
    /// The text size.
    text_size: f32,
    /// The size of the close icon.
    close_size: f32,
    /// The padding of the tabs of the [`TabBar`].
    padding: Padding,
    /// The spacing of the tabs of the [`TabBar`].
    spacing: Pixels,
    /// The optional icon font of the [`TabBar`].
    font: Option<Font>,
    /// The optional text font of the [`TabBar`].
    text_font: Option<Font>,
    /// The style of the [`TabBar`].
    class: <Theme as Catalog>::Class<'a>,
    /// Where the icon is placed relative to text
    position: Position,
    #[allow(clippy::missing_docs_in_private_items)]
    _renderer: PhantomData<Renderer>,
}

#[derive(Clone, Copy, Default)]
/// The [`Position`] of the icon relative to text, this enum is only relative if [`TabLabel::IconText`] is used.
pub enum Position {
    /// Icon is placed above of the text.
    Top,
    /// Icon is placed right of the text.
    Right,
    /// Icon is placed below of the text.
    Bottom,
    #[default]
    /// Icon is placed left of the text, the default.
    Left,
}

impl<'a, Message, TabId, Theme, Renderer> TabBar<'a, Message, TabId, Theme, Renderer>
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: Catalog,
    TabId: Eq + Clone,
{
    /// Creates a new [`TabBar`] with the index of the selected tab and a specified
    /// message which will be send when a tab is selected by the user.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn new<F>(on_select: F) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
    {
        Self::with_tab_labels(Vec::new(), on_select)
    }

    /// Similar to `new` but with a given Vector of the [`TabLabel`](crate::tab_bar::TabLabel)s.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * a vector containing the [`TabLabel`]s of the [`TabBar`].
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn with_tab_labels<F>(tab_labels: Vec<(TabId, TabLabel)>, on_select: F) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
    {
        Self {
            active_tab: 0,
            tab_indices: tab_labels.iter().map(|(id, _)| id.clone()).collect(),
            tab_labels: tab_labels.into_iter().map(|(_, label)| label).collect(),
            on_select: Box::new(on_select),
            on_close: None,
            width: Length::Fill,
            tab_width: Length::Fill,
            height: Length::Shrink,
            max_height: u32::MAX as f32,
            icon_size: DEFAULT_ICON_SIZE,
            text_size: DEFAULT_TEXT_SIZE,
            close_size: DEFAULT_CLOSE_SIZE,
            padding: DEFAULT_PADDING,
            spacing: DEFAULT_SPACING,
            font: None,
            text_font: None,
            class: <Theme as Catalog>::default(),
            position: Position::default(),
            _renderer: PhantomData,
        }
    }

    /// Sets the size of the close icon of the
    /// [`TabLabel`](crate::tab_bar::TabLabel)s of the [`TabBar`].
    #[must_use]
    pub fn close_size(mut self, close_size: f32) -> Self {
        self.close_size = close_size;
        self
    }

    /// Gets the id of the currently active tab on the [`TabBar`].
    #[must_use]
    pub fn get_active_tab_id(&self) -> Option<&TabId> {
        self.tab_indices.get(self.active_tab)
    }

    /// Gets the index of the currently active tab on the [`TabBar`].
    #[must_use]
    pub fn get_active_tab_idx(&self) -> usize {
        self.active_tab
    }

    /// Gets the width of the [`TabBar`].
    #[must_use]
    pub fn get_height(&self) -> Length {
        self.height
    }

    /// Gets the width of the [`TabBar`].
    #[must_use]
    pub fn get_width(&self) -> Length {
        self.width
    }

    /// Sets the height of the [`TabBar`].
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the font of the icons of the
    /// [`TabLabel`](crate::tab_bar::TabLabel)s of the [`TabBar`].
    #[must_use]
    pub fn icon_font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the icon size of the [`TabLabel`](crate::tab_bar::TabLabel)s of the [`TabBar`].
    #[must_use]
    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.icon_size = icon_size;
        self
    }

    /// Sets the maximum height of the [`TabBar`].
    #[must_use]
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }

    /// Sets the message that will be produced when the close icon of a tab
    /// on the [`TabBar`] is pressed.
    ///
    /// Setting this enables the drawing of a close icon on the tabs.
    #[must_use]
    pub fn on_close<F>(mut self, on_close: F) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
    {
        self.on_close = Some(Box::new(on_close));
        self
    }

    /// Sets the padding of the tabs of the [`TabBar`].
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Pushes a [`TabLabel`](crate::tab_bar::TabLabel) to the [`TabBar`].
    #[must_use]
    pub fn push(mut self, id: TabId, tab_label: TabLabel) -> Self {
        self.tab_labels.push(tab_label);
        self.tab_indices.push(id);
        self
    }

    /// Gets the amount of tabs on the [`TabBar`].
    #[must_use]
    pub fn size(&self) -> usize {
        self.tab_indices.len()
    }

    /// Sets the spacing between the tabs of the [`TabBar`].
    #[must_use]
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into();
        self
    }

    /// Sets the font of the text of the
    /// [`TabLabel`](crate::tab_bar::TabLabel)s of the [`TabBar`].
    #[must_use]
    pub fn text_font(mut self, text_font: Font) -> Self {
        self.text_font = Some(text_font);
        self
    }

    /// Sets the text size of the [`TabLabel`](crate::tab_bar::TabLabel)s of the [`TabBar`].
    #[must_use]
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = text_size;
        self
    }

    /// Sets the width of a tab on the [`TabBar`].
    #[must_use]
    pub fn tab_width(mut self, width: Length) -> Self {
        self.tab_width = width;
        self
    }

    /// Sets up the active tab on the [`TabBar`].
    #[must_use]
    pub fn set_active_tab(mut self, active_tab: &TabId) -> Self {
        self.active_tab = self
            .tab_indices
            .iter()
            .position(|id| id == active_tab)
            .map_or(0, |a| a);
        self
    }

    #[must_use]
    /// Sets the [`Position`] of the Icon next to Text, Only used in [`TabLabel::IconText`]
    pub fn set_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// Sets the style of the [`TabBar`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Sets the class of the input of the [`TabBar`].
    #[must_use]
    pub fn class(mut self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the width of the [`TabBar`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message, TabId, Theme, Renderer> Widget<Message, Theme, Renderer>
    for TabBar<'a, Message, TabId, Theme, Renderer>
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: Catalog + text::Catalog,
    TabId: Eq + Clone,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        fn layout_icon<Theme, Renderer>(
            icon: &char,
            size: f32,
            font: Option<Font>,
        ) -> Text<'_, Theme, Renderer>
        where
            Renderer: iced::advanced::text::Renderer,
            Renderer::Font: From<Font>,
            Theme: iced::widget::text::Catalog,
        {
            Text::<Theme, Renderer>::new(icon.to_string())
                .size(size)
                .font(font.unwrap_or_default())
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .shaping(iced::advanced::text::Shaping::Advanced)
                .width(Length::Shrink)
        }

        fn layout_text<Theme, Renderer>(
            text: &str,
            size: f32,
            font: Option<Font>,
        ) -> Text<'_, Theme, Renderer>
        where
            Renderer: iced::advanced::text::Renderer,
            Renderer::Font: From<Font>,
            Theme: iced::widget::text::Catalog,
        {
            Text::<Theme, Renderer>::new(text)
                .size(size)
                .font(font.unwrap_or_default())
                .align_x(alignment::Horizontal::Center)
                .shaping(text::Shaping::Advanced)
                .width(Length::Shrink)
        }

        let row = self
            .tab_labels
            .iter()
            .fold(Row::<Message, Theme, Renderer>::new(), |row, tab_label| {
                let mut label_row = Row::new()
                    .push(
                        match tab_label {
                            TabLabel::Icon(icon) => Column::new()
                                .align_x(Alignment::Center)
                                .push(layout_icon(icon, self.icon_size + 1.0, self.font)),

                            TabLabel::Text(text) => Column::new()
                                .padding(5.0)
                                .align_x(Alignment::Center)
                                .push(layout_text(text, self.text_size + 1.0, self.text_font)),

                            TabLabel::IconText(icon, text) => {
                                let mut column = Column::new().align_x(Alignment::Center);

                                match self.position {
                                    Position::Top => {
                                        column = column
                                            .push(layout_icon(
                                                icon,
                                                self.icon_size + 1.0,
                                                self.font,
                                            ))
                                            .push(layout_text(
                                                text,
                                                self.text_size + 1.0,
                                                self.text_font,
                                            ));
                                    }
                                    Position::Right => {
                                        column = column.push(
                                            Row::new()
                                                .align_y(Alignment::Center)
                                                .push(layout_text(
                                                    text,
                                                    self.text_size + 1.0,
                                                    self.text_font,
                                                ))
                                                .push(layout_icon(
                                                    icon,
                                                    self.icon_size + 1.0,
                                                    self.font,
                                                )),
                                        );
                                    }
                                    Position::Left => {
                                        column = column.push(
                                            Row::new()
                                                .align_y(Alignment::Center)
                                                .push(layout_icon(
                                                    icon,
                                                    self.icon_size + 1.0,
                                                    self.font,
                                                ))
                                                .push(layout_text(
                                                    text,
                                                    self.text_size + 1.0,
                                                    self.text_font,
                                                )),
                                        );
                                    }
                                    Position::Bottom => {
                                        column = column
                                            .push(layout_text(
                                                text,
                                                self.text_size + 1.0,
                                                self.text_font,
                                            ))
                                            .push(layout_icon(
                                                icon,
                                                self.icon_size + 1.0,
                                                self.font,
                                            ));
                                    }
                                }

                                column
                            }
                        }
                        .width(self.tab_width)
                        .height(self.height),
                    )
                    .align_y(Alignment::Center)
                    .padding(self.padding)
                    .width(self.tab_width);

                if self.on_close.is_some() {
                    label_row = label_row.push(
                        Row::new()
                            .width(Length::Fixed(self.close_size * 1.3 + 1.0))
                            .height(Length::Fixed(self.close_size * 1.3 + 1.0))
                            .align_y(Alignment::Center),
                    );
                }

                row.push(label_row)
            })
            .width(self.width)
            .height(self.height)
            .spacing(self.spacing)
            .align_y(Alignment::Center);

        let element: Element<Message, Theme, Renderer> = Element::new(row);
        let tab_tree = if let Some(child_tree) = tree.children.get_mut(0) {
            child_tree.diff(element.as_widget());
            child_tree
        } else {
            let child_tree = Tree::new(element.as_widget());
            tree.children.insert(0, child_tree);
            &mut tree.children[0]
        };

        element
            .as_widget()
            .layout(tab_tree, renderer, &limits.loose())
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor
                    .position()
                    .map_or(false, |pos| layout.bounds().contains(pos))
                {
                    let tabs_map: Vec<bool> = layout
                        .children()
                        .map(|layout| {
                            cursor
                                .position()
                                .map_or(false, |pos| layout.bounds().contains(pos))
                        })
                        .collect();

                    if let Some(new_selected) = tabs_map.iter().position(|b| *b) {
                        shell.publish(
                            self.on_close
                                .as_ref()
                                .filter(|_on_close| {
                                    let tab_layout = layout.children().nth(new_selected).expect("widgets: Layout should have a tab layout at the selected index");
                                    let cross_layout = tab_layout.children().nth(1).expect("widgets: Layout should have a close layout");

                                    cursor.position().map_or(false, |pos| cross_layout.bounds().contains(pos) )
                                })
                                .map_or_else(
                                    || (self.on_select)(self.tab_indices[new_selected].clone()),
                                    |on_close| (on_close)(self.tab_indices[new_selected].clone()),
                                ),
                        );
                        return event::Status::Captured;
                    }
                }
                event::Status::Ignored
            }
            _ => event::Status::Ignored,
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
        let children = layout.children();
        let mut mouse_interaction = mouse::Interaction::default();

        for layout in children {
            let is_mouse_over = cursor
                .position()
                .map_or(false, |pos| layout.bounds().contains(pos));
            let new_mouse_interaction = if is_mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            };

            if new_mouse_interaction > mouse_interaction {
                mouse_interaction = new_mouse_interaction;
            }
        }

        mouse_interaction
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let children = layout.children();
        let is_mouse_over = cursor.position().map_or(false, |pos| bounds.contains(pos));
        let style_sheet = if is_mouse_over {
            tab_bar::Catalog::style(theme, &self.class, Status::Hovered)
        } else {
            tab_bar::Catalog::style(theme, &self.class, Status::Disabled)
        };

        if bounds.intersects(viewport) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: (0.0).into(),
                        width: style_sheet.border_width,
                        color: style_sheet.border_color.unwrap_or(Color::TRANSPARENT),
                    },
                    shadow: Shadow::default(),
                },
                style_sheet
                    .background
                    .unwrap_or_else(|| Color::TRANSPARENT.into()),
            );
        }

        for ((i, tab), layout) in self.tab_labels.iter().enumerate().zip(children) {
            draw_tab(
                renderer,
                tab,
                layout,
                self.position,
                theme,
                &self.class,
                i == self.get_active_tab_idx(),
                cursor,
                (self.font.unwrap_or(BOOTSTRAP_FONT), self.icon_size),
                (self.text_font.unwrap_or_default(), self.text_size),
                self.close_size,
                viewport,
            );
        }
    }
}

/// Draws a tab.
#[allow(
    clippy::borrowed_box,
    clippy::too_many_lines,
    clippy::too_many_arguments
)]
fn draw_tab<Theme, Renderer>(
    renderer: &mut Renderer,
    tab: &TabLabel,
    layout: Layout<'_>,
    position: Position,
    theme: &Theme,
    class: &<Theme as Catalog>::Class<'_>,
    is_selected: bool,
    cursor: Cursor,
    icon_data: (Font, f32),
    text_data: (Font, f32),
    close_size: f32,
    viewport: &Rectangle,
) where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: Catalog + text::Catalog,
{
    fn icon_bound_rectangle(item: Option<Layout<'_>>) -> Rectangle {
        item.expect("Graphics: Layout should have an icons layout for an IconText")
            .bounds()
    }

    fn text_bound_rectangle(item: Option<Layout<'_>>) -> Rectangle {
        item.expect("Graphics: Layout should have an texts layout for an IconText")
            .bounds()
    }

    let bounds = layout.bounds();
    let is_mouse_over = cursor.position().map_or(false, |pos| bounds.contains(pos));

    let style = if is_mouse_over {
        tab_bar::Catalog::style(theme, class, Status::Hovered)
    } else if is_selected {
        tab_bar::Catalog::style(theme, class, Status::Active)
    } else {
        tab_bar::Catalog::style(theme, class, Status::Disabled)
    };

    let mut children = layout.children();
    let label_layout = children
        .next()
        .expect("Graphics: Layout should have a label layout");
    let mut label_layout_children = label_layout.children();

    if bounds.intersects(viewport) {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: (0.0).into(),
                    width: style.tab_label_border_width,
                    color: style.tab_label_border_color,
                },
                shadow: Shadow::default(),
            },
            style.tab_label_background,
        );
    }

    match tab {
        TabLabel::Icon(icon) => {
            let icon_bounds = icon_bound_rectangle(label_layout_children.next());

            renderer.fill_text(
                iced::advanced::text::Text {
                    content: icon.to_string(),
                    bounds: Size::new(icon_bounds.width, icon_bounds.height),
                    size: Pixels(icon_data.1),
                    font: icon_data.0,
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
                    line_height: LineHeight::Relative(1.3),
                    shaping: iced::advanced::text::Shaping::Advanced,
                },
                Point::new(icon_bounds.center_x(), icon_bounds.center_y()),
                style.icon_color,
                icon_bounds,
            );
        }

        TabLabel::Text(text) => {
            let text_bounds = text_bound_rectangle(label_layout_children.next());

            renderer.fill_text(
                iced::advanced::text::Text {
                    content: text.to_string(),
                    bounds: Size::new(text_bounds.width, text_bounds.height),
                    size: Pixels(text_data.1),
                    font: text_data.0,
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
                    line_height: LineHeight::Relative(1.3),
                    shaping: iced::advanced::text::Shaping::Advanced,
                },
                Point::new(text_bounds.center_x(), text_bounds.center_y()),
                style.text_color,
                text_bounds,
            );
        }
        TabLabel::IconText(icon, text) => {
            let icon_bounds: Rectangle;
            let text_bounds: Rectangle;

            match position {
                Position::Top => {
                    icon_bounds = icon_bound_rectangle(label_layout_children.next());
                    text_bounds = text_bound_rectangle(label_layout_children.next());
                }
                Position::Right => {
                    let mut row_childern = label_layout_children
                        .next()
                        .expect("Graphics: Right Layout should have have a row with one child")
                        .children();
                    text_bounds = text_bound_rectangle(row_childern.next());
                    icon_bounds = icon_bound_rectangle(row_childern.next());
                }
                Position::Left => {
                    let mut row_childern = label_layout_children
                        .next()
                        .expect("Graphics: Left Layout should have have a row with one child")
                        .children();
                    icon_bounds = icon_bound_rectangle(row_childern.next());
                    text_bounds = text_bound_rectangle(row_childern.next());
                }
                Position::Bottom => {
                    text_bounds = text_bound_rectangle(label_layout_children.next());
                    icon_bounds = icon_bound_rectangle(label_layout_children.next());
                }
            }

            renderer.fill_text(
                iced::advanced::text::Text {
                    content: icon.to_string(),
                    bounds: Size::new(icon_bounds.width, icon_bounds.height),
                    size: Pixels(icon_data.1),
                    font: icon_data.0,
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
                    line_height: LineHeight::Relative(1.3),
                    shaping: iced::advanced::text::Shaping::Advanced,
                },
                Point::new(icon_bounds.center_x(), icon_bounds.center_y()),
                style.icon_color,
                icon_bounds,
            );

            renderer.fill_text(
                iced::advanced::text::Text {
                    content: text.to_string(),
                    bounds: Size::new(text_bounds.width, text_bounds.height),
                    size: Pixels(text_data.1),
                    font: text_data.0,
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
                    line_height: LineHeight::Relative(1.3),
                    shaping: iced::advanced::text::Shaping::Advanced,
                },
                Point::new(text_bounds.center_x(), text_bounds.center_y()),
                style.text_color,
                text_bounds,
            );
        }
    };

    if let Some(cross_layout) = children.next() {
        let cross_bounds = cross_layout.bounds();
        let is_mouse_over_cross = cursor.is_over(cross_bounds);

        renderer.fill_text(
            iced::advanced::text::Text {
                content: icon_to_string(Bootstrap::X),
                bounds: Size::new(cross_bounds.width, cross_bounds.height),
                size: Pixels(close_size + if is_mouse_over_cross { 1.0 } else { 0.0 }),
                font: BOOTSTRAP_FONT,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: LineHeight::Relative(1.3),
                shaping: iced::advanced::text::Shaping::Advanced,
            },
            Point::new(cross_bounds.center_x(), cross_bounds.center_y()),
            style.text_color,
            cross_bounds,
        );

        if is_mouse_over_cross && cross_bounds.intersects(viewport) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: cross_bounds,
                    border: Border {
                        radius: style.icon_border_radius,
                        width: style.border_width,
                        color: style.border_color.unwrap_or(Color::TRANSPARENT),
                    },
                    shadow: Shadow::default(),
                },
                style
                    .icon_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }
    };
}

impl<'a, Message, TabId, Theme, Renderer> From<TabBar<'a, Message, TabId, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: 'a + Catalog + text::Catalog,
    Message: 'a,
    TabId: 'a + Eq + Clone,
{
    fn from(tab_bar: TabBar<'a, Message, TabId, Theme, Renderer>) -> Self {
        Element::new(tab_bar)
    }
}
