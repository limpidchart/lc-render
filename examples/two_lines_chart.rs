use lc_render::color::{COLOR_HEX_BLUE_1, COLOR_HEX_BLUE_4};
use lc_render::{BandScale, Chart, Color, LineView, LinearScale, PointLabelPosition};

fn main() {
    let width = 1000;
    let height = 800;

    let margin_top = 30;
    let margin_bottom = 40;
    let margin_left = 40;
    let margin_right = 20;

    let x_scale = BandScale::new(
        vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
            "G".to_string(),
            "H".to_string(),
        ],
        0,
        width - margin_left - margin_right,
    )
    .set_no_boundaries_offset(true)
    .set_inner_padding(0.0)
    .set_outer_padding(0.0);

    let y_scale = LinearScale::new(0.0, 200.0_f32, height - margin_top - margin_bottom, 0);

    let data_1 = vec![
        20_f32, 70_f32, 130_f32, 180_f32, 20_f32, 77_f32, 140_f32, 190_f32,
    ];
    let view_1 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_point_label_position(PointLabelPosition::BottomLeft)
        .set_stroke_color(Color::new_from_hex(COLOR_HEX_BLUE_1))
        .set_point_visible(false)
        .set_data(&data_1)
        .expect("setting data");

    let data_2 = vec![
        54_f32, 40_f32, 50_f32, 77_f32, 91_f32, 53_f32, 11_f32, 3_f32,
    ];
    let view_2 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_point_label_position(PointLabelPosition::TopRight)
        .set_stroke_color(Color::new_from_hex(COLOR_HEX_BLUE_4))
        .set_point_visible(false)
        .set_data(&data_2)
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
        .set_title("Two Lines Chart")
        .add_line_view(view_1)
        .add_line_view(view_2);

    chart
        .save("./examples/svg/two_lines_chart.svg")
        .expect("unable to save ./examples/svg/saving two_lines_chart.svg");
}
