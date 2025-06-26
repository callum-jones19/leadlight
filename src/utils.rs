#[cfg(test)]
pub mod test_utils {
    use nih_plug::buffer::Buffer;

    pub fn create_test_buffer(real_buffers: &mut Vec<Vec<f32>>) -> Buffer {
        // This section of unsafe code is directly pulled from nih-plug's internal
        // buffer tests. For now, I will assume this is therefore a verified
        // implementation.

        let mut buffer: Buffer = Buffer::default();
        unsafe {
            buffer.set_slices(512, |output_slices| {
                let (first_channel, other_channels) = real_buffers.split_at_mut(1);
                *output_slices = vec![&mut first_channel[0], &mut other_channels[0]];
            })
        };

        buffer
    }
}
