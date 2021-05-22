use lc_render::{
    BandScale, BarsValues, Chart, Color, LineView, LinearScale, PointType, VerticalBarView,
};

fn main() {
    // Configure document size.
    let width = 800;
    let height = 600;

    // Configure document margins.
    let margin_top = 90;
    let margin_bottom = 50;
    let margin_left = 60;
    let margin_right = 40;

    // Configure horizontal scale.
    let x_scale = BandScale::new(
        vec![
            "2015".to_string(),
            "2016".to_string(),
            "2017".to_string(),
            "2018".to_string(),
            "2019".to_string(),
            "2020".to_string(),
            "2021".to_string(),
        ],
        0,
        width - margin_left - margin_right,
    );

    // Configure vertical scale.
    // LinearScale range is inverted because SVG coordinate system's origin is at left top corner.
    let y_scale = LinearScale::new(0.0, 85.0, height - margin_top - margin_bottom, 0);

    // Data from numbeo.com.
    let values = vec![74.72, 66.34, 64.18, 74.32, 68.62, 64.76, 72.52];
    let vertical_bar_data = vec![BarsValues::new(values.clone())
        .set_fill_color(Color::new_from_hex("#77ab59"))
        .set_stroke_color(Color::new_from_hex("#36802d"))];
    let vertical_bar_view = VerticalBarView::new(x_scale.clone(), y_scale.clone())
        .set_data(&vertical_bar_data)
        .expect("unable to set vertical bar data");
    let line_view = LineView::new(x_scale.clone(), y_scale.clone())
        .set_stroke_color(Color::new_from_hex("#234d20"))
        .set_point_stroke_color(Color::new_from_hex("#234d20"))
        .set_point_fill_color(Color::new_from_hex("#234d20"))
        .set_point_label_visible(false)
        .set_point_type(PointType::Circle)
        .set_data(&values)
        .expect("unable to set line data");

    let chart = Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margin_top(margin_top)
        .set_margin_bottom(margin_bottom)
        .set_margin_left(margin_left)
        .set_margin_right(margin_right)
        .set_axis_bottom_band(x_scale)
        .set_axis_left_linear(y_scale)
        .set_title("Cost of living index in Berlin")
        .set_views(vec![&vertical_bar_view, &line_view]);

    chart
        .save("./examples/svg/line_and_vertical_bar_chart.svg")
        .expect("unable to save ./svg/line_and_vertical_chart.svg");
}
