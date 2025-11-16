use std::sync::Arc;

use nih_plug::prelude::*;
use process::process_lowpass;

pub mod process;

pub struct Lowpass {
    params: Arc<LowpassParams>,
    channel_data: Vec<f64>,
}

#[derive(Params)]
struct LowpassParams {
    #[id = "lowpass_amount"]
    lowpass_amount: FloatParam,
}

impl LowpassParams {
    pub fn new() -> Self {
        let lowpass_range = FloatRange::Linear { min: 0.0, max: 1.0 };
        LowpassParams {
            lowpass_amount: FloatParam::new("Lowpass Amount", 1.0, lowpass_range),
        }
    }
}

impl Default for Lowpass {
    fn default() -> Self {
        let default_lowpass_params = LowpassParams::new();
        Lowpass {
            params: Arc::new(default_lowpass_params),
            channel_data: Vec::new(),
        }
    }
}

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
        let lowpass_amount = self.params.lowpass_amount.value();
        process_lowpass(buffer, lowpass_amount, &mut self.channel_data);
        ProcessStatus::Normal
    }

    fn initialize(
        &mut self,
        audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        for t in audio_io_layout.main_input_channels {
            self.channel_data.push(0.0);
        }

        true
    }
}

impl Vst3Plugin for Lowpass {
    const VST3_CLASS_ID: [u8; 16] = *b"fooofooofooofoo4";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Tools];
}

impl ClapPlugin for Lowpass {
    const CLAP_ID: &'static str = "CallumJonesLowpassPlugin";

    const CLAP_DESCRIPTION: Option<&'static str> = Some(
        "A basic lowpass filter, designed based on the algorithm notes given in The Scientist and Engineer's Guide to Digital Signal Processing, 2nd edn, by Steven W. Smith.",
    );

    const CLAP_MANUAL_URL: Option<&'static str> = None;

    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Gate];
}

nih_export_vst3!(Lowpass);
nih_export_clap!(Lowpass);
