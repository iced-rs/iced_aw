//! Integration tests for the CustomLayout widget
//!
//! These tests verify the CustomLayout widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

use iced::{Length, Point, Size, Theme};
use iced_aw::widget::CustomLayout;
use iced_core::layout::{Limits, Node};
use iced_widget::Renderer;
use iced_widget::text::Text;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
enum Message {
    Action1,
    Action2,
}

#[test]
fn custom_layout_can_be_created_with_empty_elements() {
    let elements = vec![];
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(0.0, 0.0))
        });
}

#[test]
fn custom_layout_can_be_created_with_single_element() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Hello").into()];
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        });
}

#[test]
fn custom_layout_can_be_created_with_multiple_elements() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = vec![
        Text::new("First").into(),
        Text::new("Second").into(),
        Text::new("Third").into(),
    ];
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(300.0, 100.0))
        });
}

#[test]
fn custom_layout_can_set_width() {
    let _layout1: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .width(200);

    let _layout2: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .width(Length::Fill);

    let _layout3: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .width(Length::Shrink);
}

#[test]
fn custom_layout_can_set_height() {
    let _layout1: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .height(150);

    let _layout2: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .height(Length::Fill);

    let _layout3: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .height(Length::Shrink);
}

#[test]
fn custom_layout_can_chain_width_and_height() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Test").into()];
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .width(200)
        .height(150);
}

#[test]
fn custom_layout_with_different_width_types() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Test").into()];
    let _layout1: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .width(Length::Fixed(300.0));

    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Test").into()];
    let _layout2: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .width(Length::Fill);

    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Test").into()];
    let _layout3: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .width(Length::Shrink);

    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Test").into()];
    let _layout4: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .width(Length::FillPortion(3));
}

#[test]
fn custom_layout_with_different_height_types() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Test").into()];
    let _layout1: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .height(Length::Fixed(200.0));

    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Test").into()];
    let _layout2: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .height(Length::Fill);

    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Test").into()];
    let _layout3: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .height(Length::Shrink);

    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Test").into()];
    let _layout4: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        })
        .height(Length::FillPortion(2));
}

#[test]
fn custom_layout_converts_to_element() {
    use iced::Element;
    use iced_widget::Renderer;

    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Test").into()];
    let layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        });
    let _element: Element<Message, Theme, Renderer> = layout.into();
}

#[test]
fn custom_layout_with_simple_layout_function() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Item 1").into(), Text::new("Item 2").into()];

    // Simple layout: just return a fixed size
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(200.0, 100.0))
        });
}

#[test]
fn custom_layout_with_layout_using_limits() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Test").into()];

    // Layout that uses the limits parameter
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _, limits| {
            let max_size = limits.max();
            Node::new(max_size)
        });
}

#[test]
fn custom_layout_with_layout_positioning_children() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Item 1").into(), Text::new("Item 2").into()];

    // Layout that positions children
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();

            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                child.move_to_mut(Point::new(0.0, 0.0));
                children.push(child);
            }

            Node::with_children(Size::new(200.0, 100.0), children)
        });
}

#[test]
fn custom_layout_with_horizontal_layout() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = vec![
        Text::new("A").into(),
        Text::new("B").into(),
        Text::new("C").into(),
    ];

    // Horizontal layout
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            let mut x_offset = 0.0;

            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                child.move_to_mut(Point::new(x_offset, 0.0));
                x_offset += child.size().width + 10.0; // Add spacing
                children.push(child);
            }

            Node::with_children(Size::new(x_offset, 50.0), children)
        });
}

#[test]
fn custom_layout_with_vertical_layout() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = vec![
        Text::new("Line 1").into(),
        Text::new("Line 2").into(),
        Text::new("Line 3").into(),
    ];

    // Vertical layout
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            let mut y_offset = 0.0;

            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                child.move_to_mut(Point::new(0.0, y_offset));
                y_offset += child.size().height + 5.0; // Add spacing
                children.push(child);
            }

            Node::with_children(Size::new(200.0, y_offset), children)
        });
}

#[test]
fn custom_layout_with_grid_layout() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = vec![
        Text::new("1").into(),
        Text::new("2").into(),
        Text::new("3").into(),
        Text::new("4").into(),
    ];

    // 2x2 grid layout
    let _layout: CustomLayout<Message, Theme, Renderer> =
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
        });
}

#[test]
fn custom_layout_with_overlapping_elements() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = vec![
        Text::new("Background").into(),
        Text::new("Foreground").into(),
    ];

    // Overlapping layout (both at same position)
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();

            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                child.move_to_mut(Point::new(50.0, 50.0)); // All at same position
                children.push(child);
            }

            Node::with_children(Size::new(200.0, 200.0), children)
        });
}

#[test]
fn custom_layout_supports_multiple_instances() {
    let elements1: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Layout 1").into()];
    let _layout1: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements1, |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        });

    let elements2: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Layout 2").into()];
    let _layout2: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements2, |_, _, _: &Renderer, _| {
            Node::new(Size::new(200.0, 100.0))
        });

    let elements3: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Layout 3").into()];
    let _layout3: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements3, |_, _, _: &Renderer, _| {
            Node::new(Size::new(150.0, 75.0))
        });
}

#[test]
fn custom_layout_with_different_message_types() {
    #[derive(Clone)]
    #[allow(dead_code)]
    enum CustomMessage {
        Custom1,
        Custom2,
    }

    let elements: Vec<iced_core::Element<CustomMessage, Theme, Renderer>> =
        vec![Text::new("Test").into()];
    let _layout: CustomLayout<CustomMessage, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 50.0))
        });
}

#[test]
fn custom_layout_with_unicode_content() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = vec![
        Text::new("你好").into(),
        Text::new("مرحبا").into(),
        Text::new("🎨🖼️").into(),
    ];

    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(200.0, 150.0))
        });
}

#[test]
fn custom_layout_with_long_text() {
    let long_text = "This is a very long text that might be used to test how the custom layout handles elements with longer content";
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new(long_text).into()];

    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(500.0, 100.0))
        });
}

#[test]
fn custom_layout_create_multiple_with_different_configs() {
    for i in 0..5 {
        let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
            vec![Text::new(format!("Element {}", i)).into()];

        let width = 100 + (i * 50);
        let height = 50 + (i * 25);

        let _layout: CustomLayout<Message, Theme, Renderer> =
            CustomLayout::new(elements, move |_, _, _: &Renderer, _| {
                Node::new(Size::new(width as f32, height as f32))
            })
            .width(width)
            .height(height);
    }
}

#[test]
fn custom_layout_with_varying_element_counts() {
    for count in 0..10 {
        let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = (0..count)
            .map(|i| Text::new(format!("Item {}", i)).into())
            .collect();

        let _layout: CustomLayout<Message, Theme, Renderer> =
            CustomLayout::new(elements, |_, _, _: &Renderer, _| {
                Node::new(Size::new(200.0, 100.0))
            });
    }
}

#[test]
fn custom_layout_with_extreme_dimensions() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Small").into()];
    let _layout_tiny: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(vec![Text::new("Test").into()], |_, _, _: &Renderer, _| {
            Node::new(Size::new(1.0, 1.0))
        })
        .width(10)
        .height(10);

    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Large").into()];
    let _layout_large: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(10000.0, 10000.0))
        })
        .width(5000)
        .height(5000);
}

#[test]
fn custom_layout_with_zero_size() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Zero").into()];
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(0.0, 0.0))
        });
}

#[test]
fn custom_layout_chaining_all_methods() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("First").into(), Text::new("Second").into()];

    let _layout: CustomLayout<Message, Theme, Renderer> =
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
        .width(400)
        .height(200);
}

#[test]
fn custom_layout_with_centered_element() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> =
        vec![Text::new("Centered").into()];

    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            if let (Some(element), Some(tree)) = (elements.first_mut(), trees.first_mut()) {
                let child_limits = Limits::new(Size::ZERO, limits.max());
                let mut child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);

                // Center the element
                let container_size = Size::new(200.0, 100.0);
                let child_size = child.size();
                let x = (container_size.width - child_size.width) / 2.0;
                let y = (container_size.height - child_size.height) / 2.0;

                child.move_to_mut(Point::new(x, y));

                Node::with_children(container_size, vec![child])
            } else {
                Node::new(Size::new(200.0, 100.0))
            }
        });
}

#[test]
fn custom_layout_with_constrained_child_sizes() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = vec![
        Text::new("Constrained 1").into(),
        Text::new("Constrained 2").into(),
    ];

    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |elements, trees, renderer, _limits| {
            let mut children = Vec::new();
            let max_width = 100.0;
            let max_height = 50.0;

            for (element, tree) in elements.iter_mut().zip(trees.iter_mut()) {
                // Constrain each child to a specific size
                let child_limits = Limits::new(Size::ZERO, Size::new(max_width, max_height));
                let child = element
                    .as_widget_mut()
                    .layout(tree, renderer, &child_limits);
                children.push(child);
            }

            Node::with_children(Size::new(200.0, 100.0), children)
        });
}

#[test]
fn custom_layout_with_responsive_layout() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = vec![
        Text::new("Responsive 1").into(),
        Text::new("Responsive 2").into(),
    ];

    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            let available_width = limits.max().width;

            // Adjust layout based on available width
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
        });
}

#[test]
fn custom_layout_with_different_element_types() {
    use iced_widget::Button;

    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = vec![
        Text::new("Text element").into(),
        Button::new(Text::new("Button")).into(),
        Text::new("Another text").into(),
    ];

    let _layout: CustomLayout<Message, Theme, Renderer> =
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
        });
}

#[test]
fn custom_layout_with_absolute_positioning() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = vec![
        Text::new("Top-left").into(),
        Text::new("Bottom-right").into(),
        Text::new("Center").into(),
    ];

    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |elements, trees, renderer, limits| {
            let mut children = Vec::new();
            let positions = [
                Point::new(0.0, 0.0),     // Top-left
                Point::new(150.0, 150.0), // Bottom-right
                Point::new(100.0, 100.0), // Center
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
        });
}

#[test]
fn custom_layout_empty_with_non_zero_size() {
    let elements: Vec<iced_core::Element<Message, Theme, Renderer>> = vec![];
    let _layout: CustomLayout<Message, Theme, Renderer> =
        CustomLayout::new(elements, |_, _, _: &Renderer, _| {
            Node::new(Size::new(100.0, 100.0))
        })
        .width(100)
        .height(100);
}
