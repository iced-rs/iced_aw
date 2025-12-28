//! Integration tests for the CustomLayout widget
//!
//! These tests verify the CustomLayout widget's behavior and functionality
//! by actually exercising the widget through the iced test framework.

use iced::{Element, Length, Point, Settings, Size};
use iced_aw::widget::CustomLayout;
use iced_core::layout::{Limits, Node};
use iced_test::Simulator;
use iced_widget::text::Text;

// Message type for the tests (unused but required by iced)
#[derive(Clone, Debug)]
enum Message {}

type ViewFn = Box<dyn Fn() -> Element<'static, Message>>;

#[derive(Clone)]
struct App {
    view_fn: std::rc::Rc<ViewFn>,
}

impl App {
    fn new<F>(view_fn: F) -> (Self, iced::Task<Message>)
    where
        F: Fn() -> Element<'static, Message> + 'static,
    {
        (
            App {
                view_fn: std::rc::Rc::new(Box::new(view_fn)),
            },
            iced::Task::none(),
        )
    }

    fn view(&self) -> Element<'_, Message> {
        (self.view_fn)()
    }
}

/// Helper to run a test with a given view
fn run_test<F>(view_fn: F)
where
    F: Fn() -> Element<'static, Message> + 'static,
{
    let (app, _) = App::new(view_fn);
    let _ui = Simulator::with_settings(Settings::default(), app.view());
    // The widget is successfully rendered if we get here without panicking
}

#[test]
fn custom_layout_with_fixed_size_renders() {
    run_test(|| {
        let elements = vec![Text::new("Test").into()];
        CustomLayout::new(elements, |_, _, _, _| Node::new(Size::new(200.0, 100.0))).into()
    })
}

#[test]
fn custom_layout_with_single_child_renders() {
    run_test(|| {
        let elements = vec![Text::new("Child").into()];
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                child.move_to_mut(Point::new(10.0, 20.0));
                children.push(child);
            }
            Node::with_children(Size::new(200.0, 100.0), children)
        })
        .into()
    })
}

#[test]
fn custom_layout_with_horizontal_layout_renders() {
    run_test(|| {
        let elements = vec![
            Text::new("A").into(),
            Text::new("B").into(),
            Text::new("C").into(),
        ];
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            let mut x_offset = 0.0;
            let spacing = 10.0;

            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                child.move_to_mut(Point::new(x_offset, 0.0));
                x_offset += child.size().width + spacing;
                children.push(child);
            }
            Node::with_children(Size::new(x_offset, 50.0), children)
        })
        .into()
    })
}

#[test]
fn custom_layout_with_vertical_layout_renders() {
    run_test(|| {
        let elements = vec![
            Text::new("Line 1").into(),
            Text::new("Line 2").into(),
            Text::new("Line 3").into(),
        ];
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            let mut y_offset = 0.0;
            let spacing = 5.0;

            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                child.move_to_mut(Point::new(0.0, y_offset));
                y_offset += child.size().height + spacing;
                children.push(child);
            }
            Node::with_children(Size::new(200.0, y_offset), children)
        })
        .into()
    })
}

#[test]
fn custom_layout_with_grid_layout_renders() {
    run_test(|| {
        let elements = vec![
            Text::new("1").into(),
            Text::new("2").into(),
            Text::new("3").into(),
            Text::new("4").into(),
        ];
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            let cols = 2;
            let cell_width = 100.0;
            let cell_height = 50.0;

            for (i, (element, tree)) in elements.iter_mut().zip(trees.iter_mut()).enumerate() {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);

                let col = i % cols;
                let row = i / cols;
                let x = col as f32 * cell_width;
                let y = row as f32 * cell_height;

                child.move_to_mut(Point::new(x, y));
                children.push(child);
            }
            Node::with_children(Size::new(200.0, 100.0), children)
        })
        .into()
    })
}

#[test]
fn custom_layout_with_centered_element_renders() {
    run_test(|| {
        let elements = vec![Text::new("Centered").into()];
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            if let (Some(element), Some(tree)) = (elements.first_mut(), trees.first_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);

                let container_size = Size::new(200.0, 100.0);
                let child_size = child.size();
                let x = (container_size.width - child_size.width) / 2.0;
                let y = (container_size.height - child_size.height) / 2.0;

                child.move_to_mut(Point::new(x, y));
                Node::with_children(container_size, vec![child])
            } else {
                Node::new(Size::new(200.0, 100.0))
            }
        })
        .into()
    })
}

#[test]
fn custom_layout_with_empty_elements_renders() {
    run_test(|| {
        let elements = vec![];
        CustomLayout::new(elements, |_, _, _, _| Node::new(Size::new(100.0, 50.0))).into()
    })
}

#[test]
fn custom_layout_with_constrained_children_renders() {
    run_test(|| {
        let elements = vec![
            Text::new("Constrained 1").into(),
            Text::new("Constrained 2").into(),
        ];
        CustomLayout::new(elements, |elements, trees, renderer, _limits| {
            let mut children = Vec::new();
            let max_child_width = 100.0;
            let max_child_height = 50.0;

            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                let child_limits =
                    Limits::new(Size::ZERO, Size::new(max_child_width, max_child_height));
                let child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                children.push(child);
            }
            Node::with_children(Size::new(200.0, 100.0), children)
        })
        .into()
    })
}

#[test]
fn custom_layout_with_width_setting_renders() {
    run_test(|| {
        let elements = vec![Text::new("Test").into()];
        CustomLayout::new(elements, |_, _, _, _| Node::new(Size::new(100.0, 50.0)))
            .width(Length::Fixed(300.0))
            .into()
    })
}

#[test]
fn custom_layout_with_height_setting_renders() {
    run_test(|| {
        let elements = vec![Text::new("Test").into()];
        CustomLayout::new(elements, |_, _, _, _| Node::new(Size::new(100.0, 50.0)))
            .height(Length::Fixed(200.0))
            .into()
    })
}

#[test]
fn custom_layout_with_overlapping_elements_renders() {
    run_test(|| {
        let elements = vec![
            Text::new("Background").into(),
            Text::new("Foreground").into(),
        ];
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                child.move_to_mut(Point::new(50.0, 50.0));
                children.push(child);
            }
            Node::with_children(Size::new(200.0, 200.0), children)
        })
        .into()
    })
}

#[test]
fn custom_layout_with_responsive_behavior_renders() {
    run_test(|| {
        let elements = vec![Text::new("Item 1").into(), Text::new("Item 2").into()];
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            let available_width = limits.max().width;
            let spacing = if available_width > 400.0 { 20.0 } else { 10.0 };

            let mut x_offset = 0.0;
            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                child.move_to_mut(Point::new(x_offset, 0.0));
                x_offset += child.size().width + spacing;
                children.push(child);
            }
            Node::with_children(Size::new(x_offset, 50.0), children)
        })
        .into()
    })
}

#[test]
fn custom_layout_with_zero_size_renders() {
    run_test(|| {
        let elements = vec![Text::new("Zero").into()];
        CustomLayout::new(elements, |_, _, _, _| Node::new(Size::new(0.0, 0.0))).into()
    })
}

#[test]
fn custom_layout_with_absolute_positioning_renders() {
    run_test(|| {
        let elements = vec![
            Text::new("Top-left").into(),
            Text::new("Bottom-right").into(),
            Text::new("Center").into(),
        ];
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            let positions = [
                Point::new(0.0, 0.0),
                Point::new(150.0, 150.0),
                Point::new(100.0, 100.0),
            ];

            for ((element, tree), position) in elements
                .iter_mut()
                .zip(trees.iter_mut())
                .zip(positions.iter())
            {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                child.move_to_mut(*position);
                children.push(child);
            }
            Node::with_children(Size::new(200.0, 200.0), children)
        })
        .into()
    })
}

#[test]
fn custom_layout_with_different_widget_types_renders() {
    use iced_widget::Button;

    run_test(|| {
        let elements = vec![
            Text::new("Text element").into(),
            Button::new(Text::new("Button")).into(),
            Text::new("Another text").into(),
        ];
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            let mut y_offset = 0.0;

            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                child.move_to_mut(Point::new(0.0, y_offset));
                y_offset += child.size().height + 10.0;
                children.push(child);
            }
            Node::with_children(Size::new(300.0, y_offset), children)
        })
        .into()
    })
}
