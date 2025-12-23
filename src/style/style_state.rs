//! Helper enum for the state of the style

/// The state of the style
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StyleState {
    /// Use the active style
    Active,
    /// Use the selected style
    Selected,
    /// Use the hovered style
    Hovered,
    /// Use the focused style
    Focused,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn style_state_active_exists() {
        let state = StyleState::Active;
        assert_eq!(state, StyleState::Active);
    }

    #[test]
    fn style_state_selected_exists() {
        let state = StyleState::Selected;
        assert_eq!(state, StyleState::Selected);
    }

    #[test]
    fn style_state_hovered_exists() {
        let state = StyleState::Hovered;
        assert_eq!(state, StyleState::Hovered);
    }

    #[test]
    fn style_state_focused_exists() {
        let state = StyleState::Focused;
        assert_eq!(state, StyleState::Focused);
    }

    #[test]
    fn style_state_equality() {
        assert_eq!(StyleState::Active, StyleState::Active);
        assert_eq!(StyleState::Selected, StyleState::Selected);
        assert_eq!(StyleState::Hovered, StyleState::Hovered);
        assert_eq!(StyleState::Focused, StyleState::Focused);
    }

    #[test]
    fn style_state_inequality() {
        assert_ne!(StyleState::Active, StyleState::Selected);
        assert_ne!(StyleState::Active, StyleState::Hovered);
        assert_ne!(StyleState::Active, StyleState::Focused);
        assert_ne!(StyleState::Selected, StyleState::Hovered);
        assert_ne!(StyleState::Selected, StyleState::Focused);
        assert_ne!(StyleState::Hovered, StyleState::Focused);
    }

    #[test]
    fn style_state_debug() {
        let state = StyleState::Active;
        let debug_str = format!("{:?}", state);
        assert_eq!(debug_str, "Active");

        let state = StyleState::Selected;
        let debug_str = format!("{:?}", state);
        assert_eq!(debug_str, "Selected");

        let state = StyleState::Hovered;
        let debug_str = format!("{:?}", state);
        assert_eq!(debug_str, "Hovered");

        let state = StyleState::Focused;
        let debug_str = format!("{:?}", state);
        assert_eq!(debug_str, "Focused");
    }

    #[test]
    fn style_state_ordering() {
        // Verify the order based on enum definition
        assert!(StyleState::Active < StyleState::Selected);
        assert!(StyleState::Selected < StyleState::Hovered);
        assert!(StyleState::Hovered < StyleState::Focused);

        assert!(StyleState::Active <= StyleState::Active);
        assert!(StyleState::Active <= StyleState::Selected);
        assert!(StyleState::Focused >= StyleState::Active);
        assert!(StyleState::Focused >= StyleState::Focused);
    }

    #[test]
    fn style_state_hash() {
        // Verify that StyleState can be used as a HashMap key
        let mut map = HashMap::new();
        let _ = map.insert(StyleState::Active, "active");
        let _ = map.insert(StyleState::Selected, "selected");
        let _ = map.insert(StyleState::Hovered, "hovered");
        let _ = map.insert(StyleState::Focused, "focused");

        assert_eq!(map.get(&StyleState::Active), Some(&"active"));
        assert_eq!(map.get(&StyleState::Selected), Some(&"selected"));
        assert_eq!(map.get(&StyleState::Hovered), Some(&"hovered"));
        assert_eq!(map.get(&StyleState::Focused), Some(&"focused"));
    }

    #[test]
    fn all_style_states_are_different() {
        let states = [
            StyleState::Active,
            StyleState::Selected,
            StyleState::Hovered,
            StyleState::Focused,
        ];

        // Each state should be unique
        for (i, state1) in states.iter().enumerate() {
            for (j, state2) in states.iter().enumerate() {
                if i == j {
                    assert_eq!(state1, state2);
                } else {
                    assert_ne!(state1, state2);
                }
            }
        }
    }

    #[test]
    fn style_state_can_be_matched() {
        let state = StyleState::Active;
        let value = match state {
            StyleState::Active => 1,
            StyleState::Selected => 2,
            StyleState::Hovered => 3,
            StyleState::Focused => 4,
        };
        assert_eq!(value, 1);

        let state = StyleState::Focused;
        let value = match state {
            StyleState::Active => 1,
            StyleState::Selected => 2,
            StyleState::Hovered => 3,
            StyleState::Focused => 4,
        };
        assert_eq!(value, 4);
    }

    #[test]
    fn style_state_ord_comparison() {
        let mut states = vec![
            StyleState::Focused,
            StyleState::Active,
            StyleState::Hovered,
            StyleState::Selected,
        ];
        states.sort();

        assert_eq!(
            states,
            vec![
                StyleState::Active,
                StyleState::Selected,
                StyleState::Hovered,
                StyleState::Focused,
            ]
        );
    }
}
