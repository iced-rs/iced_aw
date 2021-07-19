//! Displays a [`TabBar`](TabBar) to select the content to be displayed.
//!
//! You have to manage the logic to show the contend by yourself or you may want
//! to use the [`Tabs`](super::tabs::Tabs) widget instead.
//!
//! *This API requires the following crate features to be activated: `tab_bar`*
use dodrio::{bumpalo, Node};
use iced_web::{css, Bus, Css, Element, Length, Widget};

pub mod tab_label;
pub use crate::style::tab_bar::{Style, StyleSheet};
pub use tab_label::TabLabel;

use std::rc::Rc;

const _DEFAULT_ICON_SIZE: u16 = 32;

const DEFAULT_TEXT_SIZE: u16 = 16;

const DEFAULT_CLOSE_SIZE: u16 = 16;

const DEFAULT_PADDING: u16 = 5;

const DEFAULT_SPACING: u16 = 0;

/// A tab bar to show tabs.
///
/// # Example
/// ```
/// # use iced_aw::{TabBar, tab_bar::TabLabel};
/// enum Message {
///     TabSelected(usize),
/// }
///
/// let active_tab = 0;
/// let tab_bar = TabBar::new(
///     active_tab,
///     Message::TabSelected,   
/// )
/// .push(TabLabel::Text(String::from("One")))
/// .push(TabLabel::Text(String::from("Two")))
/// .push(TabLabel::Text(String::from("Three")));
/// ```
#[allow(missing_debug_implementations)]
pub struct TabBar<Message> {
    /// The currently active tab.
    active_tab: usize,
    /// The vector containing the labels of the tabs.
    tab_labels: Vec<TabLabel>,
    /// The function that produces the message when a tab is selected.
    on_select: Rc<dyn Fn(usize) -> Message>,
    /// The function that produces the message when the close icon was pressed.
    on_close: Option<Rc<dyn Fn(usize) -> Message>>,
    /// The width of the [`TabBar`](TabBar).
    width: Length,
    /// The width of the tabs of the [`TabBar`](TabBar).
    tab_width: Length,
    /// The width of the [`TabBar`](TabBar).
    height: Length,
    /// The maximum height of the [`TabBar`](TabBar).
    max_height: u32,
    /// The icon size.
    text_size: u16,
    /// The size of the close icon.
    close_size: u16,
    /// The padding of the tabs of the [`TabBar`](TabBar).
    padding: u16,
    /// The spacing of the tabs of the [`TabBar`](TabBar).
    spacing: u16,
    /// The style of the [`TabBar`](TabBar).
    style: Box<dyn StyleSheet>,
}

impl<Message> TabBar<Message> {
    /// Creates a new [`TabBar`](TabBar) with the index of the selected tab and a
    /// specified message which will be send when a tab is selected by the user.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn new<F>(active_tab: usize, on_select: F) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        Self::width_tab_labels(active_tab, Vec::new(), on_select)
    }

    /// Similar to `new` but with a given Vector of the
    /// [`TabLabel`](tab_label::TabLabel)s.Align
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * a vector containing the [`TabLabel`](TabLabel)s of the [`TabBar`](TabBar).
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn width_tab_labels<F>(active_tab: usize, tab_labels: Vec<TabLabel>, on_select: F) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        Self {
            active_tab,
            tab_labels,
            on_select: Rc::new(on_select),
            on_close: None,
            width: Length::Fill,
            tab_width: Length::Fill,
            height: Length::Shrink,
            max_height: u32::MAX,
            text_size: DEFAULT_TEXT_SIZE,
            close_size: DEFAULT_CLOSE_SIZE,
            padding: DEFAULT_PADDING,
            spacing: DEFAULT_SPACING,
            style: Default::default(),
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
        self.on_close = Some(Rc::new(on_close));
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

    /// Sets the style of the [`TabBar`](TabBar).
    pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style = style.into();
        self
    }

    /// Pushes a [`TabLabel`](tab_label::TabLabel) to the [`TabBar`](TabBar).
    pub fn push(mut self, tab_label: TabLabel) -> Self {
        self.tab_labels.push(tab_label);
        self
    }
}

impl<Message> Widget<Message> for TabBar<Message>
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

        // TODO: State-based styling
        // (https://github.com/hecrj/iced/blob/master/web/src/widget/button.rs#L144)
        let style = self.style.active(false);
        let active_style = self.style.active(true);

        let tabs: Vec<Node> = self.tab_labels.iter().enumerate().map(|(i, tab)| {
            let label = match tab {
                TabLabel::Text(t) => {
                    use dodrio::bumpalo::collections::String;
                    text(String::from_str_in(t, bump).into_bump_str())
                },
            };

            let style = if self.active_tab == i {
                active_style
            } else {
                style
            };

            let close = self.on_close.clone().map(|on_close| {
                use dodrio::bumpalo::collections::String;
                let event_bus = bus.clone();
                div(bump)
                    .attr(
                        "style",
                        bumpalo::format!(
                            in bump,
                            "font-size: {}px; width: auto; height: auto; \
                            text-align: center; color: {}; cursor: pointer; \
                            display: flex; align-items: center;",
                            self.close_size,
                            css::color(style.icon_color)
                        ).into_bump_str(),
                    )
                    .on("click", move |_root, _vdom, _event| {
                        event_bus.publish((on_close)(i))
                    })
                    .child(text(String::from_str_in(r#"×"#, bump).into_bump_str()))
                    .finish()
            });

            let tab_label_class = style_sheet.insert(bump, css::Rule::Row);

            let background = match style.tab_label_background {
                iced_style::Background::Color(color) => css::color(color),
            };

            let mut node = div(bump)
                .attr(
                    "class",
                    bumpalo::format!(in bump, "{}", tab_label_class)
                        .into_bump_str()
                )
                .attr(
                    "style",
                    bumpalo::format!(
                        in bump,
                        "background: {}; border-style: solid; border-color: {}; border-width: {}px; \
                        color: {}; padding: {}px; font-size: {}px; cursor: pointer; user-select: none; \
                        width: {};",
                        background,
                        css::color(style.tab_label_border_color),
                        style.tab_label_border_width,
                        css::color(style.text_color),
                        self.padding,
                        self.text_size,
                        css::length(self.tab_width)
                    ).into_bump_str(),
                ).child(label);

            if let Some(close) = close {
                node = node.child(close);
            }

            let event_bus = bus.clone();
            let on_select = self.on_select.clone();
            node = node.on("click", move |_root, _vdom, _event| {
                event_bus.publish((on_select)(i));
            });

            node.finish()
        })
        .collect();

        let tab_bar_class = style_sheet.insert(bump, css::Rule::Row);

        let spacing_class = style_sheet.insert(bump, css::Rule::Spacing(self.spacing));

        let background = match style.background {
            None => String::from("none"),
            Some(background) => match background {
                iced_style::Background::Color(color) => css::color(color),
            },
        };

        let node = div(bump)
            .attr(
                "class",
                bumpalo::format!(in bump, "{} {}", tab_bar_class, spacing_class).into_bump_str(),
            )
            .attr(
                "style",
                bumpalo::format!(
                    in bump,
                    "background: {}; border-style: solid; border-color: {}; border-width: {}px; \
                    width: {}; height: {}; maximum-height: {}px",
                    background,
                    style.border_color.map_or_else(
                        || String::from("none"),
                        |color| css::color(color),
                    ),
                    style.border_width,
                    css::length(self.width),
                    css::length(self.height),
                    self.max_height
                )
                .into_bump_str(),
            )
            .children(tabs);

        node.finish()
    }
}

impl<'a, Message> From<TabBar<Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(tab_bar: TabBar<Message>) -> Self {
        Element::new(tab_bar)
    }
}
