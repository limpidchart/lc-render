use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use lc_render::{Chart, LinearScale, ScatterView};

const SIZE: i32 = 800;
const MARGIN: i32 = 40;

fn create_scatter_chart(values_count: usize) {
    let x_scale = LinearScale::new(0_f32, 200_f32, SIZE - MARGIN, 0);
    let y_scale = LinearScale::new(0_f32, 200_f32, SIZE - MARGIN, 0);
    let data = vec![(16_f32, 32_f32); values_count];
    let view = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data)
        .expect("unable to set data");
    Chart::new()
        .set_width(SIZE)
        .set_height(SIZE)
        .set_margin_top(MARGIN)
        .set_margin_bottom(MARGIN)
        .set_margin_left(MARGIN)
        .set_margin_right(MARGIN)
        .set_axis_top_linear(x_scale.clone())
        .set_axis_bottom_linear(x_scale)
        .set_axis_left_linear(y_scale.clone())
        .set_axis_right_linear(y_scale)
        .set_axis_bottom_label("X Values")
        .set_axis_left_label("Y Values")
        .set_title("Scatter Chart")
        .add_view(&view);
}

fn scatter_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("scatter_chart");
    for size in [100, 1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &values_count| {
                b.iter(|| create_scatter_chart(values_count));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, scatter_chart);
criterion_main!(benches);
