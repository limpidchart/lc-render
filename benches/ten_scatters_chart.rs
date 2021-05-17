use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use lc_render::{Chart, LinearScale, ScatterView};

const SIZE: i32 = 800;
const MARGIN: i32 = 40;

fn create_many_scatters_chart(values_count: usize) {
    let x_scale = LinearScale::new(0_f32, 200_f32, SIZE - MARGIN, 0);
    let y_scale = LinearScale::new(0_f32, 200_f32, SIZE - MARGIN, 0);

    let data_1 = vec![(2_f32, 64_f32); values_count];
    let data_2 = vec![(4_f32, 64_f32); values_count];
    let data_3 = vec![(8_f32, 64_f32); values_count];
    let data_4 = vec![(16_f32, 64_f32); values_count];
    let data_5 = vec![(32_f32, 64_f32); values_count];
    let data_6 = vec![(15_f32, 64_f32); values_count];
    let data_7 = vec![(14_f32, 64_f32); values_count];
    let data_8 = vec![(13_f32, 64_f32); values_count];
    let data_9 = vec![(12_f32, 64_f32); values_count];
    let data_10 = vec![(1_f32, 64_f32); values_count];

    let scatter_1 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_1)
        .expect("unable to set data");
    let scatter_2 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_2)
        .expect("unable to set data");
    let scatter_3 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_3)
        .expect("unable to set data");
    let scatter_4 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_4)
        .expect("unable to set data");
    let scatter_5 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_5)
        .expect("unable to set data");
    let scatter_6 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_6)
        .expect("unable to set data");
    let scatter_7 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_7)
        .expect("unable to set data");
    let scatter_8 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_8)
        .expect("unable to set data");
    let scatter_9 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_9)
        .expect("unable to set data");
    let scatter_10 = ScatterView::new(x_scale.clone(), y_scale.clone())
        .set_data(&data_10)
        .expect("unable to set data");

    Chart::new()
        .set_width(SIZE)
        .set_height(SIZE)
        .set_margin_top(MARGIN)
        .set_margin_bottom(MARGIN)
        .set_margin_left(MARGIN)
        .set_margin_right(MARGIN)
        .set_axis_bottom_linear(x_scale)
        .set_axis_left_linear(y_scale)
        .set_axis_bottom_label("X Values")
        .set_axis_left_label("Y Values")
        .set_title("Scatters Chart")
        .add_view(&scatter_1)
        .add_view(&scatter_2)
        .add_view(&scatter_3)
        .add_view(&scatter_4)
        .add_view(&scatter_5)
        .add_view(&scatter_6)
        .add_view(&scatter_7)
        .add_view(&scatter_8)
        .add_view(&scatter_9)
        .add_view(&scatter_10);
}

fn many_scatters_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("ten_scatters_chart");
    for size in [100, 1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &values_count| {
                b.iter(|| create_many_scatters_chart(values_count));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, many_scatters_chart);
criterion_main!(benches);
