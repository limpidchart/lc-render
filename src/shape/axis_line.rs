use crate::render::svg::*;

/// AxisLine represents line of an axis.
pub struct AxisLine {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    stroke_width: i32,
    stroke_color: String,
}

impl AxisLine {
    /// Create a new AxisLine.
    fn new(x1: f32, x2: f32, y1: f32, y2: f32) -> Self {
        AxisLine {
            x1,
            x2,
            y1,
            y2,
            stroke_width: DEFAULT_STROKE_WIDTH,
            stroke_color: DEFAULT_STROKE_COLOR.to_string(),
        }
    }

    /// Create a new horizontal AxisLine.
    pub fn new_horizontal(x2: f32) -> Self {
        Self::new(START, x2, START, START)
    }

    /// Create a new vertical AxisLine.
    pub fn new_vertical(y2: f32) -> Self {
        Self::new(START, START, START, y2)
    }

    /// Get SVG representation of an line.
    pub fn to_svg(&self) -> svg::node::element::Line {
        svg::node::element::Line::new()
            .set(X1_ATTR, self.x1)
            .set(X2_ATTR, self.x2)
            .set(Y1_ATTR, self.y1)
            .set(Y2_ATTR, self.y2)
            .set(SHAPE_RENDERING_ATTR, SHAPE_RENDERING_CRISP_EDGES)
            .set(STROKE_WIDTH_ATTR, self.stroke_width)
            .set(STROKE_ATTR, self.stroke_color.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis_line_horizontal_basic() {
        let expected_svg_group = r##"<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="10" y1="0" y2="0"/>"##;

        let axis_line_svg = AxisLine::new_horizontal(10_f32).to_svg();

        assert_eq!(axis_line_svg.to_string(), expected_svg_group);
    }

    #[test]
    fn axis_line_vertical_basic() {
        let expected_svg_group = r##"<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="0" y1="0" y2="10"/>"##;

        let axis_line_svg = AxisLine::new_vertical(10_f32).to_svg();

        assert_eq!(axis_line_svg.to_string(), expected_svg_group);
    }
}
