use lc_render::{Chart, Color, LinearScale, PointType, ScatterView};

fn main() {
    let width = 800;
    let height = 600;

    let margin_top = 90;
    let margin_bottom = 50;
    let margin_left = 60;
    let margin_right = 40;

    let x_scale = LinearScale::new(0.0, 200.0, 0, width - margin_left - margin_right);
    let y_scale = LinearScale::new(0.0, 100.0, height - margin_top - margin_bottom, 0);

    let data_1 = vec![
        (20_f32, 90_f32),
        (12_f32, 54_f32),
        (25_f32, 70_f32),
        (33_f32, 40_f32),
    ];
    let data_2 = vec![
        (120_f32, 10_f32),
        (143_f32, 34_f32),
        (170_f32, 14_f32),
        (190_f32, 13_f32),
    ];

    let view_1 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_point_fill_color(Color::new_from_hex("#808080"))
        .set_point_stroke_color(Color::new_from_hex("#000000"))
        .set_data(&data_1)
        .expect("setting data_1");
    let view_2 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_point_fill_color(Color::new_from_hex("#000000"))
        .set_point_stroke_color(Color::new_from_hex("#808080"))
        .set_point_type(PointType::Square)
        .set_data(&data_2)
        .expect("setting data_2");

    let chart = Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margin_top(margin_top)
        .set_margin_bottom(margin_bottom)
        .set_margin_left(margin_left)
        .set_margin_right(margin_right)
        .set_axis_bottom_linear(x_scale)
        .set_axis_left_linear(y_scale)
        .set_axis_bottom_label("X Values")
        .set_axis_left_label("Y Values")
        .set_title("Two Scatters Chart")
        .add_view(&view_1)
        .add_view(&view_2);

    chart
        .save("./examples/svg/two_scatters_chart.svg")
        .expect("saving ./examples/svg/two_scatters_chart.svg");
}
