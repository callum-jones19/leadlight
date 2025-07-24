use std::sync::Arc;

use nih_plug::prelude::*;

/// A plugin that takes any input, and then always provides an empty output.
/// This is effectively like a mute
#[derive(Default)]
pub struct Mute {
    params: Arc<MuteParams>,
}

pub fn process_algorithm(buffer: &mut Buffer) {
    for samples in buffer.iter_samples() {
        for sample in samples {
            *sample = 0.0;
        }
    }
}

/// All the paramters for the EmptyNoise plugin are held in a single struct.
/// This allows us to apply the `Params` macro to the struct itself, which then
/// allows nih_plug to
///
/// We also want to return this struct in the params function of the Plugin trait.
#[derive(Params, Default)]
struct MuteParams {}

impl Plugin for Mute {
    const NAME: &'static str = "Mute";
    const VENDOR: &'static str = "Callum Jones";
    const URL: &'static str = "TODO";
    const EMAIL: &'static str = "TODO";

    const VERSION: &'static str = "0.1.0";

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By
        // default these names are generated as needed. This layout will be called
        // 'stero'.
        names: PortNames::const_default(),
    }];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> std::sync::Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        process_algorithm(buffer);
        ProcessStatus::Normal
    }
}

impl Vst3Plugin for Mute {
    const VST3_CLASS_ID: [u8; 16] = *b"EmptyNoisePlugin";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Tools];
}

nih_export_vst3!(Mute);

/*
Testing modules
*/

#[cfg(test)]
mod tests {
    use plugin_utils::create_test_buffer;

    use crate::process_algorithm;

    #[test]
    fn mute_buffer_512_long() {
        let sample_init_val = 5.0;

        let mut real_buffers = vec![vec![sample_init_val; 512]; 2];
        let mut buffer = create_test_buffer(&mut real_buffers).unwrap();

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
        let mut buffer = create_test_buffer(&mut real_buffers).unwrap();

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
        let mut buffer = create_test_buffer(&mut real_buffers).unwrap();

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
