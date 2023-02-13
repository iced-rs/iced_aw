//! A widget that handles menu trees

use super::menu::{ItemHeight, ItemWidth, Menu, MenuState, PathHighlight};
use super::menu_tree::MenuTree;
use crate::style::menu_bar::StyleSheet;
use iced_native::widget::{tree, Tree};
use iced_native::{
    event, layout, mouse, overlay, renderer, touch, Alignment, Clipboard, Color, Element, Length,
    Padding, Point, Rectangle, Shell, Widget,
};

pub(super) struct MenuBarState {
    pub(super) pressed: bool,
    pub(super) cursor: Point,
    pub(super) open: bool,
    pub(super) active_root: Option<usize>,
    pub(super) menu_states: Vec<MenuState>,
}
impl MenuBarState {
    pub(super) fn get_trimmed_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.menu_states
            .iter()
            .take_while(|ms| ms.index.is_some())
            .map(|ms| ms.index.unwrap())
    }

    pub(super) fn reset(&mut self) {
        self.open = false;
        self.active_root = None;
        self.menu_states.clear();
    }
}
impl Default for MenuBarState {
    fn default() -> Self {
        Self {
            pressed: false,
            cursor: Point::new(-0.5, -0.5),
            open: false,
            active_root: None,
            menu_states: Vec::new(),
        }
    }
}

/// A `MenuBar` collects `MenuTree`s and handles
/// all the layout, event processing and drawing
#[allow(missing_debug_implementations)]
pub struct MenuBar<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    width: Length,
    height: Length,
    spacing: u16,
    padding: Padding,
    bounds_expand: u16,
    item_width: ItemWidth,
    item_height: ItemHeight,
    path_highlight: Option<PathHighlight>,
    menu_roots: Vec<MenuTree<'a, Message, Renderer>>,
    style: <Renderer::Theme as StyleSheet>::Style,
}
impl<'a, Message, Renderer> MenuBar<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`MenuBar`] with the given menu roots
    pub fn new(menu_roots: Vec<MenuTree<'a, Message, Renderer>>) -> Self {
        let mut menu_roots = menu_roots;
        menu_roots.iter_mut().for_each(|mr| mr.set_index());

        Self {
            width: Length::Shrink,
            height: Length::Shrink,
            spacing: 0,
            padding: Padding::ZERO,
            bounds_expand: 15,
            item_width: ItemWidth::Uniform(150),
            item_height: ItemHeight::Uniform(30),
            path_highlight: Some(PathHighlight::MenuActive),
            menu_roots,
            style: <Renderer::Theme as StyleSheet>::Style::default(),
        }
    }

    /// Sets the width of the [`MenuBar`]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`MenuBar`]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the spacing between menu roots
    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    /// Sets the expand value for each menu's check bounds
    ///
    /// When the cursor goes outside of a menu's check bounds,
    /// the menu will be closed automatically, this value expands
    /// the check bounds
    pub fn bounds_expand(mut self, value: u16) -> Self {
        self.bounds_expand = value;
        self
    }

    /// Sets the [`Padding`] of the [`MenuBar`]
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// [`ItemWidth`]
    pub fn item_width(mut self, item_width: ItemWidth) -> Self {
        self.item_width = item_width;
        self
    }

    /// [`ItemHeight`]
    pub fn item_height(mut self, item_height: ItemHeight) -> Self {
        self.item_height = item_height;
        self
    }

    /// Sets the method for drawing path highlight
    pub fn path_highlight(mut self, path_highlight: Option<PathHighlight>) -> Self {
        self.path_highlight = path_highlight;
        self
    }

    /// Sets the style of the menu bar and its menus
    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }
}
impl<'a, Message, Renderer> Widget<Message, Renderer> for MenuBar<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn state(&self) -> tree::State {
        tree::State::new(MenuBarState::default())
    }

    fn children(&self) -> Vec<Tree> {
        /*
        menu bar
            menu root 1 (stateless)
                flat tree
            menu root 2 (stateless)
                flat tree
            ...
        */

        self.menu_roots
            .iter()
            .map(|root| {
                let mut tree = Tree::empty();
                let flat = root
                    .flattern()
                    .iter()
                    .map(|mt| Tree::new(mt.item.as_widget()))
                    .collect();
                tree.children = flat;
                tree
            })
            .collect()
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        use super::flex;

        let limits = limits.width(self.width).height(self.height);
        let children = self
            .menu_roots
            .iter()
            .map(|root| &root.item)
            .collect::<Vec<_>>();
        flex::resolve(
            flex::Axis::Horizontal,
            renderer,
            &limits,
            self.padding,
            self.spacing as f32,
            Alignment::Center,
            &children,
        )
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: event::Event,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        use event::Event::*;
        use mouse::{Button::Left, Event::*};
        use touch::Event::*;

        let root_status = process_root_events(
            &mut self.menu_roots,
            cursor_position,
            tree,
            event.clone(),
            layout,
            renderer,
            clipboard,
            shell,
        );

        let state = tree.state.downcast_mut::<MenuBarState>();

        match event {
            Mouse(ButtonReleased(Left)) | Touch(FingerLifted { .. }) | Touch(FingerLost { .. }) => {
                if state.menu_states.is_empty() && layout.bounds().contains(cursor_position) {
                    state.cursor = cursor_position;
                    state.open = true;
                }
            }
            _ => (),
        }
        root_status
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as renderer::Renderer>::Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<MenuBarState>();

        let position = if state.open && (cursor_position.x < 0.0 || cursor_position.y < 0.0) {
            state.cursor
        } else {
            cursor_position
        };

        // draw path highlight
        if let Some(_) = self.path_highlight {
            let styling = theme.appearance(&self.style);
            if let Some(active) = state.active_root {
                let active_bounds = layout
                    .clone()
                    .children()
                    .skip(active)
                    .next()
                    .unwrap()
                    .bounds();
                let path_quad = renderer::Quad {
                    bounds: active_bounds,
                    border_radius: styling.border_radius.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                };
                let path_color = styling.path;
                renderer.fill_quad(path_quad, path_color);
            }
        }

        self.menu_roots
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .for_each(|((root, t), lo)| {
                root.item.as_widget().draw(
                    &t.children[root.index],
                    renderer,
                    theme,
                    style,
                    lo,
                    position,
                    viewport,
                );
            })
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        let state = tree.state.downcast_ref::<MenuBarState>();
        if !state.open {
            return None;
        }

        Some(
            Menu {
                tree,
                menu_roots: &mut self.menu_roots,
                bounds_expand: self.bounds_expand,
                item_width: self.item_width,
                item_height: self.item_height,
                bar_bounds: layout.bounds(),
                root_bounds_list: layout.children().map(|lo| lo.bounds()).collect(),
                path_highlight: self.path_highlight,
                style: &self.style,
            }
            .overlay(),
        )
    }
}
impl<'a, Message, Renderer> From<MenuBar<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(value: MenuBar<'a, Message, Renderer>) -> Self {
        Self::new(value)
    }
}

#[allow(unused_results)]
fn process_root_events<'a, 'b, Message, Renderer: renderer::Renderer>(
    menu_roots: &mut Vec<MenuTree<'a, Message, Renderer>>,
    position: Point,
    tree: &mut Tree,
    event: event::Event,
    layout: layout::Layout<'_>,
    renderer: &Renderer,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
) -> event::Status {
    menu_roots
        .iter_mut()
        .zip(&mut tree.children)
        .zip(layout.children())
        .map(|((root, t), lo)| {
            // assert!(t.tag == tree::Tag::stateless());
            root.item.as_widget_mut().on_event(
                &mut t.children[root.index],
                event.clone(),
                lo,
                position,
                renderer,
                clipboard,
                shell,
            )
        })
        .fold(event::Status::Ignored, event::Status::merge)
}
