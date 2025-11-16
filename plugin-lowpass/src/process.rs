use core::f32;

use nih_plug::buffer::Buffer;

pub fn process_lowpass(buffer: &mut Buffer, lowpass_amount: f32) {
    // TODO why do we need this calculation?
    // At the Nyquist frequency, this should be 1.
    let iir_amount: f32 = (lowpass_amount.powi(2) + lowpass_amount) / 2.0;

    // The IIR filter value. We define this outside the loop so that we can
    // accumulate the value as we iterate over the buffer. This allows us to
    // create the IIR filter.
    //
    // We start with 0.0 because the assumption with an IIR convolution is that
    // the signal is padded on either side with 0.
    let mut iir_sample: f32 = 0.0;

    for channel_samples in buffer.iter_samples() {
        for current_sample in channel_samples {
            iir_sample = (iir_sample * (1.0 - iir_amount)) + (*current_sample * iir_amount);
            *current_sample = iir_sample;
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
