use crate::render::svg::*;
use crate::shape::point::Point;
use svg::Node;

/// Area shape.
#[derive(Clone)]
pub struct Area {
    points: Vec<Point>,
    fill_color: String,
    stroke_color: String,
}

impl Area {
    /// Create a new Area.
    pub fn new(points: Vec<Point>, fill_color: &str, stroke_color: &str) -> Self {
        Self {
            points,
            fill_color: fill_color.to_string(),
            stroke_color: stroke_color.to_string(),
        }
    }

    /// Get area SVG representation.
    pub fn to_svg(&self) -> svg::node::element::Group {
        let mut res = svg::node::element::Group::new().set(CLASS_ATTR, CLASS_AREA);

        let mut data = svg::node::element::path::Data::new();
        for (point_idx, point) in self.points.iter().enumerate() {
            if point_idx == 0 {
                data = data.move_to((point.x(), point.y()));
            } else {
                data = data.line_to((point.x(), point.y()));
            }

            res.append(point.to_svg());
        }
        data = data.close();

        let area = svg::node::element::Path::new()
            .set(FILL_ATTR, self.fill_color.as_ref())
            .set(STROKE_ATTR, self.stroke_color.as_ref())
            .set(D_ATTR, data);
        res.append(area);
        for point in self.points.iter() {
            res.append(point.to_svg());
        }

        res
    }
}

impl Default for Area {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            fill_color: String::new(),
            stroke_color: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::point::PointType;

    #[test]
    fn bar_basic() {
        let expected_svg_group = r##"<g class="area">
<g class="point" transform="translate(10,20)">
<circle cx="0" cy="0" fill="#ed5d74" r="1" stroke="#e6121f"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="0" y="-13">
12
</text>
</g>
<g class="point" transform="translate(30,40)">
<circle cx="0" cy="0" fill="#ed5d74" r="1" stroke="#e6121f"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="0" y="-13">
12
</text>
</g>
<path d="M10,20 L30,40 z" fill="#e93620" stroke="#370725"/>
<g class="point" transform="translate(10,20)">
<circle cx="0" cy="0" fill="#ed5d74" r="1" stroke="#e6121f"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="0" y="-13">
12
</text>
</g>
<g class="point" transform="translate(30,40)">
<circle cx="0" cy="0" fill="#ed5d74" r="1" stroke="#e6121f"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="0" y="-13">
12
</text>
</g>
</g>"##;

        let area_svg = Area::new(
            vec![
                Point::new(
                    10_f32,
                    20_f32,
                    PointType::Circle,
                    1,
                    &12.to_string(),
                    "#ed5d74",
                    "#e6121f",
                ),
                Point::new(
                    30_f32,
                    40_f32,
                    PointType::Circle,
                    1,
                    &12.to_string(),
                    "#ed5d74",
                    "#e6121f",
                ),
            ],
            "#e93620",
            "#370725",
        )
        .to_svg();

        assert_eq!(area_svg.to_string(), expected_svg_group);
    }
}
