use crate::color::{COLOR_HEX_BLUE_3, COLOR_HEX_BLUE_4};
use crate::render::svg::*;
use crate::shape::point::Point;
use crate::{Color, Error, LinearScale, PointLabelPosition, PointType, Scale};
use svg::Node;

const DEFAULT_LABEL_VISIBLE: bool = true;
const DEFAULT_LABEL_POSITION: PointLabelPosition = PointLabelPosition::Top;

const DEFAULT_POINT_TYPE: PointType = PointType::Circle;
const DEFAULT_POINT_VISIBLE: bool = true;

/// ScatterView represents separated points view.
#[derive(Clone)]
pub struct ScatterView {
    x_scale: LinearScale,
    y_scale: LinearScale,
    point_fill_color: String,
    point_stroke_color: String,
    points: Vec<Point>,
    point_type: PointType,
    point_visible: bool,
    point_label_visible: bool,
    point_label_position: PointLabelPosition,
}

impl ScatterView {
    /// Create a new ScatterView.
    pub fn new(x_scale: LinearScale, y_scale: LinearScale) -> Self {
        Self {
            x_scale,
            y_scale,
            point_fill_color: COLOR_HEX_BLUE_4.to_string(),
            point_stroke_color: COLOR_HEX_BLUE_3.to_string(),
            points: Vec::new(),
            point_type: DEFAULT_POINT_TYPE,
            point_visible: DEFAULT_POINT_VISIBLE,
            point_label_visible: DEFAULT_LABEL_VISIBLE,
            point_label_position: DEFAULT_LABEL_POSITION,
        }
    }

    /// Set scatter points fill color.
    pub fn set_point_fill_color(mut self, point_fill_color: Color) -> Self {
        self.point_fill_color = point_fill_color.to_string();
        self
    }

    /// Set scatter points stroke color.
    pub fn set_point_stroke_color(mut self, point_stroke_color: Color) -> Self {
        self.point_stroke_color = point_stroke_color.to_string();
        self
    }

    /// Set scatter points type.
    pub fn set_point_type(mut self, point_type: PointType) -> Self {
        self.point_type = point_type;
        self
    }

    /// Set scatter points visibility.
    pub fn set_point_visible(mut self, point_visible: bool) -> Self {
        self.point_visible = point_visible;
        self
    }

    /// Set scatter points label visibility.
    pub fn set_point_label_visible(mut self, point_label_visible: bool) -> Self {
        self.point_label_visible = point_label_visible;
        self
    }

    /// Set scatter points label position.
    pub fn set_point_label_position(mut self, point_label_position: PointLabelPosition) -> Self {
        self.point_label_position = point_label_position;
        self
    }

    /// Set values for scatter view.
    pub fn set_data(mut self, data: &[(f32, f32)]) -> Result<Self, Error> {
        if data.is_empty() {
            return Err(Error::DataIsEmpty);
        }

        // Compute offsets in case there is a non-zero bandwidth.
        let x_bandwidth_offset = {
            if self.x_scale.is_range_reversed() {
                -self.x_scale.tick_offset()
            } else {
                self.x_scale.tick_offset()
            }
        };
        let y_bandwidth_offset = {
            if self.y_scale.is_range_reversed() {
                -self.y_scale.tick_offset()
            } else {
                self.y_scale.tick_offset()
            }
        };

        let mut points = Vec::new();
        for values in data.iter() {
            let scaled_x = &self.x_scale.scale(&values.0);
            let scaled_y = self.y_scale.scale(&values.1);

            let point = Point::new(
                scaled_x + x_bandwidth_offset,
                scaled_y + y_bandwidth_offset,
                self.point_type,
                DEFAULT_POINT_SIZE,
                &values.1.to_string(),
                &self.point_fill_color.to_string(),
                &self.point_stroke_color.to_string(),
            )
            .set_point_visible(self.point_visible)
            .set_x_label(&values.0.to_string())
            .set_label_visible(self.point_label_visible)
            .set_label_position(self.point_label_position);
            points.push(point);
        }
        self.points = points;

        Ok(self)
    }

    /// Get scatter view SVG representation.
    pub fn to_svg(&self) -> svg::node::element::Group {
        let mut res = svg::node::element::Group::new();
        for point in self.points.iter() {
            res.append(point.to_svg());
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scatter_basic() {
        let expected_svg_group = r##"<g>
<g class="point" transform="translate(5.125,4.7249985)">
<circle cx="0" cy="0" fill="#5095e5" r="5" stroke="#3a88e2"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="0" y="-17">
(20.5,90.55)
</text>
</g>
<g class="point" transform="translate(23.9,29.67)">
<circle cx="0" cy="0" fill="#5095e5" r="5" stroke="#3a88e2"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="0" y="-17">
(95.6,40.66)
</text>
</g>
</g>"##;

        let x_scale = LinearScale::new(0.0, 200.0, 0, 50);
        let y_scale = LinearScale::new(0.0, 100.0, 50, 0);
        let data = vec![(20.5, 90.55), (95.6, 40.66)];
        let scatter = ScatterView::new(x_scale, y_scale)
            .set_data(&data)
            .expect("unable to set data");
        let scatter_svg = scatter.to_svg();
        assert_eq!(scatter_svg.to_string(), expected_svg_group);
    }
}
