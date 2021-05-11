use crate::render::svg::*;
use crate::shape::axis::{Axis, AxisPosition};
use crate::{
    AreaView, BandScale, Error, HorizontalBarView, LineView, LinearScale, ScatterView,
    VerticalBarView,
};
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
    area_views: Vec<AreaView>,
    horizontal_bar_views: Vec<HorizontalBarView>,
    line_views: Vec<LineView>,
    scatter_views: Vec<ScatterView>,
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
            area_views: Vec::new(),
            horizontal_bar_views: Vec::new(),
            line_views: Vec::new(),
            scatter_views: Vec::new(),
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

    /// Add a single AreaView to chart.
    pub fn add_area_view(mut self, view: AreaView) -> Self {
        self.area_views.push(view);
        self
    }

    /// Add a single HorizontalBarView to chart.
    pub fn add_horizontal_bar_view(mut self, view: HorizontalBarView) -> Self {
        self.horizontal_bar_views.push(view);
        self
    }

    /// Add a single LineView to chart.
    pub fn add_line_view(mut self, view: LineView) -> Self {
        self.line_views.push(view);
        self
    }

    /// Add a single ScatterView to chart.
    pub fn add_scatter_view(mut self, view: ScatterView) -> Self {
        self.scatter_views.push(view);
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
        for view in self.area_views.iter() {
            views_group.append(view.to_svg());
        }
        for view in self.horizontal_bar_views.iter() {
            views_group.append(view.to_svg());
        }
        for view in self.line_views.iter() {
            views_group.append(view.to_svg());
        }
        for view in self.scatter_views.iter() {
            views_group.append(view.to_svg());
        }
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
