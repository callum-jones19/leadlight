use std::sync::Arc;

use nih_plug::prelude::*;
use process::process_algorithm;

pub mod process;

/// A plugin that takes any input, and then always provides an empty output.
/// This is effectively like a mute
#[derive(Default)]
pub struct Lowpass {
    params: Arc<LowpassParams>,
}

/// All the paramters for the Mute plugin are held in a single struct.
/// This allows us to apply the `Params` macro to the struct itself, which then
/// allows nih_plug to
///
/// We also want to return this struct in the params function of the Plugin trait.
#[derive(Params, Default)]
struct LowpassParams {}

impl Plugin for Lowpass {
    const NAME: &'static str = "Lowpass";
    const VENDOR: &'static str = "Callum Jones";
    const URL: &'static str = "https://github.com/callum-jones19/thesis_vst/";
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

impl Vst3Plugin for Lowpass {
    const VST3_CLASS_ID: [u8; 16] = *b"fooofooofooofoo4";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Tools];
}

impl ClapPlugin for Lowpass {
    const CLAP_ID: &'static str = "CallumJonesLowpassPlugin";

    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("A basic lowpass filter, designed based on the algorithm notes given in The Scientist and Engineer's Guide to Digital Signal Processing, 2nd edn, by Steven W. Smith.");

    const CLAP_MANUAL_URL: Option<&'static str> = None;

    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Gate];
}

nih_export_vst3!(Lowpass);
nih_export_clap!(Lowpass);
