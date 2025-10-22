use nih_plug::buffer::Buffer;

/// The mathematical algorithm applied to the mutable input buffer.
/// This takes in a generic buffer, and iterates over the samples in
/// each channel. For each sample, the algorithm sets the
/// floating-point value of the sampel to 0.0.
pub fn process_algorithm(buffer: &mut Buffer) {
    for samples in buffer.iter_samples() {
        for sample in samples {
            *sample = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use plugin_utils::create_two_channel_buffer;

    #[test]
    /// Create a two-channel buffer filled with 512 samples of value 5.0.
    /// Test that this buffer has been validly created. Then apply the
    /// processing algorithm to the buffer and verify that every sample in
    /// the modified buffer is 0.0.
    fn mute_buffer_512_long() {
        let sample_init_val = 5.0;

        let mut real_buffers = vec![vec![sample_init_val; 512]; 2];
        let mut buffer = create_two_channel_buffer(&mut real_buffers).unwrap();

        // Verify that the buffer is what we expect it to be
        for samples in buffer.iter_samples() {
            for sample in samples {
                assert_eq!(*sample, sample_init_val);
            }
        }
    }
}
