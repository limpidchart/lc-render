use lc_render::{BandScale, BarsValues, Chart, Color, LinearScale, VerticalBarView};

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
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ],
        0,
        width - margin_left - margin_right,
    );

    // Configure vertical scale.
    // LinearScale range is inverted because SVG coordinate system's origin is at left top corner.
    let y_scale = LinearScale::new(0.0, 100.0, height - margin_top - margin_bottom, 0);

    // Prepare vertical bars data.
    let data = vec![
        BarsValues::new(vec![92_f32, 12_f32, 34.8_f32, 24_f32, 2.5_f32])
            .set_fill_color(Color::new_from_hex("#36896e"))
            .set_stroke_color(Color::new_from_hex("#0c513b")),
    ];
    let view = VerticalBarView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data)
        .expect("unable to set data");

    let chart = Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margin_top(margin_top)
        .set_margin_bottom(margin_bottom)
        .set_margin_left(margin_left)
        .set_margin_right(margin_right)
        .set_axis_bottom_band(x_scale)
        .set_axis_left_linear(y_scale)
        .set_axis_bottom_label("Categories")
        .set_axis_left_label("Values")
        .set_title("Vertical Bar Chart")
        .add_view(&view);

    chart
        .save("./examples/svg/vertical_bar_chart.svg")
        .expect("unable to save ./svg/vertical_bar_chart.svg");
}
