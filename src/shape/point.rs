use crate::render::svg::*;
use svg::Node;

const DEFAULT_LABEL_VISIBLE: bool = true;
const DEFAULT_LABEL_POSITION: PointLabelPosition = PointLabelPosition::Top;

const DEFAULT_STROKE_WIDTH: &str = "2px";

const DEFAULT_X_LABEL_HORIZONTAL: i32 = 8;
const DEFAULT_X_LABEL_VERTICAL: i32 = 0;
const DEFAULT_X_LABEL_BETWEEN: i32 = 4;

const DEFAULT_Y_LABEL_HORIZONTAL: i32 = 0;
const DEFAULT_Y_LABEL_VERTICAL: i32 = 12;
const DEFAULT_Y_LABEL_BETWEEN: i32 = 8;

const DEFAULT_FONT_SIZE: &str = "14px";

const DEFAULT_POINT_VISIBLE: bool = true;

/// PointType contains available types of points.
#[derive(Copy, Clone)]
pub enum PointType {
    Circle,
    Square,
    X,
}

/// PointLabelPosition contains available types of point label positions.
#[derive(Copy, Clone)]
pub enum PointLabelPosition {
    Top,
    TopRight,
    TopLeft,
    Left,
    Right,
    Bottom,
    BottomLeft,
    BottomRight,
}

/// Point represents a point shape.
#[derive(Clone)]
pub struct Point {
    x: f32,
    y: f32,
    label_visible: bool,
    label_position: PointLabelPosition,
    point_visible: bool,
    point_type: PointType,
    size: i32,
    x_label: String,
    y_label: String,
    fill_color: String,
    stroke_color: String,
    label_text_anchor: String,
    label_x_attr: i32,
    label_y_attr: i32,
}

impl Point {
    /// Create a new Point.
    pub fn new(
        x: f32,
        y: f32,
        point_type: PointType,
        size: i32,
        y_label: &str,
        fill_color: &str,
        stroke_color: &str,
    ) -> Self {
        Point {
            x,
            y,
            point_visible: DEFAULT_POINT_VISIBLE,
            point_type,
            size,
            x_label: String::new(),
            y_label: y_label.to_string(),
            fill_color: fill_color.to_string(),
            stroke_color: stroke_color.to_string(),
            label_visible: DEFAULT_LABEL_VISIBLE,
            label_position: DEFAULT_LABEL_POSITION,
            label_text_anchor: Self::label_text_anchor(DEFAULT_LABEL_POSITION),
            label_x_attr: Self::label_x_attr(DEFAULT_LABEL_POSITION, size),
            label_y_attr: Self::label_y_attr(DEFAULT_LABEL_POSITION, size),
        }
    }

    /// Set point visibility.
    pub fn set_point_visible(mut self, point_visible: bool) -> Self {
        self.point_visible = point_visible;
        self
    }

    /// Set custom x for label.
    pub fn set_x_label(mut self, x_label: &str) -> Self {
        self.x_label = x_label.to_string();
        self
    }

    /// Set point visibility.
    pub fn set_label_visible(mut self, label_visible: bool) -> Self {
        self.label_visible = label_visible;
        self
    }

    /// Set position for point label.
    pub fn set_label_position(mut self, label_position: PointLabelPosition) -> Self {
        self.label_position = label_position;
        self.label_text_anchor = Self::label_text_anchor(label_position);
        self.label_x_attr = Self::label_x_attr(label_position, self.size);
        self.label_y_attr = Self::label_y_attr(label_position, self.size);
        self
    }

    /// Get x value of a point.
    pub fn x(&self) -> f32 {
        self.x
    }
    /// Get y x value of a point.
    pub fn y(&self) -> f32 {
        self.y
    }

    fn label_text_anchor(label_position: PointLabelPosition) -> String {
        match label_position {
            PointLabelPosition::Top | PointLabelPosition::Bottom => TEXT_ANCHOR_MIDDLE.to_string(),
            PointLabelPosition::TopRight
            | PointLabelPosition::BottomRight
            | PointLabelPosition::Right => TEXT_ANCHOR_START.to_string(),
            PointLabelPosition::TopLeft
            | PointLabelPosition::BottomLeft
            | PointLabelPosition::Left => TEXT_ANCHOR_END.to_string(),
        }
    }

    fn label_x_attr(label_position: PointLabelPosition, size: i32) -> i32 {
        match label_position {
            PointLabelPosition::Top | PointLabelPosition::Bottom => DEFAULT_X_LABEL_VERTICAL,
            PointLabelPosition::TopRight | PointLabelPosition::BottomRight => {
                size + DEFAULT_X_LABEL_BETWEEN
            }
            PointLabelPosition::Right => size + DEFAULT_X_LABEL_HORIZONTAL,
            PointLabelPosition::TopLeft | PointLabelPosition::BottomLeft => {
                -size - DEFAULT_X_LABEL_BETWEEN
            }
            PointLabelPosition::Left => -size - DEFAULT_X_LABEL_HORIZONTAL,
        }
    }

    fn label_y_attr(label_position: PointLabelPosition, size: i32) -> i32 {
        match label_position {
            PointLabelPosition::Top => -size - DEFAULT_Y_LABEL_VERTICAL,
            PointLabelPosition::TopRight | PointLabelPosition::TopLeft => {
                -size - DEFAULT_Y_LABEL_BETWEEN
            }
            PointLabelPosition::Right | PointLabelPosition::Left => DEFAULT_Y_LABEL_HORIZONTAL,
            PointLabelPosition::BottomRight | PointLabelPosition::BottomLeft => {
                size + DEFAULT_Y_LABEL_BETWEEN
            }
            PointLabelPosition::Bottom => size + DEFAULT_Y_LABEL_VERTICAL,
        }
    }

    /// Get point SVG representation.
    pub fn to_svg(&self) -> svg::node::element::Group {
        let mut res = svg::node::element::Group::new()
            .set(TRANSFORM_ATTR, translate_x_y(self.x, self.y))
            .set(CLASS_ATTR, CLASS_POINT);

        // Draw point if needed.
        if self.point_visible {
            match self.point_type {
                PointType::Circle => {
                    res.append(
                        svg::node::element::Circle::new()
                            .set(CX_ATTR, START)
                            .set(CY_ATTR, START)
                            .set(R_ATTR, self.size)
                            .set(FILL_ATTR, self.fill_color.as_ref())
                            .set(STROKE_ATTR, self.stroke_color.as_ref()),
                    );
                }
                PointType::Square => {
                    res.append(
                        svg::node::element::Rectangle::new()
                            .set(X_ATTR, -(self.size as i32))
                            .set(Y_ATTR, -(self.size as i32))
                            .set(WIDTH_ATTR, 2 * self.size)
                            .set(HEIGHT_ATTR, 2 * self.size)
                            .set(FILL_ATTR, self.fill_color.as_ref())
                            .set(STROKE_ATTR, self.stroke_color.as_ref()),
                    );
                }
                PointType::X => {
                    res.append(
                        svg::node::element::Group::new()
                            .add(
                                svg::node::element::Line::new()
                                    .set(X1_ATTR, -(self.size as i32))
                                    .set(Y1_ATTR, -(self.size as i32))
                                    .set(X2_ATTR, self.size)
                                    .set(Y2_ATTR, self.size)
                                    .set(STROKE_WIDTH_ATTR, DEFAULT_STROKE_WIDTH)
                                    .set(STROKE_ATTR, self.stroke_color.as_ref()),
                            )
                            .add(
                                svg::node::element::Line::new()
                                    .set(X1_ATTR, self.size)
                                    .set(Y1_ATTR, -(self.size as i32))
                                    .set(X2_ATTR, -(self.size as i32))
                                    .set(Y2_ATTR, self.size)
                                    .set(STROKE_WIDTH_ATTR, DEFAULT_STROKE_WIDTH)
                                    .set(STROKE_ATTR, self.stroke_color.as_ref()),
                            ),
                    );
                }
            }
        };

        // Draw label if needed.
        if self.label_visible {
            let mut label: svg::node::element::Text;

            // X label will be empty in case of Area or Line chart.
            if self.x_label.is_empty() {
                label = svg::node::element::Text::new()
                    .set(DY_ATTR, DEFAULT_DY)
                    .set(FONT_FAMILY_ATTR, DEFAULT_FONT_FAMILY)
                    .set(FILL_ATTR, DEFAULT_FONT_COLOR)
                    .set(FONT_SIZE_ATTR, DEFAULT_FONT_SIZE)
                    .add(svg::node::Text::new(self.y_label.to_string()));
            } else {
                label = svg::node::element::Text::new()
                    .set(DY_ATTR, DEFAULT_DY)
                    .set(FONT_FAMILY_ATTR, DEFAULT_FONT_FAMILY)
                    .set(FILL_ATTR, DEFAULT_FONT_COLOR)
                    .set(FONT_SIZE_ATTR, DEFAULT_FONT_SIZE)
                    .add(svg::node::Text::new(pair_x_y(&self.x_label, &self.y_label)));
            }

            label.assign(X_ATTR, self.label_x_attr);
            label.assign(Y_ATTR, self.label_y_attr);
            label.assign(TEXT_ANCHOR_ATTR, self.label_text_anchor.clone());

            res.append(label);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bar_basic() {
        let expected_svg_group = r##"<g class="point" transform="translate(10,20)">
<circle cx="0" cy="0" fill="#f289ff" r="21" stroke="#8a87f6"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="end" x="-25" y="29">
thirty
</text>
</g>"##;

        let point_svg = Point::new(
            10_f32,
            20_f32,
            PointType::Circle,
            21,
            "thirty",
            "#f289ff",
            "#8a87f6",
        )
        .set_label_position(PointLabelPosition::BottomLeft)
        .to_svg();
        assert_eq!(point_svg.to_string(), expected_svg_group);
    }
}
