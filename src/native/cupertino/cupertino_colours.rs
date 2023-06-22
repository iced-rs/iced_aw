use iced_native::Color;

/// <https://flutter.github.io/assets-for-api-docs/assets/cupertino/cupertino_system_colors_1.png>
/// <https://flutter.github.io/assets-for-api-docs/assets/cupertino/cupertino_system_colors_2.png>
/// <https://flutter.github.io/assets-for-api-docs/assets/cupertino/cupertino_system_colors_3.png>
///
/// Because iced expects `r`, `g`, and `b` to be between 0 and 1, divide by 255 everywhere.
// System Colours //

/// System Blue
#[must_use]
pub fn system_blue(alpha: f32) -> Color {
    Color::from_rgba(0.0 / 255.0, 122.0 / 255.0, 1.0, alpha)
}

/// System Green
#[must_use]
pub fn system_green(alpha: f32) -> Color {
    Color::from_rgba(52.0 / 255.0, 199.0 / 255.0, 89.0 / 255.0, alpha)
}

/// System Indigo
#[must_use]
pub fn system_indigo(alpha: f32) -> Color {
    Color::from_rgba(88.0 / 255.0, 86.0 / 255.0, 214.0 / 255.0, alpha)
}

/// System Orange
#[must_use]
pub fn system_orange(alpha: f32) -> Color {
    Color::from_rgba(1.0, 149.0 / 255.0, 0.0 / 255.0, alpha)
}

/// System Pink
#[must_use]
pub fn system_pink(alpha: f32) -> Color {
    Color::from_rgba(1.0, 45.0 / 255.0, 85.0 / 255.0, alpha)
}

/// System Purple
#[must_use]
pub fn system_purple(alpha: f32) -> Color {
    Color::from_rgba(175.0 / 255.0, 82.0 / 255.0, 222.0 / 255.0, alpha)
}

/// System Red
#[must_use]
pub fn system_red(alpha: f32) -> Color {
    Color::from_rgba(1.0, 59.0 / 255.0, 48.0 / 255.0, alpha)
}

/// System Teal
#[must_use]
pub fn system_teal(alpha: f32) -> Color {
    Color::from_rgba(90.0 / 255.0, 200.0 / 255.0, 250.0 / 255.0, alpha)
}

/// System Yellow
#[must_use]
pub fn system_yellow(alpha: f32) -> Color {
    Color::from_rgba(1.0, 204.0 / 255.0, 0.0 / 255.0, alpha)
}
//

/// Secondary System Fill
#[must_use]
pub fn secondary_system_fill() -> Color {
    Color::from_rgb(209.0 / 255.0, 209.0 / 255.0, 214.0 / 255.0)
}
