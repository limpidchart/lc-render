use crate::render::svg::*;
use crate::shape::axis::AxisPosition;
use svg::Node;

const DEFAULT_LINE_LENGTH: i32 = 6;
const DEFAULT_FONT_SIZE: &str = "12px";
const DEFAULT_LABEL_ROTATION: i32 = 0;

/// AxisTick represents a single tick on axis line.
pub struct AxisTick {
    axis_position: AxisPosition,
    tick_offset: f32,
    label: String,
    label_rotation: i32,
    label_offset: i32,
}

impl AxisTick {
    /// Create a new AxisTick.
    pub fn new(
        axis_position: AxisPosition,
        tick_offset: f32,
        label: &str,
        label_offset: i32,
    ) -> Self {
        Self {
            axis_position,
            tick_offset,
            label: label.to_string(),
            label_rotation: DEFAULT_LABEL_ROTATION,
            label_offset,
        }
    }

    /// Get SVG representation of a tick.
    pub fn to_svg(&self) -> svg::node::element::Group {
        let translate_x: f32;
        let translate_y: f32;
        let x2_attr: i32;
        let y2_attr: i32;
        let tick_label_text_anchor: &str;
        let tick_label_offset_x: i32;
        let tick_label_offset_y: i32;

        match self.axis_position {
            AxisPosition::Top => {
                translate_x = self.tick_offset;
                translate_y = START;
                x2_attr = 0;
                y2_attr = -DEFAULT_LINE_LENGTH;
                tick_label_text_anchor = TEXT_ANCHOR_MIDDLE;
                tick_label_offset_x = 0;
                tick_label_offset_y = -self.label_offset;
            }
            AxisPosition::Bottom => {
                translate_x = self.tick_offset;
                translate_y = START;
                x2_attr = 0;
                y2_attr = DEFAULT_LINE_LENGTH;
                tick_label_text_anchor = TEXT_ANCHOR_MIDDLE;
                tick_label_offset_x = 0;
                tick_label_offset_y = self.label_offset;
            }
            AxisPosition::Left => {
                translate_x = START;
                translate_y = self.tick_offset;
                x2_attr = -DEFAULT_LINE_LENGTH;
                y2_attr = 0;
                tick_label_text_anchor = TEXT_ANCHOR_END;
                tick_label_offset_x = -self.label_offset;
                tick_label_offset_y = 0;
            }
            AxisPosition::Right => {
                translate_x = START;
                translate_y = self.tick_offset;
                x2_attr = DEFAULT_LINE_LENGTH;
                y2_attr = 0;
                tick_label_text_anchor = TEXT_ANCHOR_START;
                tick_label_offset_x = self.label_offset;
                tick_label_offset_y = 0;
            }
        };

        let mut res = svg::node::element::Group::new()
            .set(CLASS_ATTR, CLASS_TICK)
            .set(TRANSFORM_ATTR, translate_x_y(translate_x, translate_y));

        let tick_line = svg::node::element::Line::new()
            .set(X1_ATTR, START)
            .set(Y1_ATTR, START)
            .set(X2_ATTR, x2_attr)
            .set(Y2_ATTR, y2_attr)
            .set(SHAPE_RENDERING_ATTR, SHAPE_RENDERING_CRISP_EDGES)
            .set(STROKE_ATTR, DEFAULT_STROKE_COLOR)
            .set(STROKE_WIDTH_ATTR, DEFAULT_STROKE_WIDTH);
        res.append(tick_line);

        let tick_label = svg::node::element::Text::new()
            .set(
                TRANSFORM_ATTR,
                rotate_a_x_y(
                    self.label_rotation,
                    tick_label_offset_x,
                    tick_label_offset_y,
                ),
            )
            .set(X_ATTR, tick_label_offset_x)
            .set(Y_ATTR, tick_label_offset_y)
            .set(DY_ATTR, DEFAULT_DY)
            .set(TEXT_ANCHOR_ATTR, tick_label_text_anchor)
            .set(FONT_SIZE_ATTR, DEFAULT_FONT_SIZE)
            .set(FONT_FAMILY_ATTR, DEFAULT_FONT_FAMILY)
            .set(FILL_ATTR, DEFAULT_FONT_COLOR)
            .add(svg::node::Text::new(self.label.to_owned()));
        res.append(tick_label);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis_tick_basic() {
        let expected_svg_group = r##"<g class="tick" transform="translate(2,0)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="0" y1="0" y2="-6"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="middle" transform="rotate(0,0,-4)" x="0" y="-4">
tick
</text>
</g>"##;

        let axis_tick_svg = AxisTick::new(AxisPosition::Top, 2_f32, "tick", 4).to_svg();

        assert_eq!(axis_tick_svg.to_string(), expected_svg_group);
    }
}
