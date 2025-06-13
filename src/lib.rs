use std::sync::Arc;

use nih_plug::prelude::*;

/// A plugin that takes any input, and then always provides an empty output.
/// This is effectively like a mute
#[derive(Default)]
struct EmptyNoise {
    params: Arc<EmptyNoiseParams>,
}

/// All the paramters for the EmptyNoise plugin are held in a single struct.
/// This allows us to apply the `Params` macro to the struct itself, which then
/// allows nih_plug to
///
/// We also want to return this struct in the params function of the Plugin trait.
#[derive(Params, Default)]
struct EmptyNoiseParams {
    tmp: f32,
}

impl Plugin for EmptyNoise {
    const NAME: &'static str = "EmptyNoise";
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
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        todo!()
    }
}

impl Vst3Plugin for EmptyNoise {
    const VST3_CLASS_ID: [u8; 16] = *b"EmptyNoisePlugin";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Tools];
}

nih_export_vst3!(EmptyNoise);
