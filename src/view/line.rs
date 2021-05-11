use crate::color::{COLOR_HEX_BLUE_1, COLOR_HEX_BLUE_2};
use crate::render::svg::*;
use crate::shape::point::Point;
use crate::{BandScale, Color, Error, LinearScale, PointLabelPosition, PointType, Scale};
use svg::Node;

const DEFAULT_LABEL_VISIBLE: bool = true;
const DEFAULT_LABEL_POSITION: PointLabelPosition = PointLabelPosition::Top;

const DEFAULT_POINT_TYPE: PointType = PointType::Circle;
const DEFAULT_POINT_VISIBLE: bool = true;

const DEFAULT_LINE_STROKE_WIDTH: i32 = 2;

/// LineView represents a single line.
#[derive(Clone)]
pub struct LineView {
    x_scale: BandScale,
    y_scale: LinearScale,
    stroke_color: String,
    point_fill_color: String,
    point_stroke_color: String,
    points: Vec<Point>,
    point_type: PointType,
    point_visible: bool,
    point_label_visible: bool,
    point_label_position: PointLabelPosition,
}

impl LineView {
    /// Create a new LineView.
    pub fn new(x_scale: BandScale, y_scale: LinearScale) -> Self {
        Self {
            x_scale,
            y_scale,
            stroke_color: COLOR_HEX_BLUE_1.to_string(),
            point_fill_color: COLOR_HEX_BLUE_2.to_string(),
            point_stroke_color: COLOR_HEX_BLUE_1.to_string(),
            points: Vec::new(),
            point_type: DEFAULT_POINT_TYPE,
            point_visible: DEFAULT_POINT_VISIBLE,
            point_label_visible: DEFAULT_LABEL_VISIBLE,
            point_label_position: DEFAULT_LABEL_POSITION,
        }
    }

    /// Set line stroke color.
    pub fn set_stroke_color(mut self, stroke_color: Color) -> Self {
        self.stroke_color = stroke_color.to_string();
        self
    }

    /// Set fill color for the point.
    pub fn set_point_fill_color(mut self, point_fill_color: Color) -> Self {
        self.point_fill_color = point_fill_color.to_string();
        self
    }

    /// Set stroke color for the point.
    pub fn set_point_stroke_color(mut self, point_stroke_color: Color) -> Self {
        self.point_stroke_color = point_stroke_color.to_string();
        self
    }

    /// Set type of the point.
    pub fn set_point_type(mut self, point_type: PointType) -> Self {
        self.point_type = point_type;
        self
    }

    /// Set point visibility.
    pub fn set_point_visible(mut self, point_visible: bool) -> Self {
        self.point_visible = point_visible;
        self
    }

    /// Set point label visibility.
    pub fn set_point_label_visible(mut self, point_label_visible: bool) -> Self {
        self.point_label_visible = point_label_visible;
        self
    }

    /// Set label position for the point.
    pub fn set_point_label_position(mut self, point_label_position: PointLabelPosition) -> Self {
        self.point_label_position = point_label_position;
        self
    }

    /// Set data for line points.
    pub fn set_data(mut self, data: &[f32]) -> Result<Self, Error> {
        if data.is_empty() {
            return Err(Error::DataIsEmpty);
        }
        if data.len() != self.x_scale.ticks().len() {
            return Err(Error::CategoriesCountDoesntEqual);
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

        let categories = self.x_scale.ticks();
        let mut points = Vec::new();
        for (idx, value) in data.iter().enumerate() {
            let category = &categories[idx];
            let scaled_x = &self.x_scale.scale(category);
            let scaled_y = self.y_scale.scale(&value);

            let point = Point::new(
                scaled_x + x_bandwidth_offset,
                scaled_y + y_bandwidth_offset,
                self.point_type,
                DEFAULT_POINT_SIZE,
                &value.to_string(),
                &self.point_fill_color.to_string(),
                &self.point_stroke_color.to_string(),
            )
            .set_point_visible(self.point_visible)
            .set_label_visible(self.point_label_visible)
            .set_label_position(self.point_label_position);
            points.push(point);
        }
        self.points = points;

        Ok(self)
    }

    /// Get line SVG representation.
    pub fn to_svg(&self) -> svg::node::element::Group {
        let mut res = svg::node::element::Group::new();
        let mut data = svg::node::element::path::Data::new();

        for (point_idx, point) in self.points.iter().enumerate() {
            if point_idx == 0 {
                data = data.move_to((point.x(), point.y()));
            } else {
                data = data.line_to((point.x(), point.y()));
            }

            res.append(point.to_svg());
        }
        let line = svg::node::element::Path::new()
            .set(FILL_ATTR, FILL_NONE)
            .set(STROKE_ATTR, self.stroke_color.clone())
            .set(STROKE_WIDTH_ATTR, DEFAULT_LINE_STROKE_WIDTH)
            .set(D_ATTR, data);

        res.append(line);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

    #[test]
    fn vertical_bar_basic() {
        let expected_svg_group = r##"<g>
<g class="point" transform="translate(13.414631,99.01)">
<g>
<line stroke="#ffffff" stroke-width="2px" x1="-5" x2="5" y1="-5" y2="5"/>
<line stroke="#ffffff" stroke-width="2px" x1="5" x2="-5" y1="-5" y2="5"/>
</g>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="0" y="17">
89.1
</text>
</g>
<g class="point" transform="translate(37.804874,99.865555)">
<g>
<line stroke="#ffffff" stroke-width="2px" x1="-5" x2="5" y1="-5" y2="5"/>
<line stroke="#ffffff" stroke-width="2px" x1="5" x2="-5" y1="-5" y2="5"/>
</g>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="0" y="17">
12.1
</text>
</g>
<g class="point" transform="translate(62.195118,99.5)">
<g>
<line stroke="#ffffff" stroke-width="2px" x1="-5" x2="5" y1="-5" y2="5"/>
<line stroke="#ffffff" stroke-width="2px" x1="5" x2="-5" y1="-5" y2="5"/>
</g>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="0" y="17">
45
</text>
</g>
<g class="point" transform="translate(86.585365,99.76667)">
<g>
<line stroke="#ffffff" stroke-width="2px" x1="-5" x2="5" y1="-5" y2="5"/>
<line stroke="#ffffff" stroke-width="2px" x1="5" x2="-5" y1="-5" y2="5"/>
</g>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="0" y="17">
21
</text>
</g>
<path d="M13.414631,99.01 L37.804874,99.865555 L62.195118,99.5 L86.585365,99.76667" fill="none" stroke="#ff006c" stroke-width="2"/>
</g>"##;

        let x_scale = BandScale::new(
            vec![
                "a".to_string(),
                "n1".to_string(),
                "n2".to_string(),
                "y".to_string(),
            ],
            0,
            100,
        );
        let y_scale = LinearScale::new(0_f32, 9000_f32, 100, 0);
        let data = vec![89.1_f32, 12.1_f32, 45_f32, 21_f32];
        let line = LineView::new(x_scale, y_scale)
            .set_point_label_position(PointLabelPosition::Bottom)
            .set_stroke_color(Color::new_from_hex("#ff006c"))
            .set_point_fill_color(Color::new_from_hex("#ffe5f5"))
            .set_point_type(PointType::X)
            .set_point_stroke_color(Color::new_from_hex("#ffffff"))
            .set_data(&data)
            .expect("unable to set data");
        let line_svg = line.to_svg();
        assert_eq!(line_svg.to_string(), expected_svg_group);
    }
}
