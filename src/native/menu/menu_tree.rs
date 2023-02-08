use iced_native::{
    Element, renderer
};

/// Nested menu is essentially a tree of items, a menu is a collection of items
/// a menu itself can also be an item of another menu. 
/// 
/// A `MenuTree` represents a node in the tree, it holds a widget as a menu item 
/// for its parent, and a list of menu tree as child nodes. 
/// Conceptually a node is either a menu(inner node) or an item(leaf node), 
/// but there's no need to explicitly distinguish them here, if a menu tree 
/// has children, it's a menu, otherwise it's an item
#[allow(missing_debug_implementations)]
pub struct MenuTree<'a, Message, Renderer>{
    /// The menu tree will be flatten into a vector to build a linear widget tree, 
    /// the `index` field is the index of the item in that vector
    pub(super) index: usize,
    
    pub(super) item: Element<'a, Message, Renderer>,
    pub(super) children: Vec<MenuTree<'a, Message, Renderer>>
}
impl<'a, Message, Renderer> MenuTree<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Create a new menu tree from a widget
    pub fn new(
        item: impl Into<Element<'a, Message, Renderer>>, 
    ) -> Self{
        Self{
            index: 0,
            item: item.into(),
            children: Vec::new(),
        }
    }

    /// Create a menu tree from a widget and a vector of sub trees
    pub fn with_children(
        item: impl Into<Element<'a, Message, Renderer>>, 
        children: Vec<impl Into<MenuTree<'a, Message, Renderer>>>
    ) -> Self{
        Self{
            index: 0,
            item: item.into(),
            children: children.into_iter().map(|item| item.into() ).collect(),
        }
    }


    /* Keep `set_index()` and `flattern()` recurse in the same order */
    
    /// Set the index of each item
    pub(super) fn set_index(&mut self){
        fn rec<'a, Message, Renderer>(
            mt: &mut MenuTree<'a, Message, Renderer>, 
            count: &mut usize)
        {
            // keep items under the same menu line up
            mt.children.iter_mut().for_each(|c|{
                c.index = *count;
                *count += 1;
            });

            mt.children.iter_mut().for_each(|c|{
                rec(c, count)
            });
        }
        
        let mut count = 0;
        self.index = count;
        count += 1;
        rec(self, &mut count)
    }

    /// Flatten the menu tree
    pub(super) fn flattern(&'a self) -> Vec<&Self>{
        fn rec<'a, Message, Renderer>(
            mt: &'a MenuTree<'a, Message, Renderer>,
            flat: &mut Vec<&MenuTree<'a, Message, Renderer>>,
        ){
            mt.children.iter().for_each(|c|{
                flat.push(c);
            });

            mt.children.iter().for_each(|c|{
                rec(c, flat);
            });
        }
        
        let mut flat = Vec::new();
        flat.push(self);
        rec(self, &mut flat);

        flat
    }

}

impl<'a, Message, Renderer> From<Element<'a, Message, Renderer>> for MenuTree<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(value: Element<'a, Message, Renderer>) -> Self {
        Self::new(value)
    }
}