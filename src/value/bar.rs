use crate::color::{COLOR_HEX_BLUE_1, COLOR_HEX_BLUE_2};
use crate::Color;

/// BarsValues contains values and color settings for bar views.
pub struct BarsValues {
    values: Vec<f32>,
    fill_color: String,
    stroke_color: String,
}

impl BarsValues {
    /// Create a new BarsValues.
    pub(crate) fn new(values: Vec<f32>) -> Self {
        Self {
            values,
            fill_color: COLOR_HEX_BLUE_2.to_string(),
            stroke_color: COLOR_HEX_BLUE_1.to_string(),
        }
    }

    /// Set fill color for BarsValues.
    pub fn set_fill_color(mut self, fill_color: Color) -> Self {
        self.fill_color = fill_color.to_string();
        self
    }

    /// Set stroke color for BarsValues.
    pub fn set_stroke_color(mut self, stroke_color: Color) -> Self {
        self.stroke_color = stroke_color.to_string();
        self
    }

    /// Get values.
    pub fn values(&self) -> &Vec<f32> {
        &self.values
    }

    /// Get fill color.
    pub fn fill_color(&self) -> &str {
        &self.fill_color
    }

    /// Get stroke color.
    pub fn stroke_color(&self) -> &str {
        &self.stroke_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{COLOR_HEX_GREEN_3, COLOR_HEX_GREEN_5};

    #[test]
    fn bars_values_basic() {
        let bars_values = BarsValues::new(vec![77_f32, 12_f32, 32_f32, 24_f32, 6_f32])
            .set_fill_color(Color::new_from_hex(COLOR_HEX_GREEN_5))
            .set_stroke_color(Color::new_from_hex(COLOR_HEX_GREEN_3));

        assert_eq!(
            *bars_values.values(),
            vec![77_f32, 12_f32, 32_f32, 24_f32, 6_f32]
        );
        assert_eq!(bars_values.fill_color(), COLOR_HEX_GREEN_5);
        assert_eq!(bars_values.stroke_color(), COLOR_HEX_GREEN_3);
    }
}
