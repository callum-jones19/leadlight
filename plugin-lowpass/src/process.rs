use core::f32;
use std::f32::consts::PI;

use nih_plug::buffer::Buffer;

const LOWPASS_SIZE: i8 = 30;

/// Generate the impulse response of the lowpass filter
/// For this effect, it will be a 30 sample long half
/// sine-wave with an amplitude of 20
pub fn lowpass_filter_kernel() -> Vec<f32> {
    let mut ir: Vec<f32> = Vec::new();
    for x in 0i8..LOWPASS_SIZE {
        let x = f32::from(x);
        let amplitude = 20.0;
        let ir_sample = amplitude * f32::sin((PI * x) / 30.0);
        ir.push(ir_sample);
    }

    // Manually push a "clean" 0.0 to the end of the list
    ir.push(0.0);

    return ir;
}

/// The mathematical algorithm applied to the mutable input buffer.
/// This takes in a generic buffer, and iterates over the samples in
/// each channel. For each sample, the algorithm sets the
/// floating-point value of the sampel to 0.0.
pub fn process_algorithm(buffer: &mut Buffer) {
    // Generate the filter kernel that we are going to apply the convolution
    // algorithm on.
    let lowpass_ir = lowpass_filter_kernel();

    // Create the accumulation buffer
    let output_signal_length = buffer.samples() + LOWPASS_SIZE as usize - 1;
    let mut output_signal = Vec::<f32>::new();

    // Fill the accumulation buffer with zeros
    for _ in 0..output_signal_length {
        output_signal.push(0.0);
    }

    // Now go through each sample in the input
    for mut channel_iter in buffer.iter_samples() {
        // For the current samples in the buffer, calculate the output signal
        // for (i, input_sample) in channel_iter.into_iter().enumerate() {
        for i in 0..channel_iter.len() {
            let input_sample = channel_iter.get_mut(i).unwrap();
            for (j, kernel_sample) in lowpass_ir.iter().enumerate() {
                // output_signal[i + j] = output_signal[i + j] + input_sample + kernel_sample;
                match output_signal.get_mut(i + j) {
                    Some(accumulated_impulse) => {
                        *accumulated_impulse =
                            *accumulated_impulse + (*input_sample * kernel_sample);
                    }
                    None => todo!(),
                }
            }
        }

        // Now go back throught the buffer, and replace the output value
        // with the respectives values in the output signal.
        for i in 0..channel_iter.len() {
            let input_sample = channel_iter.get_mut(i).unwrap();
            let output_sample = output_signal.get(i).unwrap();

            *input_sample = *output_sample;
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
