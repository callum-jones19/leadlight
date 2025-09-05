use nih_plug::buffer::Buffer;

/// Create a two-channel audio buffer from a 2D Array with 2 entries on the
/// y axis. This function wraps around the unsafe `Buffer::set_slices` function
/// to provide a safe implementation to programatically define a buffer with
/// a set float input.
///
/// ## Examples
///
/// ```
/// use plugin_utils::create_two_channel_buffer;
///
/// let sample_init_val = 34.0;
///
/// let mut real_buffers = vec![vec![sample_init_val; 1024]; 2];
/// let mut buffer = create_two_channel_buffer(&mut real_buffers).unwrap();
///
/// // Verify that the buffer is what we expect it to be
/// for samples in buffer.iter_samples() {
///     for sample in samples {
///         assert_eq!(*sample, sample_init_val);
///     }
/// }
/// ```
pub fn create_two_channel_buffer(real_buffers: &'_ mut [Vec<f32>]) -> Result<Buffer<'_>, String> {
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
