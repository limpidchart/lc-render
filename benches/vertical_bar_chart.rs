use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use lc_render::{BandScale, BarsValues, Chart, LinearScale, VerticalBarView};

const SIZE: i32 = 800;
const MARGIN: i32 = 40;

fn create_vertical_bar_chart(values_count: usize) {
    let x_scale = BandScale::new(
        (0..values_count).map(|v| v.to_string()).collect(),
        0,
        SIZE - MARGIN,
    );
    let y_scale = LinearScale::new(0.0, 100.0, SIZE - MARGIN, 0);
    let data = vec![BarsValues::new(vec![24_f32; values_count])];
    let view = VerticalBarView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data)
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
        .set_axis_bottom_label("Categories")
        .set_axis_left_label("Values")
        .set_title("Vertical Bar Chart")
        .add_view(&view);
}

fn vertical_bar_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("vertical_bar_chart");
    for size in [100, 1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &values_count| {
                b.iter(|| create_vertical_bar_chart(values_count));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, vertical_bar_chart);
criterion_main!(benches);
