#[cfg(test)]
pub mod test_utils {
    use nih_plug::buffer::Buffer;

    /// Create a buffer to be used in tests. It should have two channels, and
    /// will fill to mirror the given real_buffers. Currently it will return
    /// an error if it encournters problems validating real_buffers, but in the
    /// future I would like to make these typings more robust to enforce
    /// the correct inputs at a compilation level.
    pub fn create_test_buffer(real_buffers: &mut Vec<Vec<f32>>) -> Result<Buffer, String> {
        // This section of unsafe code is directly pulled from nih-plug's internal
        // buffer tests. For now, I will assume this is therefore a verified
        // implementation.

        // Ensure that the supplied buffer has two channels
        if real_buffers.len() != 2 {
            return Err(String::from(
                "Test buffer received did not receive an input vec with two channels",
            ));
        }

        let num_samples = match real_buffers.first() {
            Some(first) => first.len(),
            None => return Err(String::from("No value in real_buffers input")),
        };

        let mut buffer: Buffer = Buffer::default();
        unsafe {
            buffer.set_slices(num_samples, |output_slices| {
                let (first_channel, other_channels) = real_buffers.split_at_mut(1);
                *output_slices = vec![&mut first_channel[0], &mut other_channels[0]];
            })
        };

        Ok(buffer)
    }
}
