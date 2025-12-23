//! Predefined color palette based on the CSS color palette
//!
//!
//! Thanks to:
//! * [W3 Schools](https://www.w3schools.com/cssref/css_colors.asp)
//! * [Corecoding](https://corecoding.com/utilities/rgb-or-hex-to-float.php)

use iced_core::Color;

/// Primary <span style="color:dodgerblue">Color</span>.
pub const PRIMARY: Color = DODGER_BLUE;

/// Secondary <span style="color:dimgray">Color</span>.
pub const SECONDARY: Color = DIM_GRAY;

/// Success <span style="color:limeGreen">Color</span>.
pub const SUCCESS: Color = LIME_GREEN;

/// Danger <span style="color:red">Color</span>.
pub const DANGER: Color = RED;

/// Warning <span style="color:gold">Color</span>.
pub const WARNING: Color = GOLD;

/// Info <span style="color:skyBlue">Color</span>.
pub const INFO: Color = SKY_BLUE;

/// Light <span style="color:ghostWhite">Color</span>.
pub const LIGHT: Color = GHOST_WHITE;

/// Dark <span style="color:rgb(0.204, 0.227, 0.251)">Color</span>.
pub const DARK: Color = Color::from_rgb(0.204, 0.227, 0.251);

/// Alice Blue <span style="color:aliceBlue">Color</span>.
pub const ALICE_BLUE: Color = Color::from_rgb(0.941, 0.973, 1.0);

/// Antique White <span style="color:antiqueWhite">Color</span>.
pub const ANTIQUE_WHITE: Color = Color::from_rgb(0.98, 0.922, 0.843);

/// Aqua <span style="color:aqua">Color</span>.
pub const AQUA: Color = Color::from_rgb(0.0, 1.0, 1.0);

/// Aquamarine <span style="color:aquamarine">Color</span>.
pub const AQUAMARINE: Color = Color::from_rgb(0.498, 1.0, 0.831);

/// Azure <span style="color:azure">Color</span>.
pub const AZURE: Color = Color::from_rgb(0.941, 1.0, 1.0);

/// Beige <span style="color:beige">Color</span>.
pub const BEIGE: Color = Color::from_rgb(0.961, 0.961, 0.863);

/// Bisque <span style="color:bisque">Color</span>.
pub const BISQUE: Color = Color::from_rgb(1.0, 0.894, 0.769);

/// Black <span style="color:black">Color</span>.
pub const BLACK: Color = Color::BLACK;

/// Blanched Almond <span style="color:blanchedAlmond">Color</span>.
pub const BLANCHED_ALMOND: Color = Color::from_rgb(1.0, 0.922, 0.804);

/// Blue <span style="color:blue">Color</span>.
pub const BLUE: Color = Color::from_rgb(0.0, 0.0, 1.0);

/// Blue Violet <span style="color:blueViolet">Color</span>.
pub const BLUE_VIOLET: Color = Color::from_rgb(0.541, 0.169, 0.886);

/// Brown <span style="color:brown">Color</span>.
pub const BROWN: Color = Color::from_rgb(0.647, 0.165, 0.165);

/// Burly Wood <span style="color:burlyWood">Color</span>.
pub const BURLY_WOOD: Color = Color::from_rgb(0.871, 0.722, 0.529);

/// Cadet Blue <span style="color:cadetBlue">Color</span>.
pub const CADET_BLUE: Color = Color::from_rgb(0.373, 0.62, 0.627);

/// Chartreuse <span style="color:chartreuse">Color</span>.
pub const CHARTREUSE: Color = Color::from_rgb(0.498, 1.0, 0.0);

/// Chocolate <span style="color:chocolate">Color</span>.
pub const CHOCOLATE: Color = Color::from_rgb(0.824, 0.412, 0.118);

/// Coral <span style="color:coral">Color</span>.
pub const CORAL: Color = Color::from_rgb(1.0, 0.498, 0.314);

/// Cornflower Blue <span style="color:cornflowerBlue">Color</span>.
pub const CORNFLOWER_BLUE: Color = Color::from_rgb(0.392, 0.584, 0.929);

/// Cornsilk <span style="color:cornsilk">Color</span>.
pub const CORNSILK: Color = Color::from_rgb(1.0, 0.973, 0.863);

/// Crimson <span style="color:crimson">Color</span>.
pub const CRIMSON: Color = Color::from_rgb(0.863, 0.078, 0.235);

/// Cyan <span style="color:cyan">Color</span>.
pub const CYAN: Color = Color::from_rgb(0.0, 1.0, 1.0);

/// Dark Blue <span style="color:darkBlue">Color</span>.
pub const DARK_BLUE: Color = Color::from_rgb(0.0, 0.0, 0.545);

/// Dark Cyan <span style="color:darkCyan">Color</span>.
pub const DARK_CYAN: Color = Color::from_rgb(0.0, 0.545, 0.545);

/// Dark Golden Rod <span style="color:darkGoldenRod">Color</span>.
pub const DARK_GOLDEN_ROD: Color = Color::from_rgb(0.722, 0.525, 0.043);

/// Dark Gray <span style="color:darkGray">Color</span>.
pub const DARK_GRAY: Color = Color::from_rgb(0.663, 0.663, 0.663);

/// Dark Grey <span style="color:darkGrey">Color</span>.
pub const DARK_GREY: Color = DARK_GRAY;

/// Dark Green <span style="color:darkGreen">Color</span>.
pub const DARK_GREEN: Color = Color::from_rgb(0.0, 0.392, 0.0);

/// Dark Khaki <span style="color:darkKhaki">Color</span>.
pub const DARK_KHAKI: Color = Color::from_rgb(0.741, 0.718, 0.42);

/// Dark Magenta <span style="color:darkMagenta">Color</span>.
pub const DARK_MAGENTA: Color = Color::from_rgb(0.545, 0.0, 0.545);

/// Dark Olive Green <span style="color:darkOliveGreen">Color</span>.
pub const DARK_OLIVE_GREEN: Color = Color::from_rgb(0.333, 0.42, 0.184);

/// Dark Orange <span style="color:darkOrange">Color</span>.
pub const DARK_ORANGE: Color = Color::from_rgb(1.0, 0.549, 0.0);

/// Dark Orchid <span style="color:darkOrchid">Color</span>.
pub const DARK_ORCHID: Color = Color::from_rgb(0.6, 0.196, 0.8);

/// Dark Red <span style="color:darkRed">Color</span>.
pub const DARK_RED: Color = Color::from_rgb(0.545, 0.0, 0.0);

/// Dark Salmon <span style="color:darkSalmon">Color</span>.
pub const DARK_SALMON: Color = Color::from_rgb(0.914, 0.588, 0.478);

/// Dark Sea Green <span style="color:darkSeaGreen">Color</span>.
pub const DARK_SEA_GREEN: Color = Color::from_rgb(0.561, 0.737, 0.561);

/// Dark Slate Blue <span style="color:darkSlateBlue">Color</span>.
pub const DARK_SLATE_BLUE: Color = Color::from_rgb(0.282, 0.239, 0.545);

/// Dark Slate Gray <span style="color:darkSlateGray">Color</span>.
pub const DARK_SLATE_GRAY: Color = Color::from_rgb(0.184, 0.31, 0.31);

/// Dark Slate Grey <span style="color:darkSlateGrey">Color</span>.
pub const DARK_SLATE_GREY: Color = DARK_SLATE_GRAY;

/// Dark Turquoise <span style="color:darkTurquoise">Color</span>.
pub const DARK_TURQUOISE: Color = Color::from_rgb(0.0, 0.808, 0.82);

/// Dark Violet <span style="color:darkViolet">Color</span>.
pub const DARK_VIOLET: Color = Color::from_rgb(0.58, 0.0, 0.827);

/// Deep Pink <span style="color:deepPink">Color</span>.
pub const DEEP_PINK: Color = Color::from_rgb(1.0, 0.078, 0.576);

/// Deep Sky Blue <span style="color:deepSkyBlue">Color</span>.
pub const DEEP_SKY_BLUE: Color = Color::from_rgb(0.0, 0.749, 1.0);

/// Dim Gray <span style="color:dimgray">Color</span>.
pub const DIM_GRAY: Color = Color::from_rgb(0.412, 0.412, 0.412);

/// Dim Grey <span style="color:dimgrey">Color</span>.
pub const DIM_GREY: Color = DIM_GRAY;

/// Dodger Blue <span style="color:dodgerBlue">Color</span>.
pub const DODGER_BLUE: Color = Color::from_rgb(0.118, 0.565, 1.0);

/// Fire Brick <span style="color:fireBrick">Color</span>.
pub const FIRE_BRICK: Color = Color::from_rgb(0.698, 0.133, 0.133);

/// Floral White <span style="color:floralWhite">Color</span>.
pub const FLORAL_WHITE: Color = Color::from_rgb(1.0, 0.98, 0.941);

/// Forest Green <span style="color:forestGreen">Color</span>.
pub const FOREST_GREEN: Color = Color::from_rgb(0.133, 0.545, 0.133);

/// Fuchsia <span style="color:fuchsia">Color</span>.
pub const FUCHSIA: Color = Color::from_rgb(1.0, 0.0, 1.0);

/// Gainsboro <span style="color:gainsboro">Color</span>.
pub const GAINSBORO: Color = Color::from_rgb(0.863, 0.863, 0.863);

/// Ghost White <span style="color:ghostWhite">Color</span>.
pub const GHOST_WHITE: Color = Color::from_rgb(0.973, 0.973, 1.0);

/// Gold <span style="color:gold">Color</span>.
pub const GOLD: Color = Color::from_rgb(1.0, 0.843, 0.0);

/// Golden Rod <span style="color:goldenRod">Color</span>.
pub const GOLDEN_ROD: Color = Color::from_rgb(0.855, 0.647, 0.125);

/// Gray <span style="color:gray">Color</span>.
pub const GRAY: Color = Color::from_rgb(0.502, 0.502, 0.502);

/// Grey <span style="color:grey">Color</span>.
pub const GREY: Color = GRAY;

/// Green <span style="color:green">Color</span>.
pub const GREEN: Color = Color::from_rgb(0.0, 0.502, 0.0);

/// Green Yellow <span style="color:greenYellow">Color</span>.
pub const GREEN_YELLOW: Color = Color::from_rgb(0.678, 1.0, 0.184);

/// Honey Dew <span style="color:honeyDew">Color</span>.
pub const HONEY_DEW: Color = Color::from_rgb(0.941, 1.0, 0.941);

/// Hot Pink <span style="color:hotPink">Color</span>.
pub const HOT_PINK: Color = Color::from_rgb(1.0, 0.412, 0.706);

/// Indian Red <span style="color:indianRed">Color</span>.
pub const INDIAN_RED: Color = Color::from_rgb(0.804, 0.361, 0.361);

/// Indigo <span style="color:indigo">Color</span>.
pub const INDIGO: Color = Color::from_rgb(0.294, 0.0, 0.51);

/// Ivory <span style="color:ivory">Color</span>.
pub const IVORY: Color = Color::from_rgb(1.0, 1.0, 0.941);

/// Khaki <span style="color:khaki">Color</span>.
pub const KHAKI: Color = Color::from_rgb(0.941, 0.902, 0.549);

/// Lavender <span style="color:lavender">Color</span>.
pub const LAVENDER: Color = Color::from_rgb(0.902, 0.902, 0.98);

/// Lavender Blush <span style="color:lavenderBlush">Color</span>.
pub const LAVENDER_BLUSH: Color = Color::from_rgb(1.0, 0.941, 0.961);

/// Lawn Green <span style="color:lawnGreen">Color</span>.
pub const LAWN_GREEN: Color = Color::from_rgb(0.486, 0.988, 0.0);

/// Lemon Chiffon <span style="color:lemonChiffon">Color</span>.
pub const LEMON_CHIFFON: Color = Color::from_rgb(1.0, 0.98, 0.804);

/// Light Blue <span style="color:lightBlue">Color</span>.
pub const LIGHT_BLUE: Color = Color::from_rgb(0.678, 0.847, 0.902);

/// Light Coral <span style="color:lightCoral">Color</span>.
pub const LIGHT_CORAL: Color = Color::from_rgb(0.941, 0.502, 0.502);

/// Light Cyan <span style="color:lightCyan">Color</span>.
pub const LIGHT_CYAN: Color = Color::from_rgb(0.878, 1.0, 1.0);

/// Light Golden Rod Yellow <span style="color:lightGoldenRodYellow">Color</span>.
pub const LIGHT_GOLDEN_ROD_YELLOW: Color = Color::from_rgb(0.98, 0.98, 0.824);

/// Light Gray <span style="color:lightGray">Color</span>.
pub const LIGHT_GRAY: Color = Color::from_rgb(0.827, 0.827, 0.827);

/// Light Grey <span style="color:lightGrey">Color</span>.
pub const LIGHT_GREY: Color = LIGHT_GRAY;

/// Light Green <span style="color:lightGreen">Color</span>.
pub const LIGHT_GREEN: Color = Color::from_rgb(0.565, 0.933, 0.565);

/// Light Pink <span style="color:lightPink">Color</span>.
pub const LIGHT_PINK: Color = Color::from_rgb(1.0, 0.714, 0.757);

/// Light Salmon <span style="color:lightSalmon">Color</span>.
pub const LIGHT_SALMON: Color = Color::from_rgb(1.0, 0.627, 0.478);

/// Light Sea Green <span style="color:lightSeaGreen">Color</span>.
pub const LIGHT_SEA_GREEN: Color = Color::from_rgb(0.125, 0.698, 0.667);

/// Light Sky Blue <span style="color:lightSkyBlue">Color</span>.
pub const LIGHT_SKY_BLUE: Color = Color::from_rgb(0.529, 0.808, 0.98);

/// Light Slate Gray <span style="color:lightSlateGray">Color</span>.
pub const LIGHT_SLATE_GRAY: Color = Color::from_rgb(0.467, 0.533, 0.6);

/// Light Slate Grey <span style="color:lightSlateGrey">Color</span>.
pub const LIGHT_SLATE_GREY: Color = LIGHT_SLATE_GRAY;

/// Light Steel Blue <span style="color:lightSteelBlue">Color</span>.
pub const LIGHT_STEEL_BLUE: Color = Color::from_rgb(0.69, 0.769, 0.871);

/// Light Yellow <span style="color:lightYellow">Color</span>.
pub const LIGHT_YELLOW: Color = Color::from_rgb(1.0, 1.0, 0.878);

/// Lime <span style="color:lime">Color</span>.
pub const LIME: Color = Color::from_rgb(0.0, 1.0, 0.0);

/// Lime Green <span style="color:limeGreen">Color</span>.
pub const LIME_GREEN: Color = Color::from_rgb(0.196, 0.804, 0.196);

/// Linen <span style="color:linen">Color</span>.
pub const LINEN: Color = Color::from_rgb(0.98, 0.941, 0.902);

/// Magenta <span style="color:magenta">Color</span>.
pub const MAGENTA: Color = Color::from_rgb(1.0, 0.0, 1.0);

/// Maroon <span style="color:maroon">Color</span>.
pub const MAROON: Color = Color::from_rgb(0.502, 0.0, 0.0);

/// Medium Aqua Marine <span style="color:mediumAquaMarine">Color</span>.
pub const MEDIUM_AQUA_MARINE: Color = Color::from_rgb(0.4, 0.804, 0.667);

/// Medium Blue <span style="color:mediumBlue">Color</span>.
pub const MEDIUM_BLUE: Color = Color::from_rgb(0.0, 0.0, 0.804);

/// Medium Orchid <span style="color:mediumOrchid">Color</span>.
pub const MEDIUM_ORCHID: Color = Color::from_rgb(0.729, 0.333, 0.827);

/// Medium Purple <span style="color:mediumPurple">Color</span>.
pub const MEDIUM_PURPLE: Color = Color::from_rgb(0.576, 0.439, 0.859);

/// Medium Sea Green <span style="color:mediumSeaGreen">Color</span>.
pub const MEDIUM_SEA_GREEN: Color = Color::from_rgb(0.235, 0.702, 0.443);

/// Medium Slate Blue <span style="color:mediumSlateBlue">Color</span>.
pub const MEDIUM_SLATE_BLUE: Color = Color::from_rgb(0.482, 0.408, 0.933);

/// Medium Spring Green <span style="color:mediumSpringGreen">Color</span>.
pub const MEDIUM_SPRING_GREEN: Color = Color::from_rgb(0.0, 0.98, 0.604);

/// Medium Turquoise <span style="color:mediumTurquoise">Color</span>.
pub const MEDIUM_TURQUOISE: Color = Color::from_rgb(0.282, 0.82, 0.8);

/// Medium Violet Red <span style="color:mediumVioletRed">Color</span>.
pub const MEDIUM_VIOLET_RED: Color = Color::from_rgb(0.78, 0.082, 0.522);

/// Midnight Blue <span style="color:midnightBlue">Color</span>.
pub const MIDNIGHT_BLUE: Color = Color::from_rgb(0.098, 0.098, 0.439);

/// Mint Cream <span style="color:mintCream">Color</span>.
pub const MINT_CREAM: Color = Color::from_rgb(0.961, 1.0, 0.98);

/// Misty Rose <span style="color:mistyRose">Color</span>.
pub const MISTY_ROSE: Color = Color::from_rgb(1.0, 0.894, 0.882);

/// Moccasin <span style="color:moccasin">Color</span>.
pub const MOCCASIN: Color = Color::from_rgb(1.0, 0.894, 0.71);

/// Navajo White <span style="color:navajo_white">Color</span>.
pub const NAVAJO_WHITE: Color = Color::from_rgb(1.0, 0.871, 0.678);

/// Navy <span style="color:navy">Color</span>.
pub const NAVY: Color = Color::from_rgb(0.0, 0.0, 0.502);

/// Old Lace <span style="color:oldLace">Color</span>.
pub const OLD_LACE: Color = Color::from_rgb(0.992, 0.961, 0.902);

/// Olive <span style="color:olive">Color</span>.
pub const OLIVE: Color = Color::from_rgb(0.502, 0.502, 0.0);

/// Olive Drab <span style="color:oliveDrab">Color</span>.
pub const OLIVE_DRAB: Color = Color::from_rgb(0.42, 0.557, 0.137);

/// Orange <span style="color:orange">Color</span>.
pub const ORANGE: Color = Color::from_rgb(1.0, 0.647, 0.0);

/// Orange Red <span style="color:orangeRed">Color</span>.
pub const ORANGE_RED: Color = Color::from_rgb(1.0, 0.271, 0.0);

/// Orchid <span style="color:orchid">Color</span>.
pub const ORCHID: Color = Color::from_rgb(0.855, 0.439, 0.839);

/// Pale Golden Rod <span style="color:paleGoldenRod">Color</span>.
pub const PALE_GOLDEN_ROD: Color = Color::from_rgb(0.933, 0.91, 0.667);

/// Pale Green <span style="color:paleGreen">Color</span>.
pub const PALE_GREEN: Color = Color::from_rgb(0.596, 0.984, 0.596);

/// Pale Turquoise <span style="color:paleTurquoise">Color</span>.
pub const PALE_TURQUOISE: Color = Color::from_rgb(0.686, 0.933, 0.933);

/// Pale Violet Red <span style="color:paleVioletRed">Color</span>.
pub const PALE_VIOLET_RED: Color = Color::from_rgb(0.859, 0.439, 0.576);

/// Papaya Whip <span style="color:papayaWhip">Color</span>.
pub const PAPAYA_WHIP: Color = Color::from_rgb(1.0, 0.937, 0.835);

/// Peach Puff <span style="color:peachPuff">Color</span>.
pub const PEACH_PUFF: Color = Color::from_rgb(1.0, 0.855, 0.725);

/// Peru <span style="color:peru">Color</span>.
pub const PERU: Color = Color::from_rgb(0.804, 0.522, 0.247);

/// Pink <span style="color:pink">Color</span>.
pub const PINK: Color = Color::from_rgb(1.0, 0.753, 0.796);

/// Plum <span style="color:plum">Color</span>.
pub const PLUM: Color = Color::from_rgb(0.867, 0.627, 0.867);

/// Powder Blue <span style="color:powderBlue">Color</span>.
pub const POWDER_BLUE: Color = Color::from_rgb(0.69, 0.878, 0.902);

/// Purple <span style="color:purple">Color</span>.
pub const PURPLE: Color = Color::from_rgb(0.502, 0.0, 0.502);

/// Rebecca Purple <span style="color:rebeccaPurple">Color</span>.
pub const REBECCA_PURPLE: Color = Color::from_rgb(0.4, 0.2, 0.6);

/// Red <span style="color:red">Color</span>.
pub const RED: Color = Color::from_rgb(1.0, 0.0, 0.0);

/// Rosy Brown <span style="color:rosyBrown">Color</span>.
pub const ROSY_BROWN: Color = Color::from_rgb(0.737, 0.561, 0.561);

/// Royal Blue <span style="color:royalBlue">Color</span>.
pub const ROYAL_BLUE: Color = Color::from_rgb(0.255, 0.412, 0.882);

/// Saddle Brown <span style="color:saddleBrown">Color</span>.
pub const SADDLE_BROWN: Color = Color::from_rgb(0.545, 0.271, 0.075);

/// Salmon <span style="color:salmon">Color</span>.
pub const SALMON: Color = Color::from_rgb(0.98, 0.502, 0.447);

/// Sandy Brown <span style="color:sandyBrown">Color</span>.
pub const SANDY_BROWN: Color = Color::from_rgb(0.957, 0.643, 0.376);

/// Sea Green <span style="color:seaGreen">Color</span>.
pub const SEA_GREEN: Color = Color::from_rgb(0.18, 0.545, 0.341);

/// Sea Shell <span style="color:seaShell">Color</span>.
pub const SEA_SHELL: Color = Color::from_rgb(1.0, 0.961, 0.933);

/// Sienna <span style="color:sienna">Color</span>.
pub const SIENNA: Color = Color::from_rgb(0.627, 0.322, 0.176);

/// Silver <span style="color:silver">Color</span>.
pub const SILVER: Color = Color::from_rgb(0.753, 0.753, 0.753);

/// Sky Blue <span style="color:skyBlue">Color</span>.
pub const SKY_BLUE: Color = Color::from_rgb(0.529, 0.808, 0.922);

/// Slate Blue <span style="color:slateBlue">Color</span>.
pub const SLATE_BLUE: Color = Color::from_rgb(0.416, 0.353, 0.804);

/// Slate Gray <span style="color:slateGray">Color</span>.
pub const SLATE_GRAY: Color = Color::from_rgb(0.439, 0.502, 0.565);

/// Slate Grey <span style="color:slateGrey">Color</span>.
pub const SLATE_GREY: Color = SLATE_GRAY;

/// Snow <span style="color:snow">Color</span>.
pub const SNOW: Color = Color::from_rgb(1.0, 0.98, 0.98);

/// Spring Green <span style="color:springGreen">Color</span>.
pub const SPRING_GREEN: Color = Color::from_rgb(0.0, 1.0, 0.498);

/// Steel Blue <span style="color:steelBlue">Color</span>.
pub const STEEL_BLUE: Color = Color::from_rgb(0.275, 0.51, 0.706);

/// Tan <span style="color:tan">Color</span>.
pub const TAN: Color = Color::from_rgb(0.824, 0.706, 0.549);

/// Teal <span style="color:teal">Color</span>.
pub const TEAL: Color = Color::from_rgb(0.0, 0.502, 0.502);

/// Thistle <span style="color:thistle">Color</span>.
pub const THISTLE: Color = Color::from_rgb(0.847, 0.749, 0.847);

/// Tomato <span style="color:tomato">Color</span>.
pub const TOMATO: Color = Color::from_rgb(1.0, 0.388, 0.278);

/// Turquoise <span style="color:turquoise">Color</span>.
pub const TURQUOISE: Color = Color::from_rgb(0.251, 0.878, 0.816);

/// Violet <span style="color:violet">Color</span>.
pub const VIOLET: Color = Color::from_rgb(0.933, 0.51, 0.933);

/// Wheat <span style="color:wheat">Color</span>.
pub const WHEAT: Color = Color::from_rgb(0.961, 0.871, 0.702);

/// White <span style="color:white">Color</span>.
pub const WHITE: Color = Color::WHITE;

/// White Smoke <span style="color:whiteSmoke">Color</span>.
pub const WHITE_SMOKE: Color = Color::WHITE;

/// Yellow <span style="color:yellow">Color</span>.
pub const YELLOW: Color = Color::from_rgb(1.0, 1.0, 0.0);

/// Yellow Green <span style="color:yellowGreen">Color</span>.
pub const YELLOW_GREEN: Color = Color::from_rgb(0.604, 0.804, 0.196);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_colors_are_defined() {
        // Verify semantic colors point to the correct values
        assert_eq!(PRIMARY, DODGER_BLUE);
        assert_eq!(SECONDARY, DIM_GRAY);
        assert_eq!(SUCCESS, LIME_GREEN);
        assert_eq!(DANGER, RED);
        assert_eq!(WARNING, GOLD);
        assert_eq!(INFO, SKY_BLUE);
        assert_eq!(LIGHT, GHOST_WHITE);
    }

    #[test]
    fn dark_color_is_custom() {
        // DARK is a custom color, not an alias
        assert_eq!(DARK, Color::from_rgb(0.204, 0.227, 0.251));
    }

    #[test]
    fn black_and_white_constants() {
        assert_eq!(BLACK, Color::BLACK);
        assert_eq!(WHITE, Color::WHITE);
        assert_eq!(WHITE_SMOKE, Color::WHITE);
    }

    #[test]
    fn gray_grey_aliases_match() {
        // American/British spelling aliases should be equal
        assert_eq!(GRAY, GREY);
        assert_eq!(DARK_GRAY, DARK_GREY);
        assert_eq!(DIM_GRAY, DIM_GREY);
        assert_eq!(LIGHT_GRAY, LIGHT_GREY);
        assert_eq!(SLATE_GRAY, SLATE_GREY);
        assert_eq!(DARK_SLATE_GRAY, DARK_SLATE_GREY);
        assert_eq!(LIGHT_SLATE_GRAY, LIGHT_SLATE_GREY);
    }

    #[test]
    fn primary_colors_are_valid() {
        // Test basic RGB colors
        assert_eq!(RED, Color::from_rgb(1.0, 0.0, 0.0));
        assert_eq!(GREEN, Color::from_rgb(0.0, 0.502, 0.0));
        assert_eq!(BLUE, Color::from_rgb(0.0, 0.0, 1.0));
    }

    #[test]
    fn cyan_magenta_yellow_are_valid() {
        assert_eq!(CYAN, Color::from_rgb(0.0, 1.0, 1.0));
        assert_eq!(MAGENTA, Color::from_rgb(1.0, 0.0, 1.0));
        assert_eq!(YELLOW, Color::from_rgb(1.0, 1.0, 0.0));
    }

    #[test]
    fn aqua_equals_cyan() {
        assert_eq!(AQUA, CYAN);
    }

    #[test]
    fn fuchsia_equals_magenta() {
        assert_eq!(FUCHSIA, MAGENTA);
    }

    #[test]
    fn color_values_are_in_valid_range() {
        // Sample various colors to ensure RGB values are between 0.0 and 1.0
        let sample_colors = [
            ALICE_BLUE,
            ANTIQUE_WHITE,
            CORAL,
            CRIMSON,
            DARK_GOLDEN_ROD,
            DEEP_PINK,
            DODGER_BLUE,
            FOREST_GREEN,
            GOLD,
            INDIAN_RED,
            LAVENDER,
            LIME_GREEN,
            MEDIUM_PURPLE,
            NAVY,
            ORANGE,
            PALE_GREEN,
            PURPLE,
            ROSY_BROWN,
            SALMON,
            SLATE_BLUE,
            TOMATO,
            TURQUOISE,
            VIOLET,
        ];

        for color in sample_colors.iter() {
            assert!(
                color.r >= 0.0 && color.r <= 1.0,
                "Red component out of range"
            );
            assert!(
                color.g >= 0.0 && color.g <= 1.0,
                "Green component out of range"
            );
            assert!(
                color.b >= 0.0 && color.b <= 1.0,
                "Blue component out of range"
            );
        }
    }

    #[test]
    fn specific_colors_have_correct_values() {
        // Test a few specific colors to ensure accuracy
        assert_eq!(GOLD, Color::from_rgb(1.0, 0.843, 0.0));
        assert_eq!(LIME_GREEN, Color::from_rgb(0.196, 0.804, 0.196));
        assert_eq!(SKY_BLUE, Color::from_rgb(0.529, 0.808, 0.922));
        assert_eq!(DODGER_BLUE, Color::from_rgb(0.118, 0.565, 1.0));
        assert_eq!(GHOST_WHITE, Color::from_rgb(0.973, 0.973, 1.0));
        assert_eq!(DIM_GRAY, Color::from_rgb(0.412, 0.412, 0.412));
    }

    #[test]
    fn dark_colors_are_darker() {
        // Verify dark variants have lower RGB values than their base colors
        assert!(DARK_BLUE.r <= BLUE.r);
        assert!(DARK_BLUE.g <= BLUE.g);
        assert!(DARK_BLUE.b <= BLUE.b);

        assert!(DARK_GREEN.r <= GREEN.r);
        assert!(DARK_GREEN.g <= GREEN.g);
        assert!(DARK_GREEN.b <= GREEN.b);

        assert!(DARK_RED.r <= RED.r);
        assert!(DARK_RED.g <= RED.g);
        assert!(DARK_RED.b <= RED.b);
    }

    #[test]
    fn light_colors_are_lighter() {
        // Verify some light variants are generally lighter (higher average RGB brightness)
        // Note: Not all CSS "light" colors are consistently lighter than their base
        let light_blue_avg = (LIGHT_BLUE.r + LIGHT_BLUE.g + LIGHT_BLUE.b) / 3.0;
        let blue_avg = (BLUE.r + BLUE.g + BLUE.b) / 3.0;
        assert!(light_blue_avg > blue_avg);

        let light_green_avg = (LIGHT_GREEN.r + LIGHT_GREEN.g + LIGHT_GREEN.b) / 3.0;
        let green_avg = (GREEN.r + GREEN.g + GREEN.b) / 3.0;
        assert!(light_green_avg > green_avg);

        // Light Gray should be lighter than Gray in all components (grayscale)
        assert!(LIGHT_GRAY.r >= GRAY.r);
        assert!(LIGHT_GRAY.g >= GRAY.g);
        assert!(LIGHT_GRAY.b >= GRAY.b);
    }

    #[test]
    fn brown_shades_exist() {
        // Test various brown colors
        assert_eq!(BROWN, Color::from_rgb(0.647, 0.165, 0.165));
        assert_eq!(SADDLE_BROWN, Color::from_rgb(0.545, 0.271, 0.075));
        assert_eq!(SANDY_BROWN, Color::from_rgb(0.957, 0.643, 0.376));
        assert_eq!(ROSY_BROWN, Color::from_rgb(0.737, 0.561, 0.561));
    }

    #[test]
    fn purple_shades_exist() {
        // Test various purple colors
        assert_eq!(PURPLE, Color::from_rgb(0.502, 0.0, 0.502));
        assert_eq!(REBECCA_PURPLE, Color::from_rgb(0.4, 0.2, 0.6));
        assert_eq!(MEDIUM_PURPLE, Color::from_rgb(0.576, 0.439, 0.859));
    }

    #[test]
    fn orange_shades_exist() {
        // Test various orange colors
        assert_eq!(ORANGE, Color::from_rgb(1.0, 0.647, 0.0));
        assert_eq!(DARK_ORANGE, Color::from_rgb(1.0, 0.549, 0.0));
        assert_eq!(ORANGE_RED, Color::from_rgb(1.0, 0.271, 0.0));
    }
}
