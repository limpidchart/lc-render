use lc_render::{BandScale, BarsValues, Chart, Color, HorizontalBarView, LinearScale};

fn main() {
    // Configure document size.
    let width = 800;
    let height = 600;

    // Configure document margins.
    let margin_top = 90;
    let margin_bottom = 50;
    let margin_left = 60;
    let margin_right = 40;

    // Configure vertical scale.
    let y_scale = BandScale::new(
        vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ],
        0,
        height - margin_top - margin_bottom,
    );

    // Configure horizontal scale.
    // LinearScale range is inverted because SVG coordinate system's origin is at left top corner.
    let x_scale = LinearScale::new(0.0, 100.0, 0, width - margin_left - margin_right);

    // Prepare vertical bars data.
    let data = vec![
        BarsValues::new(vec![92_f32, 12_f32, 34.8_f32, 24_f32, 9.5_f32])
            .set_fill_color(Color::new_from_hex("#898fd5"))
            .set_stroke_color(Color::new_from_hex("#2c2663")),
    ];
    let view = HorizontalBarView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data)
        .expect("unable to set data");

    let chart = Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margin_top(margin_top)
        .set_margin_bottom(margin_bottom)
        .set_margin_left(margin_left)
        .set_margin_right(margin_right)
        .set_axis_bottom_linear(x_scale)
        .set_axis_left_band(y_scale)
        .set_axis_bottom_label("Categories")
        .set_axis_left_label("Values")
        .set_title("Horizontal Bar Chart")
        .add_horizontal_bar_view(view);

    chart
        .save("./examples/svg/horizontal_chart.svg")
        .expect("unable to save ./svg/horizontal_chart.svg");
}
