use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use lc_render::{BandScale, BarsValues, Chart, HorizontalBarView, LinearScale};

const SIZE: i32 = 800;
const MARGIN: i32 = 40;

fn create_horizontal_bar_chart(values_count: usize) {
    let x_scale = LinearScale::new(0.0, 100.0, SIZE - MARGIN, 0);
    let y_scale = BandScale::new(
        (0..values_count).map(|v| v.to_string()).collect(),
        0,
        SIZE - MARGIN,
    );
    let data = vec![BarsValues::new(vec![24_f32; values_count])];
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
        .set_title("Horizontal Bar Chart")
        .add_view(&view);
}

fn horizontal_bar_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("horizontal_bar_chart");
    for size in [100, 1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &values_count| {
                b.iter(|| create_horizontal_bar_chart(values_count));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, horizontal_bar_chart);
criterion_main!(benches);
