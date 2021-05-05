use std::fmt;

pub const COLOR_HEX_BLUE_1: &str = "#0e3569";
pub const COLOR_HEX_BLUE_2: &str = "#1960b2";
pub const COLOR_HEX_BLUE_3: &str = "#3a88e2";
pub const COLOR_HEX_BLUE_4: &str = "#5095e5";
pub const COLOR_HEX_BLUE_5: &str = "#a5c9f2";

pub const COLOR_HEX_GREEN_1: &str = "#0c3300";
pub const COLOR_HEX_GREEN_2: &str = "#00400e";
pub const COLOR_HEX_GREEN_3: &str = "#005813";
pub const COLOR_HEX_GREEN_4: &str = "#117401";
pub const COLOR_HEX_GREEN_5: &str = "#038d05";

/// Color can be used to configure colors of different elements on charts.
pub struct Color {
    value: String,
}

impl Color {
    /// Create color from hex string.
    pub fn new_from_hex(hex: &str) -> Self {
        Color {
            value: String::from(hex),
        }
    }

    /// Create color from (r, g, b) values.
    pub fn new_from_rgb(r: u8, g: u8, b: u8) -> Self {
        let value = format!("rgb({},{},{})", r, g, b);
        Color { value }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new_from_hex(COLOR_HEX_BLUE_2)
    }
}
