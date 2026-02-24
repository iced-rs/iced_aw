//! Drop down menu widget
//!
//! *This API requires the following crate features to be activated: `drop_down`*

use iced_core::{
    Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Size, Vector, Widget,
    keyboard::{self, key::Named},
    layout::{Limits, Node},
    mouse::{self, Cursor},
    overlay, renderer, touch,
    widget::{self, Operation, Tree},
};

pub use crate::core::{alignment::Alignment, offset::Offset};

/// Customizable drop down menu widget
pub struct DropDown<'a, Message, Theme = iced_widget::Theme, Renderer = iced_widget::Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    underlay: Element<'a, Message, Theme, Renderer>,
    overlay: Element<'a, Message, Theme, Renderer>,
    on_dismiss: Option<Message>,
    width: Option<Length>,
    height: Length,
    alignment: Alignment,
    offset: Offset,
    expanded: bool,
}

impl<'a, Message, Theme, Renderer> DropDown<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    /// Create a new [`DropDown`]
    pub fn new<U, B>(underlay: U, overlay: B, expanded: bool) -> Self
    where
        U: Into<Element<'a, Message, Theme, Renderer>>,
        B: Into<Element<'a, Message, Theme, Renderer>>,
    {
        DropDown {
            underlay: underlay.into(),
            overlay: overlay.into(),
            expanded,
            on_dismiss: None,
            width: None,
            height: Length::Shrink,
            alignment: Alignment::Bottom,
            offset: Offset::from(5.0),
        }
    }

    /// The width of the overlay
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// The height of the overlay
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// The alignment of the overlay relative to the underlay
    #[must_use]
    pub fn alignment(mut self, alignment: impl Into<Alignment>) -> Self {
        self.alignment = alignment.into();
        self
    }

    /// The offset of the overlay
    #[must_use]
    pub fn offset(mut self, offset: impl Into<Offset>) -> Self {
        self.offset = offset.into();
        self
    }

    /// Send a message when a click occur outside of the overlay when expanded
    #[must_use]
    pub fn on_dismiss(mut self, message: Message) -> Self {
        self.on_dismiss = Some(message);
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for DropDown<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.underlay.as_widget().size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
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
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.overlay)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &self.overlay]);
    }

    fn operate<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        self.underlay
            .as_widget_mut()
            .operate(&mut state.children[0], layout, renderer, operation);
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget_mut().update(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
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
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        if !self.expanded {
            return self.underlay.as_widget_mut().overlay(
                &mut state.children[0],
                layout,
                renderer,
                viewport,
                translation,
            );
        }

        Some(overlay::Element::new(Box::new(DropDownOverlay::new(
            &mut state.children[1],
            &mut self.overlay,
            self.on_dismiss.as_ref(),
            self.width.as_ref(),
            &self.height,
            &self.alignment,
            &self.offset,
            layout.bounds(),
            layout.position() + translation,
            *viewport,
        ))))
    }
}

impl<'a, Message, Theme: 'a, Renderer> From<DropDown<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
{
    fn from(drop_down: DropDown<'a, Message, Theme, Renderer>) -> Self {
        Element::new(drop_down)
    }
}

struct DropDownOverlay<
    'a,
    'b,
    Message,
    Theme = iced_widget::Theme,
    Renderer = iced_widget::Renderer,
> where
    Message: Clone,
{
    state: &'b mut Tree,
    element: &'b mut Element<'a, Message, Theme, Renderer>,
    on_dismiss: Option<&'b Message>,
    width: Option<&'b Length>,
    height: &'b Length,
    alignment: &'b Alignment,
    offset: &'b Offset,
    underlay_bounds: Rectangle,
    position: Point,
    viewport: Rectangle,
}

impl<'a, 'b, Message, Theme, Renderer> DropDownOverlay<'a, 'b, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    #[allow(clippy::too_many_arguments)]
    fn new(
        state: &'b mut Tree,
        element: &'b mut Element<'a, Message, Theme, Renderer>,
        on_dismiss: Option<&'b Message>,
        width: Option<&'b Length>,
        height: &'b Length,
        alignment: &'b Alignment,
        offset: &'b Offset,
        underlay_bounds: Rectangle,
        position: Point,
        viewport: Rectangle,
    ) -> Self {
        DropDownOverlay {
            state,
            element,
            on_dismiss,
            width,
            height,
            alignment,
            offset,
            underlay_bounds,
            position,
            viewport,
        }
    }
}

impl<Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for DropDownOverlay<'_, '_, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> Node {
        let limits = Limits::new(Size::ZERO, bounds)
            .width(
                *self
                    .width
                    .unwrap_or(&Length::Fixed(self.underlay_bounds.width)),
            )
            .height(*self.height);

        let previous_position = self.position;
        let max = limits.max();

        let height_above = (previous_position.y - self.offset.y).max(0.0);
        let height_below =
            (max.height - previous_position.y - self.underlay_bounds.height - self.offset.y)
                .max(0.0);

        let ref_center_y = previous_position.y + self.underlay_bounds.height / 2.0;
        let max_height_symmetric = (ref_center_y.min(max.height - ref_center_y) * 2.0).max(0.0);

        let limits = match self.alignment {
            Alignment::Top => limits.max_height(height_above),
            Alignment::TopStart | Alignment::TopEnd => {
                limits.max_height((height_above + self.underlay_bounds.height).max(0.0))
            }
            Alignment::Bottom => limits.max_height(height_below),
            Alignment::BottomEnd | Alignment::BottomStart => {
                limits.max_height((height_below + self.underlay_bounds.height).max(0.0))
            }
            Alignment::Start | Alignment::End => limits.max_height(max_height_symmetric),
        };

        let node = self
            .element
            .as_widget_mut()
            .layout(self.state, renderer, &limits);

        let mut new_position = match self.alignment {
            Alignment::TopStart => Point::new(
                previous_position.x - node.bounds().width - self.offset.x,
                previous_position.y - node.bounds().height + self.underlay_bounds.height
                    - self.offset.y,
            ),
            Alignment::Top => Point::new(
                previous_position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0,
                previous_position.y - node.bounds().height - self.offset.y,
            ),
            Alignment::TopEnd => Point::new(
                previous_position.x + self.underlay_bounds.width + self.offset.x,
                previous_position.y - node.bounds().height + self.underlay_bounds.height
                    - self.offset.y,
            ),
            Alignment::End => Point::new(
                previous_position.x + self.underlay_bounds.width + self.offset.x,
                previous_position.y + self.underlay_bounds.height / 2.0
                    - node.bounds().height / 2.0,
            ),
            Alignment::BottomEnd => Point::new(
                previous_position.x + self.underlay_bounds.width + self.offset.x,
                previous_position.y + self.offset.y,
            ),
            Alignment::Bottom => Point::new(
                previous_position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0,
                previous_position.y + self.underlay_bounds.height + self.offset.y,
            ),
            Alignment::BottomStart => Point::new(
                previous_position.x - node.bounds().width - self.offset.x,
                previous_position.y + self.offset.y,
            ),
            Alignment::Start => Point::new(
                previous_position.x - node.bounds().width - self.offset.x,
                previous_position.y + self.underlay_bounds.height / 2.0
                    - node.bounds().height / 2.0,
            ),
        };

        if new_position.x + node.bounds().width > self.viewport.width {
            new_position.x -= node.bounds().width;
        }

        if new_position.x < 0.0 {
            new_position.x = 0.0;
        }

        if new_position.y + node.bounds().height > self.viewport.height {
            new_position.y -= node.bounds().height;
        }
        if new_position.y < 0.0 {
            new_position.y = 0.0;
        }

        node.move_to(new_position)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
    ) {
        let bounds = layout.bounds();
        self.element
            .as_widget()
            .draw(self.state, renderer, theme, style, layout, cursor, &bounds);
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) {
        self.underlay_bounds = Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.underlay_bounds.width,
            height: self.underlay_bounds.height,
        };

        if let Some(on_dismiss) = self.on_dismiss {
            match &event {
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                    if key == &keyboard::Key::Named(Named::Escape) {
                        shell.publish(on_dismiss.clone());
                    }
                }

                Event::Mouse(mouse::Event::ButtonPressed(
                    mouse::Button::Left | mouse::Button::Right,
                ))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if !cursor.is_over(layout.bounds()) && !cursor.is_over(self.underlay_bounds) {
                        shell.publish(on_dismiss.clone());
                    }
                }

                _ => {}
            }
        }

        self.element.as_widget_mut().update(
            self.state,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.element.as_widget().mouse_interaction(
            self.state,
            layout,
            cursor,
            &self.viewport,
            renderer,
        )
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        // Operate on the overlay element to expose its children for testing
        self.element
            .as_widget_mut()
            .operate(self.state, layout, renderer, operation);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_widget::text::Text;

    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum TestMessage {
        Dismiss,
        Select(usize),
    }

    type TestDropDown<'a> = DropDown<'a, TestMessage, iced_widget::Theme, iced_widget::Renderer>;

    // ============================================================================
    // Construction Tests
    // ============================================================================

    #[test]
    fn drop_down_new_creates_instance() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false);

        assert!(!dropdown.expanded, "Should not be expanded by default");
        assert!(
            dropdown.on_dismiss.is_none(),
            "Should have no dismiss message"
        );
        assert!(dropdown.width.is_none(), "Should have no custom width");
        assert_eq!(
            dropdown.height,
            Length::Shrink,
            "Default height should be Shrink"
        );
        assert_eq!(
            dropdown.alignment,
            Alignment::Bottom,
            "Default alignment should be Bottom"
        );
    }

    #[test]
    fn drop_down_new_with_expanded_true() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, true);

        assert!(dropdown.expanded, "Should be expanded");
    }

    #[test]
    fn drop_down_new_with_expanded_false() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false);

        assert!(!dropdown.expanded, "Should not be expanded");
    }

    // ============================================================================
    // Builder Method Tests
    // ============================================================================

    #[test]
    fn drop_down_width_sets_value() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false).width(Length::Fixed(200.0));

        assert_eq!(dropdown.width, Some(Length::Fixed(200.0)));
    }

    #[test]
    fn drop_down_width_fill() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false).width(Length::Fill);

        assert_eq!(dropdown.width, Some(Length::Fill));
    }

    #[test]
    fn drop_down_height_sets_value() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false).height(Length::Fixed(150.0));

        assert_eq!(dropdown.height, Length::Fixed(150.0));
    }

    #[test]
    fn drop_down_height_fill() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false).height(Length::Fill);

        assert_eq!(dropdown.height, Length::Fill);
    }

    #[test]
    fn drop_down_alignment_sets_value() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false).alignment(Alignment::Top);

        assert_eq!(dropdown.alignment, Alignment::Top);
    }

    #[test]
    fn drop_down_alignment_bottom_start() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown =
            TestDropDown::new(underlay, overlay, false).alignment(Alignment::BottomStart);

        assert_eq!(dropdown.alignment, Alignment::BottomStart);
    }

    #[test]
    fn drop_down_alignment_top_end() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false).alignment(Alignment::TopEnd);

        assert_eq!(dropdown.alignment, Alignment::TopEnd);
    }

    #[test]
    fn drop_down_offset_sets_value() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false).offset(Offset::from(10.0));

        assert_eq!(dropdown.offset.x, 10.0);
        assert_eq!(dropdown.offset.y, 10.0);
    }

    #[test]
    fn drop_down_offset_different_x_y() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");
        let custom_offset = Offset { x: 5.0, y: 15.0 };

        let dropdown = TestDropDown::new(underlay, overlay, false).offset(custom_offset);

        assert_eq!(dropdown.offset.x, 5.0);
        assert_eq!(dropdown.offset.y, 15.0);
    }

    #[test]
    fn drop_down_on_dismiss_sets_message() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false).on_dismiss(TestMessage::Dismiss);

        assert!(dropdown.on_dismiss.is_some());
    }

    #[test]
    fn drop_down_without_on_dismiss_is_none() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false);

        assert!(dropdown.on_dismiss.is_none());
    }

    // ============================================================================
    // Method Chaining Tests
    // ============================================================================

    #[test]
    fn drop_down_chaining_methods() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, true)
            .width(Length::Fixed(300.0))
            .height(Length::Fixed(200.0))
            .alignment(Alignment::Top)
            .offset(Offset::from(15.0))
            .on_dismiss(TestMessage::Dismiss);

        assert!(dropdown.expanded);
        assert_eq!(dropdown.width, Some(Length::Fixed(300.0)));
        assert_eq!(dropdown.height, Length::Fixed(200.0));
        assert_eq!(dropdown.alignment, Alignment::Top);
        assert_eq!(dropdown.offset.x, 15.0);
        assert_eq!(dropdown.offset.y, 15.0);
        assert!(dropdown.on_dismiss.is_some());
    }

    #[test]
    fn drop_down_partial_chaining() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false)
            .width(Length::Fill)
            .alignment(Alignment::BottomEnd);

        assert_eq!(dropdown.width, Some(Length::Fill));
        assert_eq!(dropdown.alignment, Alignment::BottomEnd);
        assert_eq!(dropdown.height, Length::Shrink); // Default
        assert!(dropdown.on_dismiss.is_none()); // Not set
    }

    // ============================================================================
    // Widget Trait Tests
    // ============================================================================

    #[test]
    fn drop_down_has_two_children() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false);

        let children =
            Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::children(&dropdown);
        assert_eq!(
            children.len(),
            2,
            "DropDown should have 2 children (underlay and overlay)"
        );
    }

    #[test]
    fn drop_down_size_matches_underlay() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false);

        let size =
            Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::size(&dropdown);
        // Text widget has Shrink for both dimensions by default
        assert_eq!(size.width, Length::Shrink);
        assert_eq!(size.height, Length::Shrink);
    }

    // ============================================================================
    // Configuration Tests
    // ============================================================================

    #[test]
    fn drop_down_alignment_bottom() {
        let dropdown =
            TestDropDown::new(Text::new("Click me"), Text::new("Dropdown content"), false)
                .alignment(Alignment::Bottom);

        assert_eq!(dropdown.alignment, Alignment::Bottom);
    }

    #[test]
    fn drop_down_alignment_start() {
        let dropdown =
            TestDropDown::new(Text::new("Click me"), Text::new("Dropdown content"), false)
                .alignment(Alignment::Start);

        assert_eq!(dropdown.alignment, Alignment::Start);
    }

    #[test]
    fn drop_down_alignment_end() {
        let dropdown =
            TestDropDown::new(Text::new("Click me"), Text::new("Dropdown content"), false)
                .alignment(Alignment::End);

        assert_eq!(dropdown.alignment, Alignment::End);
    }

    #[test]
    fn drop_down_different_widths() {
        let widths = [
            Length::Fill,
            Length::Shrink,
            Length::Fixed(100.0),
            Length::Fixed(500.0),
            Length::FillPortion(2),
        ];

        for width in widths {
            let underlay = Text::new("Click me");
            let overlay = Text::new("Dropdown content");

            let dropdown = TestDropDown::new(underlay, overlay, false).width(width);

            assert_eq!(dropdown.width, Some(width));
        }
    }

    #[test]
    fn drop_down_different_heights() {
        let heights = [
            Length::Fill,
            Length::Shrink,
            Length::Fixed(100.0),
            Length::Fixed(500.0),
            Length::FillPortion(2),
        ];

        for height in heights {
            let underlay = Text::new("Click me");
            let overlay = Text::new("Dropdown content");

            let dropdown = TestDropDown::new(underlay, overlay, false).height(height);

            assert_eq!(dropdown.height, height);
        }
    }

    #[test]
    fn drop_down_zero_offset() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown =
            TestDropDown::new(underlay, overlay, false).offset(Offset { x: 0.0, y: 0.0 });

        assert_eq!(dropdown.offset.x, 0.0);
        assert_eq!(dropdown.offset.y, 0.0);
    }

    #[test]
    fn drop_down_negative_offset() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown =
            TestDropDown::new(underlay, overlay, false).offset(Offset { x: -5.0, y: -10.0 });

        assert_eq!(dropdown.offset.x, -5.0);
        assert_eq!(dropdown.offset.y, -10.0);
    }

    #[test]
    fn drop_down_large_offset() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false).offset(Offset {
            x: 1000.0,
            y: 500.0,
        });

        assert_eq!(dropdown.offset.x, 1000.0);
        assert_eq!(dropdown.offset.y, 500.0);
    }

    // ============================================================================
    // Edge Case Tests
    // ============================================================================

    #[test]
    fn drop_down_with_empty_text() {
        let underlay = Text::new("");
        let overlay = Text::new("");

        let dropdown = TestDropDown::new(underlay, overlay, false);

        assert!(!dropdown.expanded);
    }

    #[test]
    fn drop_down_multiple_instances() {
        let dropdown1 = TestDropDown::new(Text::new("First"), Text::new("Overlay 1"), false);

        let dropdown2 = TestDropDown::new(Text::new("Second"), Text::new("Overlay 2"), true);

        assert!(!dropdown1.expanded);
        assert!(dropdown2.expanded);
    }

    #[test]
    fn drop_down_with_different_message_types() {
        let dropdown1 =
            TestDropDown::new(Text::new("Click me"), Text::new("Dropdown content"), false)
                .on_dismiss(TestMessage::Dismiss);

        let dropdown2 =
            TestDropDown::new(Text::new("Click me"), Text::new("Dropdown content"), false)
                .on_dismiss(TestMessage::Select(42));

        assert!(dropdown1.on_dismiss.is_some());
        assert!(dropdown2.on_dismiss.is_some());
    }

    #[test]
    fn drop_down_converts_to_element() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false);
        let _element: Element<'_, TestMessage, iced_widget::Theme, iced_widget::Renderer> =
            dropdown.into();
        // Just verify it compiles and converts
    }

    #[test]
    fn drop_down_default_offset_is_5() {
        let underlay = Text::new("Click me");
        let overlay = Text::new("Dropdown content");

        let dropdown = TestDropDown::new(underlay, overlay, false);

        // Default offset should be 5.0 (from Offset::from(5.0) in new())
        assert_eq!(dropdown.offset.x, 5.0);
        assert_eq!(dropdown.offset.y, 5.0);
    }
}
