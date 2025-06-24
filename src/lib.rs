use std::sync::Arc;

use nih_plug::prelude::*;

/// A plugin that takes any input, and then always provides an empty output.
/// This is effectively like a mute
#[derive(Default)]
pub struct Mute {
    params: Arc<MuteParams>,
}

impl Mute {
    fn process_algorithm(&self, buffer: &mut Buffer) {
        for samples in buffer.iter_samples() {
            for sample in samples {
                *sample = 0.0;
            }
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
        self.process_algorithm(buffer);
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
    use nih_plug::{
        buffer::Buffer,
    };

    use crate::Mute;

    #[test]
    fn basic_test() {
        let sample_init_val = 5.0;

        let empty_noise_plug = Mute::default();

        // This section of unsafe code is directly pulled from nih-plug's internal
        // buffer tests. For now, I will assume this is therefore a verified
        // implementation.
        let mut real_buffers = vec![vec![5.0; 512]; 2];
        let mut buffer = Buffer::default();
        unsafe {
            buffer.set_slices(512, |output_slices| {
                let (first_channel, other_channels) = real_buffers.split_at_mut(1);
                *output_slices = vec![&mut first_channel[0], &mut other_channels[0]];
            })
        };

        // Verify that the buffer is what we expect it to be
        for samples in buffer.iter_samples() {
            for sample in samples {
                assert_eq!(*sample, sample_init_val);
            }
        }

        // Now pass the buffer through the processing algorithm, which should
        // assign every sample value to be 0.
        empty_noise_plug.process_algorithm(&mut buffer);

        // Verify the updated values
        for samples in buffer.iter_samples() {
            for sample in samples {
                assert_eq!(*sample, 0.0);
            }
        }
    }
}
