//! Displays a [`Tabs`](Tabs) widget to select the content to be displayed.
//!
//! This is a wrapper around the [`TabBar`](super::tab_bar::TabBar) widget.
//! Unlike the [`TabBar`](super::tab_bar::TabBar) widget it will also handle
//! the content of the tabs.
//!
//! *This API requires the following crate features to be activated: tabs*
use iced_graphics::{backend, Backend, Primitive, Renderer};

pub use crate::style::tab_bar::{Style, StyleSheet};
use crate::{core::renderer::DrawEnvironment, native::tabs};
use iced_native::mouse;
pub use tabs::tab_bar_position::TabBarPosition;

/// A [`Tabs`](Tabs) widget for showing a [`TabBar`](super::tab_bar::TabBar)
/// along with the tab's content.
///
/// This is an alias of an `iced_native` Tabs widget with an
/// `iced_wgpu::Renderer`.
pub type Tabs<'a, Message, Backend> = tabs::Tabs<'a, Message, Renderer<Backend>>;

impl<B> tabs::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, ()>,
        active_tab: usize,
        tab_bar: Self::Output,
        tabs: &[iced_native::Element<'_, Message, Self>],
        tab_bar_position: &TabBarPosition,
    ) -> Self::Output {
        //let bounds = layout.bounds();
        let mut children = env.layout.children();
        /*let (tab_bar_layout, tab_content_layout) = match tab_bar_position {
            TabBarPosition::Top => {
                let tab_bar_layout = children.next().unwrap();
                let tab_content_layout = children.next().unwrap();
                (tab_bar_layout, tab_content_layout)
            }
            TabBarPosition::Bottom => {
                let tab_content_layout = children.next().unwrap();
                let tab_bar_layout = children.next().unwrap();
                (tab_bar_layout, tab_content_layout)
            }
        };*/
        let tab_content_layout = match tab_bar_position {
            TabBarPosition::Top => children.last().unwrap(),
            TabBarPosition::Bottom => children.next().unwrap(),
        };

        let mut primitives = Vec::new();
        let mut mouse_interaction = mouse::Interaction::default();

        let (tab_bar_primitive, new_mouse_interaction) = tab_bar;

        if new_mouse_interaction > mouse_interaction {
            mouse_interaction = new_mouse_interaction;
        }

        primitives.push(tab_bar_primitive);

        if let Some(element) = tabs.get(active_tab) {
            let (tab_content_primitive, new_mouse_interaction) = element.draw(
                self,
                env.defaults,
                tab_content_layout,
                env.cursor_position,
                env.viewport.unwrap(),
            );

            if new_mouse_interaction > mouse_interaction {
                mouse_interaction = new_mouse_interaction;
            }

            primitives.push(tab_content_primitive);
        }

        (Primitive::Group { primitives }, mouse_interaction)
    }
}
