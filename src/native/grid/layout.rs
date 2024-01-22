use std::cmp::Ordering;

use iced_widget::core::{
    alignment::{Horizontal, Vertical},
    layout::{Limits, Node},
    widget::Tree,
    Length, Padding, Pixels, Point, Size,
};
use itertools::{Itertools, Position};

use super::types::GridRow;

#[allow(clippy::too_many_arguments)]
pub(super) fn layout<Message, Theme, Renderer>(
    tree: &mut Tree,
    renderer: &Renderer,
    limits: &Limits,
    column_count: usize,
    row_count: usize,
    element_count: usize,
    rows: &[GridRow<'_, Message, Theme, Renderer>],
    column_spacing: Pixels,
    row_spacing: Pixels,
    padding: Padding,
    horizontal_alignment: Horizontal,
    vertical_alignment: Vertical,
    width: Length,
    height: Length,
    column_lengths: &[Length],
    row_lengths: &[Length],
) -> Node
where
    Renderer: iced_widget::core::Renderer,
{
    let mut column_widths = Vec::<f32>::with_capacity(column_count);
    let mut row_heights = Vec::<f32>::with_capacity(row_count);

    // Measure the minimum row and column size to fit the contents
    minimum_row_column_sizes(tree, renderer, &mut column_widths, &mut row_heights, rows);

    // Adjust for fixed row and column sizes
    adjust_size_for_fixed_length(&mut column_widths, column_lengths);
    adjust_size_for_fixed_length(&mut row_heights, row_lengths);

    // Calculate grid limits
    let min_size = Size::new(
        total_length(&column_widths, column_spacing),
        total_length(&row_heights, row_spacing),
    );

    let grid_limits = limits
        .shrink(padding)
        .min_width(min_size.width)
        .min_height(min_size.height)
        .width(width)
        .height(height);

    //use to be grid_limits.fill();
    let grid_size = grid_limits.min();
    let grid_size = grid_limits.resolve(width, height, grid_size);

    // Allocate the available space
    let available_width = grid_size.width - total_spacing(column_count, column_spacing);
    let available_height = grid_size.height - total_spacing(row_count, row_spacing);
    allocate_space(&mut column_widths, column_lengths, available_width);
    allocate_space(&mut row_heights, row_lengths, available_height);

    // Lay out the widgets
    create_grid_layout(
        tree,
        element_count,
        rows,
        &row_heights,
        &column_widths,
        renderer,
        horizontal_alignment,
        vertical_alignment,
        column_spacing,
        row_spacing,
        padding,
        grid_size,
    )
}

fn minimum_row_column_sizes<Message, Theme, Renderer>(
    tree: &mut Tree,
    renderer: &Renderer,
    column_widths: &mut Vec<f32>,
    row_heights: &mut Vec<f32>,
    rows: &[GridRow<'_, Message, Theme, Renderer>],
) where
    Renderer: iced_widget::core::Renderer,
{
    let mut children = tree.children.iter_mut();
    for row in rows {
        let mut row_height = 0.0f32;

        for (col_idx, element) in row.elements.iter().enumerate() {
            let child_limits = Limits::NONE.width(Length::Shrink).height(Length::Shrink);
            let Size { width, height } = element
                .as_widget()
                .layout(
                    children.next().expect("grid missing expected child"),
                    renderer,
                    &child_limits,
                )
                .size();

            #[allow(clippy::option_if_let_else)]
            match column_widths.get_mut(col_idx) {
                Some(col_width) => *col_width = col_width.max(width),
                None => column_widths.insert(col_idx, width),
            }

            row_height = row_height.max(height);
        }
        row_heights.push(row_height);
    }
}

fn adjust_size_for_fixed_length(sizes: &mut [f32], length_settings: &[Length]) {
    for (size, lenght) in sizes.iter_mut().zip(length_settings.iter().cycle()) {
        if let Length::Fixed(value) = *lenght {
            *size = size.max(value);
        }
    }
}

fn total_length(element_sizes: &[f32], spacing: Pixels) -> f32 {
    let n_elements = element_sizes.len();
    element_sizes.iter().sum::<f32>() + total_spacing(n_elements, spacing)
}

fn total_spacing(element_count: usize, spacing: Pixels) -> f32 {
    element_count.saturating_sub(1) as f32 * spacing.0
}

fn allocate_space(current_sizes: &mut [f32], length_settings: &[Length], available_space: f32) {
    let mut fill_factor_sum = length_settings
        .iter()
        .cycle()
        .take(current_sizes.len())
        .map(Length::fill_factor)
        .sum::<u16>();

    if fill_factor_sum == 0 {
        return;
    }

    let mut space_to_divide = available_space;

    let sorted_iter = current_sizes
        .iter_mut()
        .zip(length_settings.iter().cycle())
        .sorted_by(|(&mut a_size, &a_length), (&mut b_size, &b_length)| {
            if a_length.fill_factor() == 0 {
                return Ordering::Less;
            } else if b_length.fill_factor() == 0 {
                return Ordering::Greater;
            }

            (b_size / f32::from(b_length.fill_factor()))
                .total_cmp(&(a_size / f32::from(a_length.fill_factor())))
        });

    for (size, length) in sorted_iter {
        let fill_factor = length.fill_factor();
        let fill_size = f32::from(fill_factor) / f32::from(fill_factor_sum) * space_to_divide;
        let new_size = size.max(fill_size);
        fill_factor_sum -= fill_factor;
        space_to_divide -= new_size;
        *size = new_size;
    }
}

#[allow(clippy::too_many_arguments)]
fn create_grid_layout<Message, Theme, Renderer>(
    tree: &mut Tree,
    element_count: usize,
    rows: &[GridRow<'_, Message, Theme, Renderer>],
    row_heights: &[f32],
    column_widths: &[f32],
    renderer: &Renderer,
    horizontal_alignment: Horizontal,
    vertical_alignment: Vertical,
    column_spacing: Pixels,
    row_spacing: Pixels,
    padding: Padding,
    grid_size: Size,
) -> Node
where
    Renderer: iced_widget::core::Renderer,
{
    let mut y = padding.top;
    let mut nodes = Vec::with_capacity(element_count);
    let mut children = tree.children.iter_mut();

    for (row_position, (row, &row_height)) in rows.iter().zip(row_heights).with_position() {
        let mut x = padding.left;
        for (col_position, (element, &column_width)) in
            row.elements.iter().zip(column_widths).with_position()
        {
            let widget = element.as_widget();
            let widget_size = widget.size();
            let widget_limits = Limits::NONE
                .width(widget_size.width)
                .height(widget_size.height)
                .max_width(column_width)
                .max_height(row_height);

            let node = widget
                .layout(
                    children.next().expect("Grid missing child"),
                    renderer,
                    &widget_limits,
                )
                .move_to(Point::new(x, y))
                .align(
                    horizontal_alignment.into(),
                    vertical_alignment.into(),
                    Size::new(column_width, row_height),
                );
            nodes.push(node);

            x += column_width;
            if not_last(col_position) {
                x += column_spacing.0;
            }
        }
        y += row_height;
        if not_last(row_position) {
            y += row_spacing.0;
        }
    }

    Node::with_children(grid_size, nodes)
}

fn not_last(position: Position) -> bool {
    position != Position::Last && position != Position::Only
}
