//! A context menu for showing actions on right click.
//!
use iced_core::{
    Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Vector, Widget,
    layout::{Limits, Node},
    mouse::{self, Button, Cursor},
    overlay, renderer,
    widget::{Operation, Tree, tree},
};

pub use crate::style::{
    context_menu::{Catalog, Style},
    status::{Status, StyleFn},
};

use crate::widget::overlay::ContextMenuOverlay;

/// A context menu
///
///
/// # Example
/// ```ignore
/// # use iced_widget::{Text, Button};
/// # use iced_aw::ContextMenu;
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
///     Action1,
/// }
///
/// let underlay = Text::new("right click me");
///
/// let cm = ContextMenu::new(
///     underlay,
///     || Button::new("action1").on_press(Message::Action1).into()
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct ContextMenu<
    'a,
    Overlay,
    Message,
    Theme = iced_widget::Theme,
    Renderer = iced_widget::Renderer,
> where
    Overlay: Fn() -> Element<'a, Message, Theme, Renderer>,
    Message: Clone,
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// The underlying element.
    underlay: Element<'a, Message, Theme, Renderer>,
    /// The content of [`ContextMenuOverlay`].
    overlay: Overlay,
    /// The style of the [`ContextMenu`].
    class: Theme::Class<'a>,
    /// Force the menu to be shown (for testing purposes). If None, uses internal state.
    force_open: Option<bool>,
}

impl<'a, Overlay, Message, Theme, Renderer> ContextMenu<'a, Overlay, Message, Theme, Renderer>
where
    Overlay: Fn() -> Element<'a, Message, Theme, Renderer>,
    Message: Clone,
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// Creates a new [`ContextMenu`]
    ///
    /// `underlay`: The underlying element.
    ///
    /// `overlay`: The content of [`ContextMenuOverlay`] which will be displayed when `underlay` is clicked.
    pub fn new<U>(underlay: U, overlay: Overlay) -> Self
    where
        U: Into<Element<'a, Message, Theme, Renderer>>,
    {
        ContextMenu {
            underlay: underlay.into(),
            overlay,
            class: Theme::default(),
            force_open: None,
        }
    }

    /// Sets the style of the [`ContextMenu`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Sets the class of the input of the [`ContextMenu`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Forces the menu to be open or closed, overriding the internal state.
    /// This is primarily useful for testing purposes.
    /// If `None`, the menu uses its internal state (toggled by right-click).
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.force_open = Some(open);
        self
    }
}

impl<'a, Content, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ContextMenu<'a, Content, Message, Theme, Renderer>
where
    Content: 'a + Fn() -> Element<'a, Message, Theme, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: Catalog,
{
    fn size(&self) -> iced_core::Size<Length> {
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

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new((self.overlay)())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &(self.overlay)()]);
    }

    fn operate<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let s: &mut State = state.state.downcast_mut();
        let show = self.force_open.unwrap_or(s.show);

        if show {
            let mut content = (self.overlay)();
            content.as_widget_mut().diff(&mut state.children[1]);

            content
                .as_widget_mut()
                .operate(&mut state.children[1], layout, renderer, operation);
        } else {
            self.underlay.as_widget_mut().operate(
                &mut state.children[0],
                layout,
                renderer,
                operation,
            );
        }
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
        if *event == Event::Mouse(mouse::Event::ButtonPressed(Button::Right)) {
            let bounds = layout.bounds();

            if cursor.is_over(bounds) {
                let s: &mut State = state.state.downcast_mut();
                s.cursor_position = cursor.position().unwrap_or_default();
                s.show = !s.show;
                shell.capture_event();
            }
        }

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
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let s: &mut State = tree.state.downcast_mut();
        let show = self.force_open.unwrap_or(s.show);

        if !show {
            return self.underlay.as_widget_mut().overlay(
                &mut tree.children[0],
                layout,
                renderer,
                viewport,
                translation,
            );
        }

        let position = s.cursor_position;
        let mut content = (self.overlay)();
        content.as_widget_mut().diff(&mut tree.children[1]);
        Some(
            ContextMenuOverlay::new(
                position + translation,
                &mut tree.children[1],
                content,
                &self.class,
                s,
            )
            .overlay(),
        )
    }
}

impl<'a, Content, Message, Theme, Renderer> From<ContextMenu<'a, Content, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Content: 'a + Fn() -> Self,
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: 'a + Catalog,
{
    fn from(modal: ContextMenu<'a, Content, Message, Theme, Renderer>) -> Self {
        Element::new(modal)
    }
}

/// The state of the ``context_menu``.
#[derive(Debug, Default)]
pub(crate) struct State {
    /// The visibility of the [`ContextMenu`] overlay.
    pub show: bool,
    /// Use for showing the overlay where the click was made.
    pub cursor_position: Point,
}

impl State {
    /// Creates a new [`State`] containing the given state data.
    pub const fn new() -> Self {
        Self {
            show: false,
            cursor_position: Point::ORIGIN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    enum TestMessage {}

    type TestContextMenu<'a> = ContextMenu<
        'a,
        fn() -> Element<'a, TestMessage, iced_widget::Theme, iced_widget::Renderer>,
        TestMessage,
        iced_widget::Theme,
        iced_widget::Renderer,
    >;

    fn create_overlay() -> Element<'static, TestMessage, iced_widget::Theme, iced_widget::Renderer>
    {
        iced_widget::text::Text::new("Overlay").into()
    }

    #[test]
    fn state_new_has_default_values() {
        let state = State::new();
        assert!(!state.show);
        assert_eq!(state.cursor_position, Point::ORIGIN);
    }

    #[test]
    fn state_default_trait() {
        let state = State::default();
        assert!(!state.show);
        assert_eq!(state.cursor_position, Point::ORIGIN);
    }

    #[test]
    fn context_menu_new_creates_instance() {
        let underlay = iced_widget::text::Text::new("Underlay");
        let context_menu = TestContextMenu::new(underlay, create_overlay);

        let size =
            Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::size(&context_menu);
        assert_eq!(size.width, Length::Shrink);
        assert_eq!(size.height, Length::Shrink);
    }

    #[test]
    fn context_menu_tag_returns_state_tag() {
        let underlay = iced_widget::text::Text::new("Underlay");
        let context_menu = TestContextMenu::new(underlay, create_overlay);

        let tag =
            Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::tag(&context_menu);
        assert_eq!(tag, tree::Tag::of::<State>());
    }

    #[test]
    fn context_menu_has_two_children() {
        let underlay = iced_widget::text::Text::new("Underlay");
        let context_menu = TestContextMenu::new(underlay, create_overlay);

        let children = Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::children(
            &context_menu,
        );
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn context_menu_size_matches_underlay() {
        let underlay = iced_widget::text::Text::new("Underlay");
        let context_menu = TestContextMenu::new(underlay, create_overlay);

        let size =
            Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::size(&context_menu);
        assert_eq!(size.width, Length::Shrink);
        assert_eq!(size.height, Length::Shrink);
    }

    #[test]
    fn state_cursor_position_can_be_set() {
        let mut state = State::new();
        state.cursor_position = Point::new(100.0, 200.0);
        assert_eq!(state.cursor_position.x, 100.0);
        assert_eq!(state.cursor_position.y, 200.0);
    }

    #[test]
    fn state_show_can_be_toggled() {
        let mut state = State::new();
        assert!(!state.show);

        state.show = true;
        assert!(state.show);

        state.show = false;
        assert!(!state.show);
    }

    #[test]
    fn state_allows_show_and_position_updates() {
        // Test that State can track show status and cursor position
        // This exercises the internal state logic used by update()
        let mut state = State::new();

        // Initially not shown
        assert!(!state.show);
        assert_eq!(state.cursor_position, Point::ORIGIN);

        // Simulate right-click at position
        state.show = !state.show;
        state.cursor_position = Point::new(10.0, 10.0);

        assert!(state.show);
        assert_eq!(state.cursor_position, Point::new(10.0, 10.0));

        // Simulate toggle off
        state.show = !state.show;
        assert!(!state.show);
    }

    #[test]
    fn widget_state_creates_correct_initial_state() {
        // Test that widget creates correct initial state
        let underlay = iced_widget::text::Text::new("Underlay");
        let context_menu = TestContextMenu::new(underlay, create_overlay);

        let state =
            Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::state(&context_menu);

        // Verify state is created with correct type
        let s: &State = state.downcast_ref();
        assert!(!s.show);
        assert_eq!(s.cursor_position, Point::ORIGIN);
    }

    #[test]
    fn widget_children_returns_two_elements() {
        // Test that children() returns underlay and overlay
        let underlay = iced_widget::text::Text::new("Underlay");
        let context_menu = TestContextMenu::new(underlay, create_overlay);

        let children = Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::children(
            &context_menu,
        );

        // Should have 2 children: underlay and overlay
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn open_method_sets_force_open_to_true() {
        let underlay = iced_widget::text::Text::new("Underlay");
        let context_menu = TestContextMenu::new(underlay, create_overlay).open(true);

        assert_eq!(context_menu.force_open, Some(true));
    }

    #[test]
    fn open_method_sets_force_open_to_false() {
        let underlay = iced_widget::text::Text::new("Underlay");
        let context_menu = TestContextMenu::new(underlay, create_overlay).open(false);

        assert_eq!(context_menu.force_open, Some(false));
    }

    #[test]
    fn open_method_can_be_chained() {
        let underlay = iced_widget::text::Text::new("Underlay");
        let context_menu = TestContextMenu::new(underlay, create_overlay)
            .open(true)
            .class(<iced_widget::Theme as crate::style::context_menu::Catalog>::default());

        assert_eq!(context_menu.force_open, Some(true));
    }

    #[test]
    fn default_force_open_is_none() {
        let underlay = iced_widget::text::Text::new("Underlay");
        let context_menu = TestContextMenu::new(underlay, create_overlay);

        assert_eq!(context_menu.force_open, None);
    }
}
