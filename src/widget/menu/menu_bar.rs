//! [`MenuBar`]

#![allow(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use iced::{
    advanced::{
        layout::{Limits, Node},
        mouse, overlay, renderer,
        widget::{tree, Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    alignment, event, Element, Event, Length, Padding, Pixels, Rectangle, Size, Color
};

use super::{common::*, flex, menu_bar_overlay::MenuBarOverlay, menu_tree::*};
use crate::style::menu_bar::*;
pub use crate::style::status::{Status, StyleFn};

#[derive(Debug, Clone, Copy)]
pub(super) enum MenuBarTask{
    OpenOnClick,
    CloseOnClick,
}

#[derive(Default, Debug)]
pub(super) struct GlobalState {
    pub(super) pressed: bool,
    task: Option<MenuBarTask>,
}
impl GlobalState{
    pub(super) fn schedule(&mut self, task: MenuBarTask) {
        self.task = Some(task);
    }

    pub(super) fn task(&self) -> Option<MenuBarTask> {
        self.task
    }
}

#[derive(Default, Debug)]
pub(super) struct MenuBarState {
    pub(super) global_state: GlobalState,
    /// state is None when the menu bar is not open
    pub(super) menu_state: Option<MenuState>,
}
impl MenuBarState {
    pub(super) fn open<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
        &mut self,
        roots: &mut [Item<'a, Message, Theme, Renderer>],
        item_trees: &mut [Tree],
        item_layouts: impl Iterator<Item = Layout<'b>>,
        cursor: mouse::Cursor,
        shell: &mut Shell<'_, Message>,
    ){
        if self.menu_state.is_none() {
            self.menu_state = Some(MenuState::default());
        }
        let bar_menu_state = self.menu_state.as_mut().unwrap();

        try_open_menu(
            roots,
            bar_menu_state,
            item_trees,
            item_layouts,
            cursor,
            shell,
            0,
        );

        self.global_state.task = None;
    }
    
    pub(super) fn close<Message>(
        &mut self,
        item_trees: &mut [Tree],
        shell: &mut Shell<'_, Message>,
    ) {
        if self.global_state.pressed {
            return;
        }
        
        for item_tree in item_trees.iter_mut() {
            if item_tree.children.len() == 2 {
                let _ = item_tree.children.pop();
                shell.invalidate_layout();
            }
        }
        self.global_state.pressed = false;
        self.global_state.task = None;
        self.menu_state = None;
        shell.request_redraw();
    }
}

/// menu bar
#[must_use]
pub struct MenuBar<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    pub(super) roots: Vec<Item<'a, Message, Theme, Renderer>>,
    spacing: Pixels,
    padding: Padding,
    width: Length,
    height: Length,
    pub(super) global_parameters: GlobalParameters<'a, Theme>,
}
impl<'a, Message, Theme, Renderer> MenuBar<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    /// Creates a [`MenuBar`] with the given root items.
    pub fn new(mut roots: Vec<Item<'a, Message, Theme, Renderer>>) -> Self {
        for i in &mut roots {
            if let Some(m) = i.menu.as_mut() {
                m.axis = Axis::Vertical;
            }
        }

        Self {
            roots,
            spacing: Pixels::ZERO,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            global_parameters: GlobalParameters {
                check_bounds_width: 50.0,
                draw_path: DrawPath::FakeHovering,
                scroll_speed: ScrollSpeed {
                    line: 60.0,
                    pixel: 1.0,
                },
                close_on_click: false,
                class: Theme::default(),
            },
        }
    }

    /// Sets the width of the [`MenuBar`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`MenuBar`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the spacing of the [`MenuBar`].
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into();
        self
    }

    /// Sets the width of the check bounds of the [`Menu`]s in the [`MenuBar`].
    pub fn check_bounds_width(mut self, check_bounds_width: f32) -> Self {
        self.global_parameters.check_bounds_width = check_bounds_width;
        self
    }

    /// Sets the draw path option of the [`MenuBar`]
    pub fn draw_path(mut self, draw_path: DrawPath) -> Self {
        self.global_parameters.draw_path = draw_path;
        self
    }

    /// Sets the scroll speed of the [`Menu`]s in the [`MenuBar`]
    pub fn scroll_speed(mut self, scroll_speed: ScrollSpeed) -> Self {
        self.global_parameters.scroll_speed = scroll_speed;
        self
    }

    /// Sets the close on click option of the [`MenuBar`]
    pub fn close_on_click(mut self, close_on_click: bool) -> Self {
        self.global_parameters.close_on_click = close_on_click;
        self
    }

    /// Sets the padding of the [`MenuBar`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the style of the [`MenuBar`].
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.global_parameters.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Sets the class of the input of the [`MenuBar`].
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.global_parameters.class = class.into();
        self
    }
}
impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for MenuBar<'_, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<MenuBarState>()
    }

    fn state(&self) -> tree::State {
        tree::State::Some(Box::<MenuBarState>::default())
    }

    /// \[Tree{stateless, \[widget_state, menu_state]}...]
    fn children(&self) -> Vec<Tree> {
        self.roots.iter().map(Item::tree).collect::<Vec<_>>()
    }

    /// tree: Tree{bar, \[item_tree...]}
    fn diff(&self, tree: &mut Tree) {
        println!("MenuBar::diff()");
        tree.diff_children_custom(&self.roots, |tree, item| item.diff(tree), Item::tree);
    }

    /// tree: Tree{bar, \[item_tree...]}
    /// 
    /// out: Node{bar bounds , \[widget_layout, widget_layout, ...]}
    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        flex::resolve(
            flex::Axis::Horizontal,
            renderer,
            limits,
            self.width,
            self.height,
            self.padding,
            self.spacing,
            alignment::Alignment::Center,
            &self.roots.iter().map(|item| &item.item).collect::<Vec<_>>(),
            &mut tree
                .children
                .iter_mut()
                .map(|tree| &mut tree.children[0])
                .collect::<Vec<_>>(),
        )
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &event::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        println!("MenuBar::update()");
        update_items(
            self.roots.as_mut_slice(), 
            tree.children.as_mut_slice(), 
            layout.children(), 
            event, cursor, renderer, clipboard, shell, viewport
        );

        let Tree{ state, children: item_trees, .. } = tree;
        let bar = state.downcast_mut::<MenuBarState>();
        let MenuBarState { global_state, menu_state: bar_state } = bar;

        let bar_bounds = layout.bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(bar_bounds) {
                    global_state.pressed = true;
                    global_state.schedule(MenuBarTask::OpenOnClick);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                global_state.pressed = false;

                if let Some(task) = global_state.task {
                    match task {
                        MenuBarTask::OpenOnClick => {
                            bar.open(&mut self.roots, item_trees, layout.children(), cursor, shell);
                        }
                        MenuBarTask::CloseOnClick => {
                            bar.close(item_trees, shell);
                        }
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(bar_menu_state) = bar_state.as_mut() {
                    if cursor.is_over(bar_bounds) {
                        try_open_menu(
                            &mut self.roots,
                            bar_menu_state,
                            item_trees,
                            layout.children(),
                            cursor,
                            shell,
                            0,
                        );
                        shell.capture_event();
                    } else {
                        bar.close(item_trees, shell);
                    }
                    // println!("MenuBar::update() | CursorMoved | bar: {:?}", bar);
                }
            }
            _ => {}
        }
        
        println!("MenuBar::update() | return | bar: {:?}", bar);
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.roots
                .iter() // [Item...]
                .zip(tree.children.iter_mut()) // [item_tree...]
                .zip(layout.children()) // [widget_node...]
                .for_each(|((child, state), layout)| {
                    child.operate(state, layout, renderer, operation);
                });
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.roots
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((item, tree), layout)| item.mouse_interaction(tree, layout, cursor, renderer))
            .max()
            .unwrap_or_default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bar = tree.state.downcast_ref::<MenuBarState>();
        let MenuBarState { global_state, menu_state: bar_state } = bar;

        let styling = theme.style(&self.global_parameters.class, Status::Active);
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: styling.bar_border,
                shadow: styling.bar_shadow,
                ..Default::default()
            },
            global_state.pressed.then(|| Color::from([0.5; 3]).into())
                .unwrap_or(styling.bar_background),
            // styling.bar_background,
        );

        if let (DrawPath::Backdrop, Some(MenuState { active: Some(active), .. })) 
        = (&self.global_parameters.draw_path, bar_state.as_ref()) {
            let active_bounds = layout.children()
                .nth(*active)
                .expect(&format!("Index {:?} is not within the menu bar layout \
                    | layout.children().count(): {:?} \
                    | This should not happen, please report this issue
                    ",
                    active,
                    layout.children().count()
                ))
                .bounds();

            renderer.fill_quad(
                renderer::Quad {
                    bounds: active_bounds,
                    border: styling.path_border,
                    ..Default::default()
                },
                styling.path,
            );
        }

        self.roots
            .iter() // [Item...]
            .zip(tree.children.iter()) // [item_tree...]
            .zip(layout.children()) // [widget_node...]
            .for_each(|((item, tree), layout)| {
                item.draw(tree, renderer, theme, style, layout, cursor, viewport);
            });
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: iced::Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        println!("MenuBar::overlay()");
        let bar = tree.state.downcast_mut::<MenuBarState>();

        if bar.menu_state.is_some() {
            println!("MenuBar::overlay() | return | Menu Overlay");
            Some(
                MenuBarOverlay {
                    menu_bar: self,
                    layout,
                    translation,
                    tree,
                }
                .overlay_element(),
            )
        } else {
            println!("MenuBar::overlay() | state not open | try return root overlays");
            let overlays = self.roots.iter_mut()
                .zip(tree.children.iter_mut()) // [item_tree...]
                .zip(layout.children())
                .filter_map(|((item, item_tree), item_layout)| {
                    item.item.as_widget_mut().overlay(
                        &mut item_tree.children[0],
                        item_layout, 
                        renderer, 
                        viewport,
                        translation
                    )
                }).collect::<Vec<_>>();

            if overlays.is_empty(){
                println!("MenuBar::overlay() | return | None");
                None
            }else{
                println!("MenuBar::overlay() | return | Root Item Overlay");
                Some(overlay::Group::with_children(overlays).overlay())
            }
        }
    }
}
impl<'a, Message, Theme, Renderer> From<MenuBar<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + renderer::Renderer,
{
    fn from(value: MenuBar<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}

