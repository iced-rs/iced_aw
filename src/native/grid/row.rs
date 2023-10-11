use iced_widget::core::Element;

/// A container that distributes its contents in a row of a [`crate::Grid`].
#[allow(missing_debug_implementations)]
pub struct GridRow<'a, Message, Renderer = crate::Renderer> {
    pub(crate) elements: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Default for GridRow<'a, Message, Renderer>
where
    Renderer: iced_widget::core::Renderer,
{
    fn default() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl<'a, Message, Renderer> GridRow<'a, Message, Renderer>
where
    Renderer: iced_widget::core::Renderer,
{
    /// Creates a new [`GridRow`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`GridRow`] with the given widgets.
    pub fn with_elements(children: Vec<impl Into<Element<'a, Message, Renderer>>>) -> Self {
        Self {
            elements: children.into_iter().map(|child| child.into()).collect(),
        }
    }

    /// Adds a widget to the [`GridRow`].
    pub fn push<E>(mut self, element: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.elements.push(element.into());
        self
    }
}
