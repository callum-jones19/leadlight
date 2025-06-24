use nih_plug::nih_export_standalone;
use thesis_vst::EmptyNoise;

fn main() {
    nih_export_standalone::<EmptyNoise>();
}
