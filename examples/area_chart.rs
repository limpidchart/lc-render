use lc_render::{AreaView, BandScale, Chart, Color, LinearScale, PointLabelPosition, PointType};

fn main() {
    let width = 1200;
    let height = 700;

    let margin_top = 20;
    let margin_bottom = 70;
    let margin_left = 40;
    let margin_right = 30;

    let x_scale = BandScale::new(
        vec![
            "a1".to_string(),
            "a2".to_string(),
            "a3".to_string(),
            "a4".to_string(),
            "a5".to_string(),
            "a6".to_string(),
        ],
        0,
        width - margin_left - margin_right,
    )
    .set_no_boundaries_offset(true)
    .set_inner_padding(0.0)
    .set_outer_padding(0.0);
    let y_scale = LinearScale::new(0_f32, 200_f32, height - margin_top - margin_bottom, 0);

    let data = vec![12_f32, 100_f32, 120_f32, 180_f32, 40_f32, 8_f32];

    let view = AreaView::new(x_scale.clone(), y_scale.clone())
        .set_point_label_position(PointLabelPosition::TopRight)
        .set_point_stroke_color(Color::new_from_hex("#ff7400"))
        .set_point_fill_color(Color::new_from_hex("#ff7400"))
        .set_stroke_color(Color::new_from_hex("#ff8d00"))
        .set_fill_color(Color::new_from_hex("#ffa700"))
        .set_point_type(PointType::Square)
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
        .set_axis_bottom_label("X Values")
        .set_axis_left_label("Y Values")
        .set_title("Single Area Chart")
        .add_view(&view);

    chart
        .save("./examples/svg/area_chart.svg")
        .expect("unable to save ./examples/svg/area_chart.svg");
}
