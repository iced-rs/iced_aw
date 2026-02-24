//! Status Enum of an mouse Event.
//!
/// The Status of a widget event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// can be pressed.
    Active,
    /// can be pressed and it is being hovered.
    Hovered,
    /// is being pressed.
    Pressed,
    /// cannot be pressed.
    Disabled,
    /// is focused.
    Focused,
    /// is Selected.
    Selected,
}

/// The style function of widget.
pub type StyleFn<'a, Theme, Style> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_active_exists() {
        let status = Status::Active;
        assert_eq!(status, Status::Active);
    }

    #[test]
    fn status_hovered_exists() {
        let status = Status::Hovered;
        assert_eq!(status, Status::Hovered);
    }

    #[test]
    fn status_pressed_exists() {
        let status = Status::Pressed;
        assert_eq!(status, Status::Pressed);
    }

    #[test]
    fn status_disabled_exists() {
        let status = Status::Disabled;
        assert_eq!(status, Status::Disabled);
    }

    #[test]
    fn status_focused_exists() {
        let status = Status::Focused;
        assert_eq!(status, Status::Focused);
    }

    #[test]
    fn status_selected_exists() {
        let status = Status::Selected;
        assert_eq!(status, Status::Selected);
    }

    #[test]
    fn status_equality() {
        assert_eq!(Status::Active, Status::Active);
        assert_eq!(Status::Hovered, Status::Hovered);
        assert_eq!(Status::Pressed, Status::Pressed);
        assert_eq!(Status::Disabled, Status::Disabled);
        assert_eq!(Status::Focused, Status::Focused);
        assert_eq!(Status::Selected, Status::Selected);
    }

    #[test]
    fn status_inequality() {
        assert_ne!(Status::Active, Status::Hovered);
        assert_ne!(Status::Active, Status::Pressed);
        assert_ne!(Status::Active, Status::Disabled);
        assert_ne!(Status::Active, Status::Focused);
        assert_ne!(Status::Active, Status::Selected);
        assert_ne!(Status::Hovered, Status::Pressed);
        assert_ne!(Status::Hovered, Status::Disabled);
        assert_ne!(Status::Disabled, Status::Focused);
    }

    #[test]
    fn status_clone() {
        let status = Status::Active;
        let cloned = status;
        assert_eq!(status, cloned);
    }

    #[test]
    fn status_copy() {
        let status = Status::Active;
        let copied = status;
        assert_eq!(status, copied);
    }

    #[test]
    fn status_debug() {
        let status = Status::Active;
        let debug_str = format!("{status:?}");
        assert_eq!(debug_str, "Active");

        let status = Status::Hovered;
        let debug_str = format!("{status:?}");
        assert_eq!(debug_str, "Hovered");
    }

    #[test]
    fn all_statuses_are_different() {
        let statuses = [
            Status::Active,
            Status::Hovered,
            Status::Pressed,
            Status::Disabled,
            Status::Focused,
            Status::Selected,
        ];

        // Each status should be unique
        for (i, status1) in statuses.iter().enumerate() {
            for (j, status2) in statuses.iter().enumerate() {
                if i == j {
                    assert_eq!(status1, status2);
                } else {
                    assert_ne!(status1, status2);
                }
            }
        }
    }

    #[test]
    fn style_fn_type_compiles() {
        // This test verifies that the StyleFn type alias is correctly defined
        // and can be used with a simple function
        fn example_style_fn(_theme: &(), status: Status) -> u32 {
            match status {
                Status::Active => 1,
                Status::Hovered => 2,
                Status::Pressed => 3,
                Status::Disabled => 4,
                Status::Focused => 5,
                Status::Selected => 6,
            }
        }

        let style_fn: StyleFn<(), u32> = Box::new(example_style_fn);
        assert_eq!(style_fn(&(), Status::Active), 1);
        assert_eq!(style_fn(&(), Status::Hovered), 2);
        assert_eq!(style_fn(&(), Status::Pressed), 3);
        assert_eq!(style_fn(&(), Status::Disabled), 4);
        assert_eq!(style_fn(&(), Status::Focused), 5);
        assert_eq!(style_fn(&(), Status::Selected), 6);
    }
}
