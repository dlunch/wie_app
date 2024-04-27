pub struct AudioSink;

impl wie_backend::AudioSink for AudioSink {
    fn play_wave(&self, _channel: u8, _sampling_rate: u32, _wave_data: &[i16]) {
        // TODO
    }

    fn midi_note_on(&self, _channel_id: u8, _note: u8, _velocity: u8) {
        // TODO
    }

    fn midi_note_off(&self, _channel_id: u8, _note: u8, _velocity: u8) {
        // TODO
    }

    fn midi_program_change(&self, _channel_id: u8, _program: u8) {
        // TODO
    }

    fn midi_control_change(&self, _channel_id: u8, _control: u8, _value: u8) {
        // TODOs
    }
}
