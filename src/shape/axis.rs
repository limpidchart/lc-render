use crate::render::svg::*;
use crate::scale::Scale;
use crate::shape::axis_line::AxisLine;
use crate::shape::axis_tick::AxisTick;
use std::string::ToString;
use svg::Node;

const DEFAULT_TICK_LABEL_HORIZONTAL_OFFSET: i32 = 12;
const DEFAULT_TICK_LABEL_VERTICAL_OFFSET: i32 = 16;

const DEFAULT_FONT_SIZE: &str = "14px";

const DEFAULT_AXIS_LABEL_TOP_OFFSET: i32 = -32;
const DEFAULT_AXIS_LABEL_BOTTOM_OFFSET: i32 = 42;
const DEFAULT_AXIS_LABEL_LEFT_OFFSET: i32 = -42;
const DEFAULT_AXIS_LABEL_RIGHT_OFFSET: i32 = -28;

/// AxisPosition represents a position for axis line on a chart.
#[derive(Copy, Clone)]
pub enum AxisPosition {
    Top,
    Right,
    Bottom,
    Left,
}

/// Axis represents a line with ticks.
pub struct Axis {
    ticks: Vec<AxisTick>,
    line: AxisLine,
    position: AxisPosition,
    label: String,
    label_x_attr: i32,
    label_y_attr: i32,
    label_rotation: i32,
}

impl Axis {
    /// Create a new Axis.
    pub fn new<T: ToString>(
        scale: &dyn Scale<T>,
        position: AxisPosition,
        view_width: i32,
        view_height: i32,
    ) -> Self {
        Self {
            ticks: Self::axis_ticks(scale, position),
            position,
            line: Self::axis_line(position, view_width as f32, view_height as f32),
            label: String::new(),
            label_x_attr: Self::axis_label_x_attr(position, view_width, view_height),
            label_y_attr: Self::axis_label_y_attr(position),
            label_rotation: Self::axis_label_rotation(position),
        }
    }

    /// Configure Axis label.
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    fn axis_ticks<T: ToString>(scale: &dyn Scale<T>, position: AxisPosition) -> Vec<AxisTick> {
        let mut res = Vec::new();
        let label_offset = match position {
            AxisPosition::Top | AxisPosition::Bottom => DEFAULT_TICK_LABEL_VERTICAL_OFFSET,
            AxisPosition::Left | AxisPosition::Right => DEFAULT_TICK_LABEL_HORIZONTAL_OFFSET,
        };

        for tick in scale.ticks() {
            let tick_offset = scale.scale(&tick) + scale.tick_offset();
            let axis_tick = AxisTick::new(position, tick_offset, &*tick.to_string(), label_offset);
            res.push(axis_tick);
        }

        res
    }

    fn axis_line(position: AxisPosition, view_width: f32, view_height: f32) -> AxisLine {
        match position {
            AxisPosition::Top | AxisPosition::Bottom => AxisLine::new_horizontal(view_width),
            AxisPosition::Left | AxisPosition::Right => AxisLine::new_vertical(view_height),
        }
    }

    fn axis_label_x_attr(position: AxisPosition, view_width: i32, view_height: i32) -> i32 {
        let length = match position {
            AxisPosition::Top | AxisPosition::Bottom => view_width,
            AxisPosition::Left | AxisPosition::Right => view_height,
        };

        match position {
            AxisPosition::Left => -(length / 2),
            _ => length / 2,
        }
    }

    fn axis_label_y_attr(position: AxisPosition) -> i32 {
        match position {
            AxisPosition::Top => DEFAULT_AXIS_LABEL_TOP_OFFSET,
            AxisPosition::Bottom => DEFAULT_AXIS_LABEL_BOTTOM_OFFSET,
            AxisPosition::Left => DEFAULT_AXIS_LABEL_LEFT_OFFSET,
            AxisPosition::Right => DEFAULT_AXIS_LABEL_RIGHT_OFFSET,
        }
    }

    fn axis_label_rotation(position: AxisPosition) -> i32 {
        match position {
            AxisPosition::Left => -90,
            AxisPosition::Right => 90,
            _ => 0,
        }
    }

    /// Get axis SVG representation.
    pub fn to_svg(&self) -> svg::node::element::Group {
        let axis_class = match self.position {
            AxisPosition::Top | AxisPosition::Bottom => CLASS_X_AXIS,
            AxisPosition::Left | AxisPosition::Right => CLASS_Y_AXIS,
        };

        let mut res = svg::node::element::Group::new()
            .set(CLASS_ATTR, axis_class)
            .add(self.line.to_svg());
        for tick in self.ticks.iter() {
            res.append(tick.to_svg());
        }

        if self.label.is_empty() {
            return res;
        }

        let axis_label = svg::node::element::Text::new()
            .set(X_ATTR, self.label_x_attr)
            .set(Y_ATTR, self.label_y_attr)
            .set(TEXT_ANCHOR_ATTR, TEXT_ANCHOR_MIDDLE)
            .set(FONT_SIZE_ATTR, DEFAULT_FONT_SIZE)
            .set(FONT_FAMILY_ATTR, DEFAULT_FONT_FAMILY)
            .set(FILL_ATTR, DEFAULT_FONT_COLOR)
            .set(TRANSFORM_ATTR, rotate_a(self.label_rotation))
            .add(svg::node::Text::new(&self.label));
        res.append(axis_label);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BandScale;

    #[test]
    fn axis_basic() {
        let expected_svg_group = r##"<g class="y-axis">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="0" y1="0" y2="200"/>
<g class="tick" transform="translate(0,177.41937)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
a1
</text>
</g>
<g class="tick" transform="translate(0,500)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
a2
</text>
</g>
<g class="tick" transform="translate(0,822.58057)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
3
</text>
</g>
</g>"##;

        let scale = BandScale::new(
            vec!["a1".to_string(), "a2".to_string(), "3".to_string()],
            0,
            1000,
        );
        let axis_svg = Axis::new(&scale, AxisPosition::Left, 100, 200).to_svg();

        assert_eq!(axis_svg.to_string(), expected_svg_group);
    }
}
