//! A tree structure for constructing a hierarchical menu

use iced_widget::core::{renderer, Element};
/// Nested menu is essentially a tree of items, a menu is a collection of items
/// a menu itself can also be an item of another menu.
///
/// A `MenuTree` represents a node in the tree, it holds a widget as a menu item
/// for its parent, and a list of menu tree as child nodes.
/// Conceptually a node is either a menu(inner node) or an item(leaf node),
/// but there's no need to explicitly distinguish them here, if a menu tree
/// has children, it's a menu, otherwise it's an item
#[allow(missing_debug_implementations)]
pub struct MenuTree<'a, Message, Theme = iced_widget::Theme, Renderer = iced_widget::Renderer> {
    /// The menu tree will be flatten into a vector to build a linear widget tree,
    /// the `index` field is the index of the item in that vector
    pub(super) index: usize,

    /// The item of the menu tree
    pub(super) item: Element<'a, Message, Theme, Renderer>,
    /// The children of the menu tree
    pub(super) children: Vec<MenuTree<'a, Message, Theme, Renderer>>,
    /// The width of the menu tree
    pub(super) width: Option<u16>,
    /// The height of the menu tree
    pub(super) height: Option<u16>,
}
impl<'a, Message, Theme, Renderer> MenuTree<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Create a new menu tree from a widget
    pub fn new(item: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            index: 0,
            item: item.into(),
            children: Vec::new(),
            width: None,
            height: None,
        }
    }

    /// Create a menu tree from a widget and a vector of sub trees
    pub fn with_children(
        item: impl Into<Element<'a, Message, Theme, Renderer>>,
        children: Vec<impl Into<MenuTree<'a, Message, Theme, Renderer>>>,
    ) -> Self {
        Self {
            index: 0,
            item: item.into(),
            children: children.into_iter().map(Into::into).collect(),
            width: None,
            height: None,
        }
    }

    /// Sets the width of the menu tree.
    /// See [`ItemWidth`]
    ///
    /// [`ItemWidth`]:`super::ItemWidth`
    #[must_use]
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height of the menu tree.
    /// See [`ItemHeight`]
    ///
    /// [`ItemHeight`]: `super::ItemHeight`
    #[must_use]
    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    /* Keep `set_index()` and `flattern()` recurse in the same order */

    /// Set the index of each item
    pub(super) fn set_index(&mut self) {
        /// inner counting function.
        fn rec<Message, Theme, Renderer>(mt: &mut MenuTree<'_, Message, Theme, Renderer>, count: &mut usize) {
            // keep items under the same menu line up
            mt.children.iter_mut().for_each(|c| {
                c.index = *count;
                *count += 1;
            });

            mt.children.iter_mut().for_each(|c| rec(c, count));
        }

        let mut count = 0;
        self.index = count;
        count += 1;
        rec(self, &mut count);
    }

    /// Flatten the menu tree
    pub(super) fn flattern(&'a self) -> Vec<&Self> {
        /// Inner flattening function
        fn rec<'a, Message, Theme, Renderer>(
            mt: &'a MenuTree<'a, Message, Theme, Renderer>,
            flat: &mut Vec<&MenuTree<'a, Message, Theme, Renderer>>,
        ) {
            mt.children.iter().for_each(|c| {
                flat.push(c);
            });

            mt.children.iter().for_each(|c| {
                rec(c, flat);
            });
        }

        let mut flat = Vec::new();
        flat.push(self);
        rec(self, &mut flat);

        flat
    }
}

impl<'a, Message, Theme, Renderer> From<Element<'a, Message, Theme, Renderer>> for MenuTree<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(value: Element<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}
