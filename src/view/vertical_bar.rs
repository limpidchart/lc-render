use crate::BandScale;
use crate::BarsValues;
use crate::Error;
use crate::LinearScale;
use crate::Orientation;
use crate::Scale;
use crate::{Bar, BarLabelPosition};
use std::collections::HashMap;
use svg::node::Node;

const DEFAULT_BAR_LABEL_VISIBLE: bool = true;
const DEFAULT_BAR_LABEL_POSITION: BarLabelPosition = BarLabelPosition::Center;

/// VerticalBarView represents a chart view with vertical bars.
#[derive(Clone)]
pub struct VerticalBarView {
    x_scale: BandScale,
    y_scale: LinearScale,
    bars: Vec<Bar>,
    bar_label_visible: bool,
    bar_label_position: BarLabelPosition,
}

impl VerticalBarView {
    /// Create a new VerticalBarView.
    pub fn new(x_scale: BandScale, y_scale: LinearScale) -> Self {
        Self {
            x_scale,
            y_scale,
            bars: Vec::new(),
            bar_label_visible: DEFAULT_BAR_LABEL_VISIBLE,
            bar_label_position: DEFAULT_BAR_LABEL_POSITION,
        }
    }

    /// Configure label visibility for bars.
    pub fn set_bar_label_visible(mut self, bar_label_visible: bool) -> Self {
        self.bar_label_visible = bar_label_visible;
        self
    }

    /// Configure label position for bars.
    pub fn set_bar_label_position(mut self, bar_label_position: BarLabelPosition) -> Self {
        self.bar_label_position = bar_label_position;
        self
    }

    /// Set values for bars.
    pub fn set_data(mut self, bars_values: &[BarsValues]) -> Result<Self, Error> {
        if bars_values.is_empty() {
            return Err(Error::DataIsEmpty);
        }

        // Populate a map of category to tuples of (value, fill_color, stroke_color).
        let mut bars_categories = HashMap::new();
        for bv_opts in bars_values.iter() {
            if bv_opts.values().len() > self.x_scale.ticks().len() {
                return Err(Error::CategoriesCountIsLess);
            }
            for (i, value) in bv_opts.values().iter().enumerate() {
                let category = self.x_scale.ticks()[i].clone();
                bars_categories
                    .entry(category)
                    .or_insert_with(|| vec![(value, bv_opts.fill_color(), bv_opts.stroke_color())]);
            }
        }

        // Create vector of bars from the bars_categories map.
        let mut bars = Vec::new();
        for (category, category_entries) in bars_categories.iter() {
            let mut value_acc = 0_f32;
            let mut start = self.y_scale.scale(&value_acc);
            let mut end = start;

            for category_entry in category_entries.iter() {
                let value = category_entry.0;
                let fill_color = category_entry.1;
                let stroke_color = category_entry.2;

                value_acc += value;
                if self.y_scale.is_range_reversed() {
                    end = start;
                    start = self.y_scale.scale(&value_acc);
                } else {
                    start = end;
                    end = self.y_scale.scale(&value_acc);
                }

                let bar = Bar::new(
                    start,
                    end,
                    *value,
                    self.x_scale.bandwidth(),
                    self.x_scale.scale(&category.to_string()),
                    Orientation::Vertical,
                )
                .set_fill_color(fill_color)
                .set_stroke_color(stroke_color)
                .set_label_visible(self.bar_label_visible)
                .set_label_position(self.bar_label_position);
                bars.push(bar);
            }
        }
        self.bars = bars;

        Ok(self)
    }

    /// Get bar view SVG representation.
    pub fn to_svg(&self) -> svg::node::element::Group {
        let mut res = svg::node::element::Group::new();

        for bar in self.bars.iter() {
            res.append(bar.to_svg());
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{COLOR_HEX_BLUE_2, COLOR_HEX_BLUE_4};
    use crate::Color;

    #[test]
    fn vertical_bar_basic() {
        let expected_svg_group = r##"<g>
<g class="bar" transform="translate(3.2258034,0)">
<rect fill="#5095e5" height="66" shape-rendering="crispEdges" stroke="#1960b2" stroke-width="1" width="29.032257" x="0" y="34"/>
<text dy=".35em" fill="#080808" font-family="sans-serif" font-size="14px" text-anchor="middle" x="14.516129" y="67">
66
</text>
</g>
</g>"##;

        let x_scale = BandScale::new(
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            0,
            100,
        );
        let y_scale = LinearScale::new(0_f32, 100_f32, 100, 0);
        let data = vec![BarsValues::new(vec![66_f32])
            .set_fill_color(Color::new_from_hex(COLOR_HEX_BLUE_4))
            .set_stroke_color(Color::new_from_hex(COLOR_HEX_BLUE_2))];
        let vertical_bar = VerticalBarView::new(x_scale, y_scale)
            .set_data(&data)
            .expect("unable to set data");
        let vertical_bar_svg = vertical_bar.to_svg();
        assert_eq!(vertical_bar_svg.to_string(), expected_svg_group);
    }
}
