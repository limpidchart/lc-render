use crate::color::{COLOR_HEX_BLUE_1, COLOR_HEX_BLUE_2};
use crate::math::linear::range;
use crate::render::svg::*;
use crate::Orientation;
use svg::Node;

const DEFAULT_FONT_SIZE: &str = "14px";
const DEFAULT_LABEL_HORIZONTAL_OFFSET: f32 = 12_f32;
const DEFAULT_LABEL_VERTICAL_OFFSET: f32 = 16_f32;
const DEFAULT_LABEL_VISIBLE: bool = true;
const DEFAULT_LABEL_POSITION: BarLabelPosition = BarLabelPosition::Center;

/// BarLabelPosition represents available label positions of a bar shape.
#[derive(Copy, Clone)]
pub enum BarLabelPosition {
    StartOutside,
    StartInside,
    Center,
    EndInside,
    EndOutside,
}

/// Bar represents a shape that can be used with vertical or horizontal views.
#[derive(Clone)]
pub struct Bar {
    start: f32,
    end: f32,
    size: f32,
    width: f32,
    offset: f32,
    orientation: Orientation,
    fill_color: String,
    stroke_color: String,
    stroke_width: i32,
    label_visible: bool,
    label_position: BarLabelPosition,
    label_text_anchor: String,
    label_x_attr: f32,
}

impl Bar {
    /// Create a new Bar.
    pub fn new(
        start: f32,
        end: f32,
        size: f32,
        width: f32,
        offset: f32,
        orientation: Orientation,
    ) -> Self {
        Bar {
            start,
            end,
            size,
            width,
            offset,
            orientation,
            fill_color: COLOR_HEX_BLUE_2.to_string(),
            stroke_color: COLOR_HEX_BLUE_1.to_string(),
            stroke_width: DEFAULT_STROKE_WIDTH,
            label_visible: DEFAULT_LABEL_VISIBLE,
            label_position: DEFAULT_LABEL_POSITION,
            label_text_anchor: Self::label_text_anchor(DEFAULT_LABEL_POSITION, orientation),
            label_x_attr: Self::label_x_attr(start, end, DEFAULT_LABEL_POSITION, orientation),
        }
    }

    /// Set bar fill color.
    pub fn set_fill_color(mut self, fill_color: &str) -> Self {
        self.fill_color = fill_color.to_string();
        self
    }

    /// Set bar stroke color.
    pub fn set_stroke_color(mut self, stroke_color: &str) -> Self {
        self.stroke_color = stroke_color.to_string();
        self
    }

    /// Set bar label visibility.
    pub fn set_label_visible(mut self, label_visible: bool) -> Self {
        self.label_visible = label_visible;
        self
    }

    /// Set label position for bar.
    pub fn set_label_position(mut self, label_position: BarLabelPosition) -> Self {
        self.label_position = label_position;
        self.label_text_anchor = Self::label_text_anchor(label_position, self.orientation);
        self.label_x_attr =
            Self::label_x_attr(self.start, self.end, label_position, self.orientation);
        self
    }

    fn label_text_anchor(label_position: BarLabelPosition, orientation: Orientation) -> String {
        match label_position {
            BarLabelPosition::StartOutside => {
                if orientation == Orientation::Horizontal {
                    return TEXT_ANCHOR_END.to_string();
                }
                TEXT_ANCHOR_MIDDLE.to_string()
            }
            BarLabelPosition::StartInside => {
                if orientation == Orientation::Horizontal {
                    return TEXT_ANCHOR_START.to_string();
                }
                TEXT_ANCHOR_MIDDLE.to_string()
            }
            BarLabelPosition::Center => TEXT_ANCHOR_MIDDLE.to_string(),
            BarLabelPosition::EndInside => {
                if orientation == Orientation::Horizontal {
                    return TEXT_ANCHOR_END.to_string();
                }
                TEXT_ANCHOR_MIDDLE.to_string()
            }
            BarLabelPosition::EndOutside => {
                if orientation == Orientation::Horizontal {
                    return TEXT_ANCHOR_START.to_string();
                }
                TEXT_ANCHOR_MIDDLE.to_string()
            }
        }
    }

    fn label_x_attr(
        start: f32,
        end: f32,
        label_position: BarLabelPosition,
        orientation: Orientation,
    ) -> f32 {
        match label_position {
            BarLabelPosition::StartOutside => {
                if orientation == Orientation::Horizontal {
                    return start - DEFAULT_LABEL_HORIZONTAL_OFFSET;
                }
                end + DEFAULT_LABEL_VERTICAL_OFFSET
            }
            BarLabelPosition::StartInside => {
                if orientation == Orientation::Horizontal {
                    return start + DEFAULT_LABEL_HORIZONTAL_OFFSET;
                }
                end - DEFAULT_LABEL_VERTICAL_OFFSET
            }
            BarLabelPosition::Center => start + (range(start, end) / 2.0),
            BarLabelPosition::EndInside => {
                if orientation == Orientation::Horizontal {
                    return end - DEFAULT_LABEL_VERTICAL_OFFSET;
                }
                start + DEFAULT_LABEL_HORIZONTAL_OFFSET
            }
            BarLabelPosition::EndOutside => {
                if orientation == Orientation::Horizontal {
                    return end + DEFAULT_LABEL_VERTICAL_OFFSET;
                }
                start - DEFAULT_LABEL_HORIZONTAL_OFFSET
            }
        }
    }

    /// Get bar SVG representation.
    pub fn to_svg(&self) -> svg::node::element::Group {
        let x_attr = match self.orientation {
            Orientation::Horizontal => (X_ATTR),
            Orientation::Vertical => (Y_ATTR),
        };
        let y_attr = match self.orientation {
            Orientation::Horizontal => (Y_ATTR),
            Orientation::Vertical => (X_ATTR),
        };
        let w_attr = match self.orientation {
            Orientation::Horizontal => (WIDTH_ATTR),
            Orientation::Vertical => (HEIGHT_ATTR),
        };
        let h_attr = match self.orientation {
            Orientation::Horizontal => (HEIGHT_ATTR),
            Orientation::Vertical => (WIDTH_ATTR),
        };
        let rect = svg::node::element::Rectangle::new()
            .set(x_attr, self.start)
            .set(y_attr, START)
            .set(w_attr, range(self.start, self.end))
            .set(h_attr, self.width)
            .set(SHAPE_RENDERING_ATTR, SHAPE_RENDERING_CRISP_EDGES)
            .set(FILL_ATTR, self.fill_color.to_string())
            .set(STROKE_WIDTH_ATTR, self.stroke_width)
            .set(STROKE_ATTR, self.stroke_color.to_string());

        let offset_x = match self.orientation {
            Orientation::Horizontal => 0.0,
            Orientation::Vertical => self.offset,
        };
        let offset_y = match self.orientation {
            Orientation::Horizontal => self.offset,
            Orientation::Vertical => 0.0,
        };
        let mut group = svg::node::element::Group::new()
            .set(TRANSFORM_ATTR, translate_x_y(offset_x, offset_y))
            .set(CLASS_ATTR, CLASS_BAR);
        group.append(rect);

        if !self.label_visible {
            return group;
        }

        let label = svg::node::element::Text::new()
            .set(x_attr, self.label_x_attr)
            .set(y_attr, self.width / 2.0) // we want label to be centered vertically
            .set(TEXT_ANCHOR_ATTR, self.label_text_anchor.to_owned())
            .set(DY_ATTR, DEFAULT_DY)
            .set(FONT_FAMILY_ATTR, DEFAULT_FONT_FAMILY)
            .set(FILL_ATTR, DEFAULT_FONT_COLOR)
            .set(FONT_SIZE_ATTR, DEFAULT_FONT_SIZE)
            .add(svg::node::Text::new(self.size.to_string()));
        group.append(label);

        group
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{COLOR_HEX_GREEN_2, COLOR_HEX_GREEN_4};

    #[test]
    fn bar_basic() {
        let expected_svg_group = r##"<g class="bar" transform="translate(5,0)">
<rect fill="#117401" height="10" shape-rendering="crispEdges" stroke="#00400e" stroke-width="1" width="40" x="0" y="10"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="20" y="36">
30
</text>
</g>"##;

        let bar_svg = Bar::new(10_f32, 20_f32, 30_f32, 40_f32, 5_f32, Orientation::Vertical)
            .set_fill_color(COLOR_HEX_GREEN_4)
            .set_stroke_color(COLOR_HEX_GREEN_2)
            .set_label_visible(true)
            .set_label_position(BarLabelPosition::StartOutside)
            .to_svg();

        assert_eq!(bar_svg.to_string(), expected_svg_group);
    }
}
