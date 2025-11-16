use core::f32;

use nih_plug::buffer::Buffer;
use rand::Rng;

pub fn process_lowpass(buffer: &mut Buffer, lowpass_amount: f32, channel_data: &mut Vec<f64>) {
    // TODO why do we need this calculation?
    // At the Nyquist frequency, this should be 1.
    // This should never exceed 1 or go below 0.
    let cast_lowpass: f64 = lowpass_amount.into();
    // let mut iir_amount: f64 = (cast_lowpass.powi(2) + cast_lowpass) / 2.0;
    // iir_amount += iir_amount;
    // iir_amount = iir_amount.clamp(0.0, 1.0);
    let iir_amount: f64 = cast_lowpass / 5.0;

    // The IIR filter value. We define this outside the loop so that we can
    // accumulate the value as we iterate over the buffer. This allows us to
    // create the IIR filter.
    //
    // We start with 0.0 because the assumption with an IIR convolution is that
    // the signal is padded on either side with 0 (not repeating like an FIR
    // would assume)
    // let mut iir_sample_channels: Vec<f64> = vec![0.0; buffer.channels()];
    for channel_samples in buffer.iter_samples() {
        for (channel_index, current_sample) in channel_samples.into_iter().enumerate() {
            let cast_curr_sample: f64 = (*current_sample).into();
            let weighted_iir_sample: f64 = channel_data[channel_index] * (1.0 - iir_amount);
            let weighted_curr_sample: f64 = cast_curr_sample * iir_amount;
            channel_data[channel_index] = weighted_iir_sample + weighted_curr_sample;

            // Output to buffer
            *current_sample = channel_data[channel_index] as f32;
        }
    }
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
