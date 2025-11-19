use core::f32;

use nih_plug::{buffer::Buffer, plugin::ProcessStatus};

const CHANNEL_INCORRECT_LEN_ERR: &'static str =
    "Channel feedback values were not properly initialised for Lowpass";

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
///
/// Returns `ProcessStatus`
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
    let iir_amount: f64 = cast_lowpass.clamp(0.0, 1.0).powi(2);

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
                    return ProcessStatus::Error(CHANNEL_INCORRECT_LEN_ERR);
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
    use nih_plug::plugin::ProcessStatus;
    use plugin_utils::create_two_channel_buffer;

    use crate::process::{CHANNEL_INCORRECT_LEN_ERR, process_lowpass};

    #[test]
    /// Initialise a buffer with 512 samples of value 10.0
    /// Applying the lowpass processing function with a `lowpass_value > 1` should
    /// clamp the `lowpass_value` to 1 and cause it to not affect the input buffer
    fn lowpass_excessive_amount() {
        let sample_init_val = 10.0;

        let mut real_buffers = vec![vec![sample_init_val; 512]; 2];
        let mut buffer = create_two_channel_buffer(&mut real_buffers).unwrap();
        let mut channel_feedback_values: Vec<f64> = vec![0.0, 0.0];

        let proc_status = process_lowpass(&mut buffer, 2.0, &mut channel_feedback_values);

        assert_eq!(proc_status, ProcessStatus::Normal);
        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                assert_eq!(*sample, 10.0);
            }
        }
    }

    /// Initialise a buffer with 512 samples of value 10.0
    /// Applying the lowpass processing function with a `lowpass_value < 0` should
    /// clamp the `lowpass_value` to 0 and cause it to convert all samples to 0
    #[test]
    fn lowpass_underflow_amount() {
        let sample_init_val = 10.0;

        let mut real_buffers = vec![vec![sample_init_val; 512]; 2];
        let mut buffer = create_two_channel_buffer(&mut real_buffers).unwrap();
        let mut channel_feedback_values: Vec<f64> = vec![0.0, 0.0];

        let proc_status = process_lowpass(&mut buffer, -1.0, &mut channel_feedback_values);

        assert_eq!(proc_status, ProcessStatus::Normal);
        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                assert_eq!(*sample, 0.0);
            }
        }
    }

    /// Initialise a buffer with samples set to 10.0, and length 1000
    /// Test that when the `lowpass_amount` is set to 1.0, the lowpass does not
    /// affect the output.
    #[test]
    fn lowpass_amount_1() {
        let sample_init_val = 10.0;

        let mut real_buffers = vec![vec![sample_init_val; 512]; 2];
        let mut buffer = create_two_channel_buffer(&mut real_buffers).unwrap();

        let mut channel_feedback_values: Vec<f64> = vec![0.0, 0.0];

        let proc_status = process_lowpass(&mut buffer, 1.0, &mut channel_feedback_values);

        assert_eq!(proc_status, ProcessStatus::Normal);
        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                assert_eq!(*sample, 10.0);
            }
        }
    }

    /// Check that when the `lowpass_amount` is set to 0, the output is silent
    /// (nothing should make it past the filter threshold).
    #[test]
    fn lowpass_amount_0() {
        let sample_init_val = 10.0;

        let mut real_buffers = vec![vec![sample_init_val; 512]; 2];
        let mut buffer = create_two_channel_buffer(&mut real_buffers).unwrap();

        let mut channel_feedback_values: Vec<f64> = vec![0.0, 0.0];

        let proc_status = process_lowpass(&mut buffer, 0.0, &mut channel_feedback_values);

        assert_eq!(proc_status, ProcessStatus::Normal);
        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                assert_eq!(*sample, 0.0);
            }
        }
    }

    /// Test that the plugin correctly returns `ProcessStatus::Error` if the
    /// channel feedback values list is incorrectly sized.
    #[test]
    fn lowpass_incorrect_channel_feedback_vec_size() {
        let sample_init_val = 10.0;

        let mut real_buffers = vec![vec![sample_init_val; 512]; 2];
        let mut buffer = create_two_channel_buffer(&mut real_buffers).unwrap();
        let mut channel_feedback_values: Vec<f64> = vec![0.0];

        let proc_status = process_lowpass(&mut buffer, 0.0, &mut channel_feedback_values);

        assert_eq!(proc_status, ProcessStatus::Error(CHANNEL_INCORRECT_LEN_ERR));
    }
}
