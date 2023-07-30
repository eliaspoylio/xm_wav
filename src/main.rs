use hound;
use std::i16;

use libxm::XMContext;

fn main() {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 96000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("test.wav", spec).unwrap();

    let bytes = include_bytes!("../test.xm");
    let data = bytes.to_vec();

    let mut xm = XMContext::new(&data, 96000).unwrap();
    xm.set_max_loop_count(1);

    let mut buffer = [0.0; 4096];
    while xm.loop_count() == 0 {
        xm.generate_samples(&mut buffer);
        for b in buffer {
            let amplitude = i16::MAX as f32;
            writer.write_sample((b * amplitude) as i16).unwrap();
        }
    }
    writer.finalize().unwrap();
}
