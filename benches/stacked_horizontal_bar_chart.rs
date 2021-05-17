use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use lc_render::{BandScale, BarsValues, Chart, HorizontalBarView, LinearScale};

const SIZE: i32 = 800;
const MARGIN: i32 = 40;

fn create_stacked_horizontal_bar_chart(values_count: usize) {
    let x_scale = LinearScale::new(0.0, 200.0, SIZE - MARGIN, 0);
    let y_scale = BandScale::new(
        (0..values_count).map(|v| v.to_string()).collect(),
        0,
        SIZE - MARGIN,
    );
    let data = vec![
        BarsValues::new(vec![2_f32; values_count]),
        BarsValues::new(vec![4_f32; values_count]),
        BarsValues::new(vec![8_f32; values_count]),
        BarsValues::new(vec![16_f32; values_count]),
        BarsValues::new(vec![32_f32; values_count]),
        BarsValues::new(vec![32_f32; values_count]),
        BarsValues::new(vec![16_f32; values_count]),
        BarsValues::new(vec![8_f32; values_count]),
        BarsValues::new(vec![4_f32; values_count]),
        BarsValues::new(vec![2_f32; values_count]),
    ];
    let view = HorizontalBarView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data)
        .expect("unable to set data");
    Chart::new()
        .set_width(SIZE)
        .set_height(SIZE)
        .set_margin_top(MARGIN)
        .set_margin_bottom(MARGIN)
        .set_margin_left(MARGIN)
        .set_margin_right(MARGIN)
        .set_axis_bottom_linear(x_scale)
        .set_axis_left_band(y_scale)
        .set_axis_bottom_label("Values")
        .set_axis_left_label("Categories")
        .set_title("Stacked Horizontal Bar Chart")
        .add_view(&view);
}

fn stacked_horizontal_bar_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("stacked_horizontal_bar_chart");
    for size in [100, 1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &values_count| {
                b.iter(|| create_stacked_horizontal_bar_chart(values_count));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, stacked_horizontal_bar_chart);
criterion_main!(benches);
