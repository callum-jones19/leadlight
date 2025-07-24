use nih_plug::buffer::Buffer;

pub fn process_algorithm(buffer: &mut Buffer) {
    for samples in buffer.iter_samples() {
        for sample in samples {
            *sample = 0.0;
        }
    }
}
