use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use lc_render::{AreaView, BandScale, Chart, LinearScale};

const SIZE: i32 = 800;
const MARGIN: i32 = 40;

fn create_area_chart(values_count: usize) {
    let x_scale = BandScale::new(
        (0..values_count).map(|v| v.to_string()).collect(),
        0,
        SIZE - MARGIN,
    )
    .set_no_boundaries_offset(true)
    .set_inner_padding(0.0)
    .set_outer_padding(0.0);
    let y_scale = LinearScale::new(0_f32, 200_f32, SIZE - MARGIN, 0);
    let data = vec![24_f32; values_count];
    let view = AreaView::new(x_scale.clone(), y_scale.clone())
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
        .set_axis_bottom_label("X Values")
        .set_axis_left_label("Y Values")
        .set_title("Area Chart")
        .add_view(&view);
}

fn area_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("area_chart");
    for size in [100, 1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &values_count| {
                b.iter(|| create_area_chart(values_count));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, area_chart);
criterion_main!(benches);
