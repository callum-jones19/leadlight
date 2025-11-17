use core::f32;

use nih_plug::{buffer::Buffer, plugin::ProcessStatus};

/// An infinite impulse response (IIR) lowpass filter. This filter operates in
/// the time domain, using a recursive, moving-average algorithm to calculate
/// each sample value. Doing this levels out higher frequencies, and affects
/// lower frequencies less, giving the desired lowpass effect. To achieve this,
/// the algorithm keeps track of an accumulated `iir_sample` value for each channel,
/// and replaces each sample with a weighted combination of the previous samples
/// and current sample. The lower the value of `lowpass_amount`, the stronger
/// the weighting of the `iir_sample` in the accumulated sample value, and the
/// weaker the value of the current sample. This makes the 'averaging' effect
/// stronger, and therefore creates a stronger lowpass effect.
///
/// When `lowpass_amount` is set to 1.0, the output value should be unchanged.
/// At 0.0, the output value should always be 0.
pub fn process_lowpass(
    buffer: &mut Buffer,
    lowpass_amount: f32,
    channel_feedback_values: &mut Vec<f64>,
) -> ProcessStatus {
    // At the Nyquist frequency, this should be 1.
    // This should never exceed 1 or go below 0.
    let cast_lowpass: f64 = lowpass_amount.into();

    // Take the power of the value to make its effect scale a bit more
    // aggressively. It should never go above 1 or 0, but clamp guarantees
    // this in case something goes wrong with input handling, etc.
    let iir_amount: f64 = cast_lowpass.powi(3).clamp(0.0, 1.0);

    for channel_samples in buffer.iter_samples() {
        for (channel_index, current_sample) in channel_samples.into_iter().enumerate() {
            // Take the value of the current sample, and weight its value with
            // the feedback values in this channel.
            // The lower the lowpass threshold (i.e., the lower the `lowpass_amount`),
            // the more weight will be given to the feedback value, and the less
            // weight will be given to the current sample.
            let cast_curr_sample: f64 = (*current_sample).into();

            // Catch edge-case if a feedback value hasn't been initialised properly
            // in the channel_feedback_vals list.
            let channel_feedback = match channel_feedback_values.get_mut(channel_index) {
                Some(channel_feedback) => channel_feedback,
                None => {
                    return ProcessStatus::Error(
                        "Channel feedback values were not properly initialised for Lowpass",
                    );
                }
            };

            // Combine the weighted values
            let weighted_feedback_val: f64 = *channel_feedback * (1.0 - iir_amount);
            let weighted_curr_sample: f64 = cast_curr_sample * iir_amount;
            *channel_feedback = weighted_feedback_val + weighted_curr_sample;

            // Output to buffer
            *current_sample = channel_feedback_values[channel_index] as f32;
        }
    }

    ProcessStatus::Normal
}

#[cfg(test)]
mod tests {
    #[test]
    /// Create a two-channel buffer filled with 512 samples of value 5.0.
    /// Test that this buffer has been validly created. Then apply the
    /// processing algorithm to the buffer and verify that every sample in
    /// the modified buffer is 0.0.
    fn mute_buffer_512_long() {
        todo!()
    }
}
