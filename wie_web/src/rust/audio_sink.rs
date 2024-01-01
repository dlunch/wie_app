pub struct AudioSink;

impl wie_backend::AudioSink for AudioSink {
    fn play_wave(&self, _channel: u8, _sampling_rate: u32, _wave_data: &[i16]) {
        // TODO
    }
}
