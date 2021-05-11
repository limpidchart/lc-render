use lc_render::Chart;
use lc_render::LinearScale;
use lc_render::PointLabelPosition;
use lc_render::ScatterView;

fn main() {
    let width = 800;
    let height = 600;

    let margin_top = 90;
    let margin_bottom = 50;
    let margin_left = 60;
    let margin_right = 40;

    let x_scale = LinearScale::new(0.0, 200.0, 0, width - margin_left - margin_right);
    let y_scale = LinearScale::new(0.0, 100.0, height - margin_top - margin_bottom, 0);

    let data = vec![
        (20.1, 54.11),
        (70.2, 40.22),
        (130.3, 50.33),
        (170.4, 70.44),
        (20.5, 90.55),
        (95.6, 40.66),
        (130.7, 12.77),
        (170.8, 2.88),
    ];

    let view = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_point_label_position(PointLabelPosition::TopRight)
        .set_data(&data)
        .expect("setting data");

    let chart = Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margin_top(margin_top)
        .set_margin_bottom(margin_bottom)
        .set_margin_left(margin_left)
        .set_margin_right(margin_right)
        .set_axis_top_linear(x_scale.clone())
        .set_axis_bottom_linear(x_scale)
        .set_axis_left_linear(y_scale.clone())
        .set_axis_right_linear(y_scale)
        .set_axis_bottom_label("X Values")
        .set_axis_left_label("Y Values")
        .set_title("Single Scatter Chart")
        .add_scatter_view(view);

    chart
        .save("./examples/svg/single_scatter_chart.svg")
        .expect("saving ./examples/svg/single_scatter_chart.svg");
}
