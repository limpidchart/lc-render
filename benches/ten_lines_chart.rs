use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use lc_render::{BandScale, Chart, LineView, LinearScale};

const SIZE: i32 = 800;
const MARGIN: i32 = 40;

fn create_many_lines_chart(values_count: usize) {
    let x_scale = BandScale::new(
        (0..values_count).map(|v| v.to_string()).collect(),
        0,
        SIZE - MARGIN,
    )
    .set_no_boundaries_offset(true)
    .set_inner_padding(0.0)
    .set_outer_padding(0.0);
    let y_scale = LinearScale::new(0_f32, 200_f32, SIZE - MARGIN, 0);

    let data_1 = vec![2_f32; values_count];
    let data_2 = vec![4_f32; values_count];
    let data_3 = vec![8_f32; values_count];
    let data_4 = vec![16_f32; values_count];
    let data_5 = vec![32_f32; values_count];
    let data_6 = vec![15_f32; values_count];
    let data_7 = vec![14_f32; values_count];
    let data_8 = vec![13_f32; values_count];
    let data_9 = vec![12_f32; values_count];
    let data_10 = vec![1_f32; values_count];

    let line_1 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_1)
        .expect("unable to set data");
    let line_2 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_2)
        .expect("unable to set data");
    let line_3 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_3)
        .expect("unable to set data");
    let line_4 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_4)
        .expect("unable to set data");
    let line_5 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_5)
        .expect("unable to set data");
    let line_6 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_6)
        .expect("unable to set data");
    let line_7 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_7)
        .expect("unable to set data");
    let line_8 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_8)
        .expect("unable to set data");
    let line_9 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_9)
        .expect("unable to set data");
    let line_10 = LineView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_10)
        .expect("unable to set data");

    Chart::new()
        .set_width(SIZE)
        .set_height(SIZE)
        .set_margin_top(MARGIN)
        .set_margin_bottom(MARGIN)
        .set_margin_left(MARGIN)
        .set_margin_right(MARGIN)
        .set_axis_bottom_band(x_scale)
        .set_axis_left_linear(y_scale)
        .set_axis_bottom_label("X Values")
        .set_axis_left_label("Y Values")
        .set_title("Lines Chart")
        .add_view(&line_1)
        .add_view(&line_2)
        .add_view(&line_3)
        .add_view(&line_4)
        .add_view(&line_5)
        .add_view(&line_6)
        .add_view(&line_7)
        .add_view(&line_8)
        .add_view(&line_9)
        .add_view(&line_10);
}

fn many_lines_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("ten_lines_chart");
    for size in [100, 1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &values_count| {
                b.iter(|| create_many_lines_chart(values_count));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, many_lines_chart);
criterion_main!(benches);
