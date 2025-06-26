use nih_plug::nih_export_standalone;
use thesis_vst::Mute;

fn main() {
    let mut real_buffers = vec![vec![5.0; 512]; 2];
    println!("{:?}", real_buffers);

    // nih_export_standalone::<Mute>();
}
