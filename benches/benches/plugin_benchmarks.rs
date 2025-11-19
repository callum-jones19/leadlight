use criterion::{Criterion, criterion_group, criterion_main};
use plugin_utils::create_two_channel_buffer;

fn mute_benchmark(c: &mut Criterion) {
    let sample_init_val = 5.0;

    let mut real_buffers = vec![vec![sample_init_val; 4410000]; 2];
    let mut buffer = create_two_channel_buffer(&mut real_buffers).unwrap();

    c.bench_function("mute_process", |b| {
        b.iter(|| {
            plugin_mute::process::process_algorithm(&mut buffer);
        });
    });
}

fn lowpass_benchmark(c: &mut Criterion) {
    let sample_init_val = 5.0;

    // Let's make this mimic an actual buffer size
    // 44100 sample rate = 44100 samples per second = 44100 * 10 for a 100 second
    // song, = buffer size of 441000
    let mut real_buffers = vec![vec![sample_init_val; 4410000]; 2];
    let mut buffer = create_two_channel_buffer(&mut real_buffers).unwrap();
    let mut channel_feedback_values: Vec<f64> = vec![0.0, 0.0];

    c.bench_function("lowpass_process", |b| {
        b.iter(|| {
            plugin_lowpass::process::process_lowpass(&mut buffer, 0.3, &mut channel_feedback_values);
        });
    });
}

criterion_group!(benches, mute_benchmark, lowpass_benchmark);
criterion_main!(benches);
