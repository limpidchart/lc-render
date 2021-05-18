use lc_render::{BandScale, BarsValues, Chart, Color, LinearScale, VerticalBarView};

fn main() {
    let width = 800;
    let height = 1000;

    let margin_top = 90;
    let margin_bottom = 50;
    let margin_left = 60;
    let margin_right = 40;

    let x_scale = BandScale::new(
        vec![
            "Russia".to_string(),
            "Germany".to_string(),
            "Netherlands".to_string(),
            "Canada".to_string(),
            "United States".to_string(),
            "Australia".to_string(),
        ],
        0,
        width - margin_left - margin_right,
    );

    let y_scale = LinearScale::new(0.0, 450.0, height - margin_top - margin_bottom, 0);

    // Data from numbeo.com.
    let data = vec![
        // Cost of living index.
        BarsValues::new(vec![33.17, 70.62, 78.64, 70.08, 71.92, 84.14])
            .set_fill_color(Color::new_from_hex("#01629c"))
            .set_stroke_color(Color::new_from_hex("#00296f")),
        // Rent index.
        BarsValues::new(vec![9.77, 29.64, 39.31, 32.48, 41.14, 38.38])
            .set_fill_color(Color::new_from_hex("#00fff9"))
            .set_stroke_color(Color::new_from_hex("#00a2c5")),
        // Cost of living plus rent index.
        BarsValues::new(vec![21.99, 51.04, 59.85, 52.12, 57.21, 62.28])
            .set_fill_color(Color::new_from_hex("#3f962c"))
            .set_stroke_color(Color::new_from_hex("#13761f")),
        // Groceries index.
        BarsValues::new(vec![27.81, 54.69, 61.63, 68.50, 70.24, 81.14])
            .set_fill_color(Color::new_from_hex("#5eab2e"))
            .set_stroke_color(Color::new_from_hex("#168523")),
        // Restaurant price index.
        BarsValues::new(vec![30.65, 65.00, 81.62, 63.96, 69.42, 76.28])
            .set_fill_color(Color::new_from_hex("#ffa700"))
            .set_stroke_color(Color::new_from_hex("#ff7400")),
        // Local purchasing power index.
        BarsValues::new(vec![34.61, 93.72, 83.89, 82.76, 102.58, 99.29])
            .set_fill_color(Color::new_from_hex("#ffce00"))
            .set_stroke_color(Color::new_from_hex("#ff8d00")),
    ];
    let view = VerticalBarView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data)
        .expect("setting data");

    let chart = Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margin_top(margin_top)
        .set_margin_bottom(margin_bottom)
        .set_margin_left(margin_left)
        .set_margin_right(margin_right)
        .set_axis_bottom_band(x_scale)
        .set_axis_left_linear(y_scale)
        .set_title("Cost of living index")
        .add_view(&view);

    chart
        .save("./examples/svg/stacked_vertical_bar_chart.svg")
        .expect("unable to save ./examples/svg/stacked_vertical_bar_chart.svg");
}
