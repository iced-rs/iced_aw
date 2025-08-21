//! This module provides a [`LabeledFrame`] widget which allows you to draw a border with a title around some content.

use iced::advanced;
use iced::Pixels;
use iced::Rectangle;

/// The style of a [`LabeledFrame`]
pub struct Style {
    /// The color of the border/frame
    pub color: iced::Background,
    /// The border radius of the border/frame
    pub radius: iced::border::Radius,
}

/// The theme catalog of a [`LabeledFrame`]
pub trait Catalog {
    /// The item class of [`Catalog`]
    type Class<'a>;
    /// The default class produced by the [`Catalog`]
    fn default<'a>() -> Self::Class<'a>;
    /// The [`Style`] of a class
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// A styling function for a [`LabeledFrame`]
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for iced::Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme| Style {
            color: iced::Background::Color(theme.extended_palette().secondary.weak.color),
            radius: iced::border::Radius::default(),
        })
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        (class)(self)
    }
}

/// A labeled frame widget
pub struct LabeledFrame<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
{
    title: iced::Element<'a, Message, Theme, Renderer>,
    content: iced::Element<'a, Message, Theme, Renderer>,
    width: iced::Length,
    height: iced::Length,
    class: Theme::Class<'a>,
    inset: f32,
    outset: f32,
    stroke_width: f32,
    horizontal_title_padding: f32,
}

impl<'a, Message, Theme, Renderer> LabeledFrame<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
{
    /// Creates a new [`LabeledFrame`]
    pub fn new(
        title: impl Into<iced::Element<'a, Message, Theme, Renderer>>,
        content: impl Into<iced::Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            width: iced::Shrink,
            height: iced::Shrink,
            class: Theme::default(),
            inset: 5.0,
            outset: 5.0,
            stroke_width: 3.0,
            horizontal_title_padding: 5.0,
        }
    }

    /// Sets the width of the [`LabeledFrame`]
    #[must_use]
    pub fn width(mut self, width: impl Into<iced::Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`LabeledFrame`]
    #[must_use]
    pub fn height(mut self, height: impl Into<iced::Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the inset that is between the border and the inner content
    #[must_use]
    pub fn inset(mut self, inset: impl Into<Pixels>) -> Self {
        self.inset = inset.into().0;
        self
    }

    /// Sets the outset that is put around the border
    #[must_use]
    pub fn outset(mut self, outset: impl Into<Pixels>) -> Self {
        self.outset = outset.into().0;
        self
    }

    /// Sets the width of the stroke/border drawn around the content
    #[must_use]
    pub fn stroke_width(mut self, stroke_width: impl Into<Pixels>) -> Self {
        self.stroke_width = stroke_width.into().0;
        self
    }

    /// Sets the padding that the title gets on the horizontal axis
    #[must_use]
    pub fn horizontal_title_padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.horizontal_title_padding = padding.into().0;
        self
    }

    /// Changes the style of the [`LabeledFrame`]
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }
}

impl<Message, Theme, Renderer> advanced::Widget<Message, Theme, Renderer>
    for LabeledFrame<'_, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer,
    Theme: Catalog,
{
    fn size(&self) -> iced::Size<iced::Length> {
        iced::Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        tree: &mut advanced::widget::Tree,
        renderer: &Renderer,
        limits: &advanced::layout::Limits,
    ) -> advanced::layout::Node {
        let limits = (*limits).height(self.height).width(self.width).loose();
        let title_layout = self
            .title
            .as_widget_mut()
            .layout(
                &mut tree.children[0],
                renderer,
                &limits.shrink(
                    iced::Padding::default()
                        .left(
                            (self.outset + self.inset + self.stroke_width) * 2.0
                                + self.horizontal_title_padding,
                        )
                        .right(
                            (self.outset + self.inset + self.stroke_width) * 2.0
                                + self.horizontal_title_padding,
                        ),
                ),
            )
            .translate([
                (self.outset + self.inset + self.stroke_width) * 2.0
                    + self.horizontal_title_padding,
                0.0,
            ]);
        let content_layout = self
            .content
            .as_widget_mut()
            .layout(
                &mut tree.children[1],
                renderer,
                &limits.shrink(
                    iced::Padding::default()
                        .left(self.outset + self.inset + self.stroke_width)
                        .right(self.outset + self.inset + self.stroke_width)
                        .bottom(self.outset + self.inset + self.stroke_width)
                        .top(
                            (self.outset + self.inset + self.stroke_width)
                                .max(title_layout.size().height),
                        ),
                ),
            )
            .translate([
                self.outset + self.inset + self.stroke_width,
                (self.outset + self.inset + self.stroke_width).max(title_layout.bounds().height),
            ]);

        advanced::layout::Node::with_children(
            limits.resolve(
                self.width,
                self.height,
                iced::Size::new(
                    (content_layout.size().width
                        + (self.outset + self.inset + self.stroke_width) * 2.0)
                        .max(
                            title_layout.size().width
                                + (self.outset + self.inset + self.stroke_width) * 4.0,
                        ),
                    content_layout.bounds().y
                        + content_layout.size().height
                        + (self.outset + self.inset + self.stroke_width),
                ),
            ),
            vec![title_layout, content_layout],
        )
    }

    fn children(&self) -> Vec<advanced::widget::Tree> {
        vec![
            advanced::widget::Tree::new(&self.title),
            advanced::widget::Tree::new(&self.content),
        ]
    }

    fn diff(&mut self, tree: &mut advanced::widget::Tree) {
        tree.diff_children(&mut [&mut self.title, &mut self.content]);
    }

    fn draw(
        &self,
        tree: &advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &advanced::renderer::Style,
        layout: advanced::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        [&self.title, &self.content]
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .for_each(|((child, state), layout)| {
                child
                    .as_widget()
                    .draw(state, renderer, theme, style, layout, cursor, viewport);
            });
        let style = theme.style(&self.class);
        let title_layout = layout.children().next().expect("missing title layout");
        let top_line_y =
            title_layout.position().y + (title_layout.bounds().height - self.stroke_width) / 2.0;
        // left line
        renderer.fill_quad(
            advanced::renderer::Quad {
                bounds: iced::Rectangle {
                    x: layout.position().x + self.outset,
                    y: top_line_y,
                    width: self.stroke_width,
                    height: layout.bounds().height
                        - self.outset
                        - (top_line_y - layout.position().y),
                },
                border: iced::Border {
                    radius: iced::border::Radius::default()
                        .top_left(style.radius.top_left)
                        .bottom_left(style.radius.bottom_left),
                    ..Default::default()
                },
                shadow: iced::Shadow::default(),
                ..Default::default()
            },
            style.color,
        );
        // right line
        renderer.fill_quad(
            advanced::renderer::Quad {
                bounds: iced::Rectangle {
                    x: layout.position().x + layout.bounds().width
                        - self.outset
                        - self.stroke_width,
                    y: top_line_y,
                    width: self.stroke_width,
                    height: layout.bounds().height
                        - self.outset
                        - (top_line_y - layout.position().y),
                },
                border: iced::Border {
                    radius: iced::border::Radius::default()
                        .top_right(style.radius.top_right)
                        .bottom_right(style.radius.bottom_right),
                    ..Default::default()
                },
                shadow: iced::Shadow::default(),
                ..Default::default()
            },
            style.color,
        );
        // bottom line
        renderer.fill_quad(
            advanced::renderer::Quad {
                bounds: iced::Rectangle {
                    x: layout.position().x + self.outset,
                    y: layout.position().y + layout.bounds().height
                        - self.outset
                        - self.stroke_width,
                    width: layout.bounds().width - self.outset * 2.0,
                    height: self.stroke_width,
                },
                border: iced::Border {
                    radius: iced::border::Radius::default()
                        .bottom_right(style.radius.top_right)
                        .bottom_left(style.radius.top_left),
                    ..Default::default()
                },
                shadow: iced::Shadow::default(),
                ..Default::default()
            },
            style.color,
        );
        // top line left
        renderer.fill_quad(
            advanced::renderer::Quad {
                bounds: iced::Rectangle {
                    x: layout.position().x + self.outset,
                    y: top_line_y,
                    width: title_layout.position().x
                        - (layout.position().x + self.outset)
                        - self.horizontal_title_padding,
                    height: self.stroke_width,
                },
                border: iced::Border {
                    radius: iced::border::Radius::default().top_left(style.radius.top_left),
                    ..Default::default()
                },
                shadow: iced::Shadow::default(),
                ..Default::default()
            },
            style.color,
        );
        // top line right
        renderer.fill_quad(
            advanced::renderer::Quad {
                bounds: iced::Rectangle {
                    x: title_layout.position().x
                        + title_layout.bounds().width
                        + self.horizontal_title_padding,
                    y: top_line_y,
                    width: (layout.position().x + layout.bounds().width - self.outset)
                        - (title_layout.position().x + title_layout.bounds().width)
                        - self.horizontal_title_padding,
                    height: self.stroke_width,
                },
                border: iced::Border {
                    radius: iced::border::Radius::default().top_right(style.radius.top_right),
                    ..Default::default()
                },
                shadow: iced::Shadow::default(),
                ..Default::default()
            },
            style.color,
        );
    }

    fn mouse_interaction(
        &self,
        state: &advanced::widget::Tree,
        layout: advanced::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> advanced::mouse::Interaction {
        [&self.title, &self.content]
            .iter()
            .zip(&state.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
    }

    fn update(
        &mut self,
        state: &mut advanced::widget::Tree,
        event: &iced::Event,
        layout: advanced::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn advanced::Clipboard,
        shell: &mut advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        // STK: clean up if working
        // let status = [&mut self.title, &mut self.content]
        //     .iter_mut()
        //     .zip(&mut state.children)
        //     .zip(layout.children())
        //     .map(|((child, state), layout)| {
        //         child.as_widget_mut().update(
        //             state, event, layout, cursor, renderer, clipboard, shell, viewport,
        //         )
        //     })
        //     .fold(iced::event::Status::Ignored, iced::event::Status::merge);
        // if status == iced::event::Status::Captured {
        //     shell.capture_event();
        // }
        for ((child, state), layout) in [&mut self.title, &mut self.content]
            .iter_mut()
            .zip(&mut state.children)
            .zip(layout.children())
        {
            child.as_widget_mut().update(
                state, event, layout, cursor, renderer, clipboard, shell, viewport,
            );
        }
    }

    fn operate(
        &mut self,
        state: &mut advanced::widget::Tree,
        layout: advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn advanced::widget::Operation,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            [&mut self.title, &mut self.content]
                .iter_mut()
                .zip(&mut state.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget_mut()
                        .operate(state, layout, renderer, operation);
                });
        });
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut advanced::widget::Tree,
        layout: advanced::Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: iced::Vector,
    ) -> Option<advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        let children = vec![&mut self.title, &mut self.content]
            .into_iter()
            .zip(&mut state.children)
            .zip(layout.children())
            .filter_map(|((child, state), layout)| {
                child
                    .as_widget_mut()
                    .overlay(state, layout, renderer, viewport, translation)
            })
            .collect::<Vec<_>>();

        (!children.is_empty()).then(|| advanced::overlay::Group::with_children(children).overlay())
    }

    fn size_hint(&self) -> iced::Size<iced::Length> {
        self.size()
    }
}

impl<'a, Message, Theme, Renderer> From<LabeledFrame<'a, Message, Theme, Renderer>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: advanced::Renderer + 'a,
{
    fn from(value: LabeledFrame<'a, Message, Theme, Renderer>) -> Self {
        iced::Element::new(value)
    }
}
