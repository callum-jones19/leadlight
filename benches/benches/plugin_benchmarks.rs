use criterion::{Criterion, criterion_group, criterion_main};
use plugin_utils::create_test_buffer;

fn base_benchmark(c: &mut Criterion) {
    let sample_init_val = 5.0;

    let mut real_buffers = vec![vec![sample_init_val; 1024]; 2];
    let mut buffer = create_test_buffer(&mut real_buffers).unwrap();

    c.bench_function("base mute", |b| {
        b.iter(|| {
            plugin_mute::process::process_algorithm(&mut buffer);
        });
    });
}

criterion_group!(benches, base_benchmark);
criterion_main!(benches);
