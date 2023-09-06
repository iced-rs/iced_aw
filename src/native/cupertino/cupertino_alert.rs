#![allow(clippy::todo)]

use iced_widget::{
    core::{
        self,
        alignment::Horizontal,
        event, keyboard,
        layout::{Limits, Node},
        mouse::{self, Cursor},
        renderer::{self, Quad},
        touch,
        widget::Tree,
        Background, Clipboard, Color, Element, Event, Font, Layout, Length, Point, Rectangle,
        Shell, Size, Widget,
    },
    style::application,
    text, Text,
};

use std::ops::Range;

// INTERNAL //
use crate::graphics::SF_UI_ROUNDED;
use crate::native::cupertino::cupertino_colours::secondary_system_fill;
//

/**
 * `CupertinoDialogAction`
 *
 * See
 *
 * <https://github.com/flutter/flutter/blob/master/packages/flutter/lib/src/cupertino/dialog.dart>
 *
 * for constants, and
 *
 * <https://api.flutter.dev/flutter/cupertino/CupertinoAlertDialog-class.html>
 *
 * for the Flutter example / expected usage.
 *
 */
#[allow(missing_debug_implementations)]
pub struct CupertinoDialogAction<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer + core::text::Renderer + 'a,
    Renderer::Theme: application::StyleSheet + text::StyleSheet,
{
    on_pressed: Option<Message>,

    /// The content to show in the dialog box (typically a button)
    child: Element<'a, Message, Renderer>,

    /// Use `is_enabled` to provide logic for making modal action buttons enabled/disabled.
    /// Defaults to `true`
    is_enabled: bool,
}

impl<'a, Message, Renderer> Default for CupertinoDialogAction<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer + core::text::Renderer + 'a,
    Renderer::Theme: application::StyleSheet + text::StyleSheet,
{
    fn default() -> Self {
        Self {
            is_enabled: true,
            on_pressed: None,
            child: Text::new("Example").into(),
        }
    }
}

impl<'a, Message, Renderer> CupertinoDialogAction<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer + core::text::Renderer + 'a,
    Renderer::Theme: application::StyleSheet + text::StyleSheet,
{
    /// Creates a new [`CupertinoDialogAction`] widget.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `child` for the [`CupertinoDialogAction`].
    #[must_use]
    pub fn child(mut self, child: Element<'a, Message, Renderer>) -> Self {
        self.child = child;
        self
    }

    /// Sets `is_enabled` for the [`CupertinoDialogAction`].
    #[must_use]
    pub fn is_enabled(mut self, is_enabled: bool) -> Self {
        self.is_enabled = is_enabled;
        self
    }

    /// Sets `on_pressed` for the [`CupertinoDialogAction`].
    #[must_use]
    pub fn on_pressed(mut self, on_pressed: Option<Message>) -> Self {
        self.on_pressed = on_pressed;
        self
    }
}

/**
 * `CupertinoAlert`
 *
 * See both
 *
 * 1. <https://api.flutter.dev/flutter/cupertino/CupertinoAlertDialog-class.html>
 * 2. <https://api.flutter.dev/flutter/cupertino/CupertinoDialogAction-class.html>
 *
 * as sources for API and behavior. The Iced AW modal (`src/native/modal.rs`) is also a source for
 * the design of this struct.
 *
 * The example under `examples/cupertino/cupertino_alert/` shows how to work with the
 * `CupertinoAlert`.
 *
 * Design and Default Behaviour
 * ----------------------------
 * 1. By default: clicking the "Cancel" or "Confirm" buttons causes the modal to close
 * 2. By default: pressing the escape key causes the modal to close
 * 3. By default: clicking anywhere in the backdrop causes the modal to close
 * 4. The modal assumes the actions are of length 2, with the "Confirm" action coming first, and
 *    the "Cancel" action coming second.
 *
 */
#[allow(missing_debug_implementations)]
pub struct CupertinoAlert<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer + core::text::Renderer<Font = Font> + 'a,
    Renderer::Theme: application::StyleSheet + text::StyleSheet,
{
    width: Length,
    height: Length,
    is_hidden: bool,
    title: String,
    content: String,

    /// Dialog actions (confirm, cancel, etc.)
    actions: Vec<CupertinoDialogAction<'a, Message, Renderer>>,

    /// The optional message that will be sent when the user clicks on the backdrop.
    backdrop: Option<Message>,

    /// The optional message that will be sent when the ESC key is pressed.
    on_escape: Option<Message>,
}

impl<'a, Message, Renderer> Default for CupertinoAlert<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: core::Renderer + core::text::Renderer<Font = Font> + 'a,
    Renderer::Theme: application::StyleSheet + text::StyleSheet,
{
    fn default() -> Self {
        Self {
            width: Length::Fixed(400.0),
            height: Length::Fixed(200.0),
            is_hidden: true,
            title: "Title".to_owned(),
            content: "Content".to_owned(),
            actions: vec![],
            backdrop: None,
            on_escape: None,
        }
    }
}

impl<'a, Message, Renderer> CupertinoAlert<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: core::Renderer + core::text::Renderer<Font = Font> + 'a,
    Renderer::Theme: application::StyleSheet + text::StyleSheet,
{
    /// Creates a new [`CupertinoAlert`] widget.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `width` of the [`CupertinoAlert`].
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the `height` of the [`CupertinoAlert`].
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets `is_hidden` for the [`CupertinoAlert`].
    #[must_use]
    pub fn is_hidden(mut self, is_hidden: bool) -> Self {
        self.is_hidden = is_hidden;
        self
    }

    /// Sets the `title` of the [`CupertinoAlert`].
    #[must_use]
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Sets the `content` of the [`CupertinoAlert`].
    #[must_use]
    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    /// Sets the `actions` of the [`CupertinoAlert`].
    #[must_use]
    pub fn actions(mut self, actions: Vec<CupertinoDialogAction<'a, Message, Renderer>>) -> Self {
        self.actions = actions;
        self
    }

    /// Sets the `backdrop` of the [`CupertinoAlert`].
    #[must_use]
    pub fn backdrop(mut self, backdrop: Option<Message>) -> Self {
        self.backdrop = backdrop;
        self
    }

    /// Sets `on_escape` for the [`CupertinoAlert`].
    #[must_use]
    pub fn on_escape(mut self, on_escape: Option<Message>) -> Self {
        self.on_escape = on_escape;
        self
    }

    // Internal //
    fn _text_with_font<T>(element: T) -> Element<'a, Message, Renderer>
    where
        T: Into<Text<'a, Renderer>>,
    {
        let as_text_element = element.into().font(SF_UI_ROUNDED);

        Element::from(as_text_element)
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for CupertinoAlert<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: core::Renderer + core::text::Renderer<Font = Font> + 'a,
    Renderer::Theme: application::StyleSheet + text::StyleSheet,
{
    fn width(&self) -> Length {
        if self.is_hidden {
            Length::Fixed(0.0)
        } else {
            self.width
        }
    }

    fn height(&self) -> Length {
        if self.is_hidden {
            Length::Fixed(0.0)
        } else {
            self.height
        }
    }

    fn layout(&self, _renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(
            limits
                .width(if self.is_hidden {
                    Length::Fixed(0.0)
                } else {
                    self.width
                })
                .height(if self.is_hidden {
                    Length::Fixed(0.0)
                } else {
                    self.height
                })
                .resolve(Size::new(f32::INFINITY, f32::INFINITY)),
        )
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        // Technically, only 2 actions are supported at the moment... //
        assert!((1..3).contains(&self.actions.len()));

        if !self.is_hidden {
            let bounds: Rectangle = layout.bounds();
            let center: Point = bounds.center();

            // The origin (`Point::ORIGIN`) leaves a slight gap in x and y. Move the point back
            // (up-left) in x and y, and scale the size to cover the remaining space.
            let rectangle: Rectangle = Rectangle::new(
                Point {
                    x: Point::ORIGIN.x - 100.0,
                    y: Point::ORIGIN.y - 100.0,
                },
                Size {
                    width: viewport.width + 100.0,
                    height: viewport.height + 100.0,
                },
            );

            let draw_element = |r: &mut Renderer| {
                // Overlay //
                r.fill_quad(
                    Quad {
                        bounds: rectangle,
                        border_radius: [0.0, 0.0, 0.0, 0.0].into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                    Background::Color(secondary_system_fill()),
                );

                // Dialog Box //
                let Length::Fixed(width) = self.width else {
                    todo!()
                };
                let Length::Fixed(height) = self.height else {
                    todo!()
                };

                // The `center` puts the top-left corner of the rectangle in the origin, so shift the
                // rectangle up and to the left. The `height` calculation may seem strange, but
                // it's to center the box on the page
                let dialog_box: Rectangle = Rectangle::new(
                    Point {
                        x: center.x - width / 2.0,
                        y: center.y - 0.75 * height,
                    },
                    Size { width, height },
                );

                r.fill_quad(
                    Quad {
                        bounds: dialog_box,
                        border_radius: [15.0, 15.0, 15.0, 15.0].into(),
                        border_width: 0.0,
                        border_color: Color::WHITE,
                    },
                    Background::Color(Color::WHITE),
                );

                // Bottom Section //
                // TODO: Cover the case in which there is only one action (for whatever reason;
                // maybe just a cancel button?)
                // This is where things get interesting. Draw lines using very thin rectangles! //
                let bottom_bar: Rectangle = Rectangle::new(
                    Point {
                        x: center.x - width / 2.0,
                        y: center.y,
                    },
                    Size { width, height: 2.0 },
                );

                // Horizontal Bar //
                r.fill_quad(
                    Quad {
                        bounds: bottom_bar,
                        border_radius: [0.0, 0.0, 0.0, 0.0].into(),
                        border_width: 0.0,
                        border_color: secondary_system_fill(),
                    },
                    Background::Color(secondary_system_fill()),
                );

                // Vertical Bar //
                let vertical_bar: Rectangle = Rectangle::new(
                    Point {
                        x: center.x,
                        y: center.y,
                    },
                    Size {
                        width: 2.0,
                        height: height / 4.0,
                    },
                );

                r.fill_quad(
                    Quad {
                        bounds: vertical_bar,
                        border_radius: [0.0, 0.0, 0.0, 0.0].into(),
                        border_width: 0.0,
                        border_color: secondary_system_fill(),
                    },
                    Background::Color(secondary_system_fill()),
                );

                if self.actions.len() == 2 {
                    let child_1 = self.actions[0].child.as_widget();
                    let child_2 = self.actions[1].child.as_widget();

                    let Length::Fixed(child_1_width) = child_1.width() else {
                        todo!()
                    };
                    let Length::Fixed(child_1_height) = child_1.height() else {
                        todo!()
                    };
                    let Length::Fixed(child_2_width) = child_2.width() else {
                        todo!()
                    };
                    let Length::Fixed(child_2_height) = child_2.height() else {
                        todo!()
                    };

                    let mut bottom_left: Node = Node::new(Size {
                        width: child_1_width,
                        height: child_1_height,
                    });

                    let mut bottom_right: Node = Node::new(Size {
                        width: child_2_width,
                        height: child_2_height,
                    });

                    bottom_left.move_to(Point {
                        x: center.x - width / 3.0,
                        y: center.y + 10.0,
                    });

                    bottom_right.move_to(Point {
                        x: center.x + width / 6.0,
                        y: center.y + 10.0,
                    });

                    child_1.draw(
                        state,
                        r,
                        theme,
                        style,
                        Layout::new(&bottom_left),
                        cursor,
                        viewport,
                    );

                    child_2.draw(
                        state,
                        r,
                        theme,
                        style,
                        Layout::new(&bottom_right),
                        cursor,
                        viewport,
                    );
                }

                let mut title_node: Node = Node::new(Size {
                    width,
                    height: 75.0,
                });
                let mut content_node: Node = Node::new(Size {
                    width,
                    height: 150.0,
                });

                title_node.move_to(Point {
                    x: center.x - width / 2.0,
                    y: center.y - height / 1.5,
                });

                content_node.move_to(Point {
                    x: center.x - width / 2.0,
                    y: center.y - height / 3.0,
                });

                let title: Element<'a, Message, Renderer> =
                    CupertinoAlert::<'a, Message, Renderer>::_text_with_font(
                        Text::new(self.title.clone()).horizontal_alignment(Horizontal::Center),
                    );

                title.as_widget().draw(
                    state,
                    r,
                    theme,
                    style,
                    Layout::new(&title_node),
                    cursor,
                    viewport,
                );

                let content: Element<'a, Message, Renderer> =
                    CupertinoAlert::<'a, Message, Renderer>::_text_with_font(
                        Text::new(self.content.clone()).horizontal_alignment(Horizontal::Center),
                    );

                content.as_widget().draw(
                    state,
                    r,
                    theme,
                    style,
                    Layout::new(&content_node),
                    cursor,
                    viewport,
                );
            };

            renderer.with_layer(rectangle, draw_element);
        }
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
        let cur_pos = cursor.position().unwrap_or_default();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let bounds: Rectangle = layout.bounds();

                // If either of the bounds width/height are 0, exit early; this means the modal
                // is changing from hidden to visible
                if bounds.width == 0.0 || bounds.height == 0.0 {
                    return event::Status::Ignored;
                }

                // TODO: Handle the case when there is only 1 button, for whatever reason... //
                if self.actions.len() == 2 {
                    // For some reason, the button callbacks aren't being triggered... //

                    // Check for left button click //
                    // The hit boxes here are, by design, more generous //
                    // For the Y hit box, the range is very narrow, hence more tuning for the
                    // range
                    if self.actions[0].on_pressed.is_some() {
                        let hit_x: Range<f32> =
                            (bounds.x + 2.0 * bounds.width * 0.3)..(bounds.x + bounds.width * 0.9);
                        let hit_y: Range<f32> = (bounds.y + bounds.height / 2.0 + 10.0)
                            ..(bounds.y + bounds.height / 2.0 + 30.0);

                        if hit_x.contains(&cur_pos.x) && hit_y.contains(&cur_pos.y) {
                            shell.publish(
                                self.actions[0]
                                    .on_pressed
                                    .clone()
                                    .expect("Unable to retrieve the left button click message"),
                            );
                        }
                    }

                    // Check for right button click //
                    if self.actions[1].on_pressed.is_some() {
                        let hit_x: Range<f32> =
                            (bounds.x + bounds.width / 6.0)..(bounds.x + bounds.width / 2.0);
                        let hit_y: Range<f32> = (bounds.y + bounds.height / 2.0 + 10.0)
                            ..(bounds.y + bounds.height / 2.0 + 30.0);

                        if hit_x.contains(&cur_pos.x) && hit_y.contains(&cur_pos.y) {
                            shell.publish(
                                self.actions[1]
                                    .on_pressed
                                    .clone()
                                    .expect("Unable to retrieve the right button click message"),
                            );
                        }
                    }
                }

                // Check for clicking on the overlay //
                let hit_x: Range<f32> = bounds.x..(bounds.x + bounds.width);
                let hit_y: Range<f32> =
                    (bounds.y - bounds.height / 4.0)..(bounds.y + bounds.height * 0.85);

                if !hit_x.contains(&cur_pos.x) || !hit_y.contains(&cur_pos.y) {
                    if self.backdrop.is_some() {
                        shell.publish(
                            self.backdrop
                                .clone()
                                .expect("Unable to retrieve the backdrop message"),
                        );
                    }

                    // Default behaviour: hide the modal after clicking on the backdrop //
                    self.is_hidden = true;
                }
            }

            Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                if key_code == keyboard::KeyCode::Escape && self.on_escape.is_some() {
                    self.is_hidden = true;

                    shell.publish(
                        self.on_escape
                            .clone()
                            .expect("Unable to retrieve the escape message"),
                    );
                    return event::Status::Captured;
                }
            }

            _ => return event::Status::Ignored,
        }

        event::Status::Ignored
    }
}

#[allow(clippy::type_repetition_in_bounds)]
impl<'a, Message, Renderer: 'a> From<CupertinoAlert<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: core::Renderer + core::text::Renderer<Font = Font> + 'a,
    Renderer::Theme: application::StyleSheet + text::StyleSheet,
{
    fn from(alert: CupertinoAlert<'a, Message, Renderer>) -> Self {
        Self::new(alert)
    }
}
