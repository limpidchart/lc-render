use crate::render::svg::*;
use crate::shape::axis::{Axis, AxisPosition};
use crate::Error;
use crate::{BandScale, LinearScale, VerticalBarView};
use std::path::Path;
use svg::Node;

const DEFAULT_MARGIN_TOP: i32 = 90;
const DEFAULT_MARGIN_BOTTOM: i32 = 50;
const DEFAULT_MARGIN_LEFT: i32 = 60;
const DEFAULT_MARGIN_RIGHT: i32 = 40;
const DEFAULT_WIDTH: i32 = 800;
const DEFAULT_HEIGHT: i32 = 600;

const DEFAULT_TITLE_FONT_SIZE: &str = "24px";
const DEFAULT_TITLE_Y_TRANSFORM: i32 = 25;

/// Chart represents a single document with one or more views, axes and a title.
/// It will also contain grid and legend in the future.
pub struct Chart {
    margin_top: i32,
    margin_bottom: i32,
    margin_left: i32,
    margin_right: i32,
    width: i32,
    height: i32,
    x_axis_top: Option<Axis>,
    x_axis_bottom: Option<Axis>,
    y_axis_left: Option<Axis>,
    y_axis_right: Option<Axis>,
    vertical_bar_views: Vec<VerticalBarView>,
    title: String,
}

impl Chart {
    /// Create a new chart.
    pub fn new() -> Self {
        Chart {
            margin_top: DEFAULT_MARGIN_TOP,
            margin_bottom: DEFAULT_MARGIN_BOTTOM,
            margin_left: DEFAULT_MARGIN_LEFT,
            margin_right: DEFAULT_MARGIN_RIGHT,
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            x_axis_top: None,
            x_axis_bottom: None,
            y_axis_left: None,
            y_axis_right: None,
            vertical_bar_views: Vec::new(),
            title: String::new(),
        }
    }

    /// Get chart width that can be used for views.
    pub fn view_width(&self) -> i32 {
        self.width - self.margin_left - self.margin_right
    }

    /// Get chart height that can be used for views.
    pub fn view_height(&self) -> i32 {
        self.height - self.margin_top - self.margin_bottom
    }

    /// Set chart top margin.
    pub fn set_margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = margin_top;
        self
    }

    /// Set chart bottom margin.
    pub fn set_margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = margin_bottom;
        self
    }

    /// Set chart left margin.
    pub fn set_margin_left(mut self, margin_left: i32) -> Self {
        self.margin_left = margin_left;
        self
    }

    /// Set chart right margin.
    pub fn set_margin_right(mut self, margin_right: i32) -> Self {
        self.margin_right = margin_right;
        self
    }

    /// Set chart width.
    pub fn set_width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    /// Set chart height.
    pub fn set_height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }

    /// Set BandScale for top axis.
    pub fn set_axis_top_band(mut self, scale: BandScale) -> Self {
        self.x_axis_top = Some(Axis::new(
            &scale,
            AxisPosition::Top,
            self.view_width(),
            self.view_height(),
        ));
        self
    }

    /// Set LinearScale for top axis.
    pub fn set_axis_top_linear(mut self, scale: LinearScale) -> Self {
        self.x_axis_top = Some(Axis::new(
            &scale,
            AxisPosition::Top,
            self.view_width(),
            self.view_height(),
        ));
        self
    }

    /// Set BandScale for bottom axis.
    pub fn set_axis_bottom_band(mut self, scale: BandScale) -> Self {
        self.x_axis_bottom = Some(Axis::new(
            &scale,
            AxisPosition::Bottom,
            self.view_width(),
            self.view_height(),
        ));
        self
    }

    /// Set LinearScale for bottom axis.
    pub fn set_axis_bottom_linear(mut self, scale: LinearScale) -> Self {
        self.x_axis_bottom = Some(Axis::new(
            &scale,
            AxisPosition::Bottom,
            self.view_width(),
            self.view_height(),
        ));
        self
    }

    /// Set BandScale for left axis.
    pub fn set_axis_left_band(mut self, scale: BandScale) -> Self {
        self.y_axis_left = Some(Axis::new(
            &scale,
            AxisPosition::Left,
            self.view_width(),
            self.view_height(),
        ));
        self
    }

    /// Set LinearScale for left axis.
    pub fn set_axis_left_linear(mut self, scale: LinearScale) -> Self {
        self.y_axis_left = Some(Axis::new(
            &scale,
            AxisPosition::Left,
            self.view_width(),
            self.view_height(),
        ));
        self
    }

    /// Set BandScale for right axis.
    pub fn set_axis_right_band(mut self, scale: BandScale) -> Self {
        self.y_axis_right = Some(Axis::new(
            &scale,
            AxisPosition::Right,
            self.view_width(),
            self.view_height(),
        ));
        self
    }

    /// Set LinearScale for right axis.
    pub fn set_axis_right_linear(mut self, scale: LinearScale) -> Self {
        self.y_axis_right = Some(Axis::new(
            &scale,
            AxisPosition::Right,
            self.view_width(),
            self.view_height(),
        ));
        self
    }

    /// Set label for top axis.
    pub fn set_axis_top_label(mut self, label: &str) -> Self {
        if let Some(ref mut axis) = self.x_axis_top {
            axis.set_label(label);
        }
        self
    }

    /// Set label for bottom axis.
    pub fn set_axis_bottom_label(mut self, label: &str) -> Self {
        if let Some(ref mut axis) = self.x_axis_bottom {
            axis.set_label(label);
        }
        self
    }

    /// Set label for left axis.
    pub fn set_axis_left_label(mut self, label: &str) -> Self {
        if let Some(ref mut axis) = self.y_axis_left {
            axis.set_label(label);
        }
        self
    }

    /// Set label for right axis.
    pub fn set_axis_right_label(mut self, label: &str) -> Self {
        if let Some(ref mut axis) = self.y_axis_right {
            axis.set_label(label);
        }
        self
    }

    /// Set chart title.
    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Add a single VerticalBarView to chart.
    pub fn add_vertical_bar_view(mut self, view: VerticalBarView) -> Self {
        self.vertical_bar_views.push(view);
        self
    }

    /// Get chart SVG representation.
    pub fn to_svg(&self) -> svg::Document {
        let mut res = svg::node::element::Group::new().set(CLASS_ATTR, CLASS_CHART);

        // Add axes.
        if let Some(ref axis) = self.x_axis_top {
            let mut axis_group = axis.to_svg();
            axis_group.assign(
                TRANSFORM_ATTR,
                translate_x_y(self.margin_left, self.margin_top),
            );
            res.append(axis_group);
        };
        if let Some(ref axis) = self.x_axis_bottom {
            let mut axis_group = axis.to_svg();
            axis_group.assign(
                TRANSFORM_ATTR,
                translate_x_y(self.margin_left, self.height - self.margin_bottom),
            );
            res.append(axis_group);
        };
        if let Some(ref axis) = self.y_axis_left {
            let mut axis_group = axis.to_svg();
            axis_group.assign(
                TRANSFORM_ATTR,
                translate_x_y(self.margin_left, self.margin_top),
            );
            res.append(axis_group);
        };
        if let Some(ref axis) = self.y_axis_right {
            let mut axis_group = axis.to_svg();
            axis_group.assign(
                TRANSFORM_ATTR,
                translate_x_y(self.width - self.margin_right, self.margin_top),
            );
            res.append(axis_group);
        };

        // Add views.
        let mut views_group = svg::node::element::Group::new()
            .set(CLASS_ATTR, CLASS_VIEWS)
            .set(
                TRANSFORM_ATTR,
                translate_x_y(self.margin_left, self.margin_top),
            );
        for view in self.vertical_bar_views.iter() {
            views_group.append(view.to_svg());
        }
        res.append(views_group);

        // Add title.
        if !self.title.is_empty() {
            let title_group = svg::node::element::Group::new()
                .set(CLASS_ATTR, CLASS_TITLE)
                .set(
                    TRANSFORM_ATTR,
                    translate_x_y(self.width / 2, DEFAULT_TITLE_Y_TRANSFORM),
                )
                .add(
                    svg::node::element::Text::new()
                        .set(X_ATTR, START)
                        .set(Y_ATTR, START)
                        .set(DY_ATTR, DEFAULT_DY)
                        .set(FILL_ATTR, DEFAULT_FONT_COLOR)
                        .set(TEXT_ANCHOR_ATTR, TEXT_ANCHOR_MIDDLE)
                        .set(FONT_SIZE_ATTR, DEFAULT_TITLE_FONT_SIZE)
                        .set(FONT_FAMILY_ATTR, DEFAULT_FONT_FAMILY)
                        .add(svg::node::Text::new(&self.title)),
                );
            res.append(title_group);
        }

        svg::Document::new()
            .set(WIDTH_ATTR, self.width)
            .set(HEIGHT_ATTR, self.height)
            .set(VIEW_BOX_ATTR, (START, START, self.width, self.height))
            .add(res)
    }

    /// Save chart to SVG file at the specified path.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        svg::save(path, &self.to_svg())?;

        Ok(())
    }
}

impl Default for Chart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{COLOR_HEX_GREEN_1, COLOR_HEX_GREEN_5};
    use crate::{BarsValues, Color};

    #[test]
    fn vertical_bar_chart_basic() {
        let expected_svg_group = r##"<svg height="2000" viewBox="0 0 1000 2000" width="1000" xmlns="http://www.w3.org/2000/svg">
<g class="chart">
<g class="x-axis" transform="translate(20,1970)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="930" y1="0" y2="0"/>
<g class="tick" transform="translate(50,0)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="0" y1="0" y2="6"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="middle" transform="rotate(0,0,16)" x="0" y="16">
single_category
</text>
</g>
<text fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" transform="rotate(0)" x="465" y="42">
Categories
</text>
</g>
<g class="y-axis" transform="translate(20,10)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="0" y1="0" y2="1960"/>
<g class="tick" transform="translate(0,100)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
500
</text>
</g>
<g class="tick" transform="translate(0,90)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
550
</text>
</g>
<g class="tick" transform="translate(0,80)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
600
</text>
</g>
<g class="tick" transform="translate(0,70)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
650
</text>
</g>
<g class="tick" transform="translate(0,60)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
700
</text>
</g>
<g class="tick" transform="translate(0,50)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
750
</text>
</g>
<g class="tick" transform="translate(0,39.999996)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
800
</text>
</g>
<g class="tick" transform="translate(0,30)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
850
</text>
</g>
<g class="tick" transform="translate(0,20)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
900
</text>
</g>
<g class="tick" transform="translate(0,10)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
950
</text>
</g>
<g class="tick" transform="translate(0,0)">
<line shape-rendering="crispEdges" stroke="#bbbbbb" stroke-width="1" x1="0" x2="-6" y1="0" y2="0"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="12px" text-anchor="end" transform="rotate(0,-12,0)" x="-12" y="0">
1000
</text>
</g>
<text fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" transform="rotate(-90)" x="-980" y="-42">
Values
</text>
</g>
<g class="views" transform="translate(20,10)">
<g>
<g class="bar" transform="translate(9.090912,0)">
<rect fill="#038d05" height="179" shape-rendering="crispEdges" stroke="#0c3300" stroke-width="1" width="81.81818" x="0" y="21"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="40.90909" y="110.5">
895
</text>
</g>
</g>
</g>
<g class="title" transform="translate(500,25)">
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="24px" text-anchor="middle" x="0" y="0">
Vertical Bar Chart
</text>
</g>
</g>
</svg>"##;

        let x_scale = BandScale::new(vec!["single_category".to_string()], 0, 100);
        let y_scale = LinearScale::new(500_f32, 1000_f32, 100, 0);
        let data = vec![BarsValues::new(vec![895_f32])
            .set_fill_color(Color::new_from_hex(COLOR_HEX_GREEN_5))
            .set_stroke_color(Color::new_from_hex(COLOR_HEX_GREEN_1))];
        let view = VerticalBarView::new(x_scale.clone(), y_scale.clone())
            .set_data(&data)
            .expect("unable to set data");
        let chart_svg = Chart::new()
            .set_width(1000)
            .set_height(2000)
            .set_margin_top(10)
            .set_margin_bottom(30)
            .set_margin_left(20)
            .set_margin_right(50)
            .set_axis_bottom_band(x_scale)
            .set_axis_left_linear(y_scale)
            .set_axis_bottom_label("Categories")
            .set_axis_left_label("Values")
            .set_title("Vertical Bar Chart")
            .add_vertical_bar_view(view)
            .to_svg();

        assert_eq!(chart_svg.to_string(), expected_svg_group);
    }
}
