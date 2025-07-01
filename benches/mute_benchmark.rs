mod utils;

use crate::utils::test_utils::create_test_buffer;
use criterion::{Criterion, criterion_group, criterion_main};
use thesis_vst::Mute;

fn base_benchmark(c: &mut Criterion) {
    let sample_init_val = 5.0;
    let empty_noise_plug = Mute::default();

    let mut real_buffers = vec![vec![sample_init_val; 1024]; 2];
    let mut buffer = create_test_buffer(&mut real_buffers).unwrap();

    c.bench_function("base mute", |b| {
        b.iter(|| {
            empty_noise_plug.process_algorithm(&mut buffer);
        });
    });
}

criterion_group!(benches, base_benchmark);
criterion_main!(benches);
