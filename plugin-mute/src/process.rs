use nih_plug::buffer::Buffer;

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

    use crate::process::process_algorithm;


    #[test]
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

        // Now pass the buffer through the processing algorithm, which should
        // assign every sample value to be 0.
        process_algorithm(&mut buffer);

        // Verify the updated values
        for samples in buffer.iter_samples() {
            for sample in samples {
                assert_eq!(*sample, 0.0);
            }
        }
    }

    #[test]
    fn mute_buffer_0_long() {
        let sample_init_val = 5.0;

        let mut real_buffers = vec![vec![sample_init_val; 0]; 2];
        let mut buffer = create_two_channel_buffer(&mut real_buffers).unwrap();

        // Verify that the buffer is what we expect it to be
        for samples in buffer.iter_samples() {
            for sample in samples {
                assert_eq!(*sample, sample_init_val);
            }
        }

        // Now pass the buffer through the processing algorithm, which should
        // assign every sample value to be 0.
        process_algorithm(&mut buffer);

        // Verify the updated values
        for samples in buffer.iter_samples() {
            for sample in samples {
                assert_eq!(*sample, 0.0);
            }
        }
    }

    #[test]
    fn mute_buffer_1024_long() {
        let sample_init_val = 34.0;

        let mut real_buffers = vec![vec![sample_init_val; 1024]; 2];
        let mut buffer = create_two_channel_buffer(&mut real_buffers).unwrap();

        // Verify that the buffer is what we expect it to be
        for samples in buffer.iter_samples() {
            for sample in samples {
                assert_eq!(*sample, sample_init_val);
            }
        }

        // Now pass the buffer through the processing algorithm, which should
        // assign every sample value to be 0.
        process_algorithm(&mut buffer);

        // Verify the updated values
        for samples in buffer.iter_samples() {
            for sample in samples {
                assert_eq!(*sample, 0.0);
            }
        }
    }
}

