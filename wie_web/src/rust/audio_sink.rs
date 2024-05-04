use midir::MidiOutputConnection;
use rodio::{buffer::SamplesBuffer, OutputStreamHandle, Sink};

pub struct AudioSink {
    midi_out: MidiOutputConnection,
    sink: Sink,
}

// XXX wasm32 is single-threaded anyway
#[cfg(target_arch = "wasm32")]
unsafe impl Sync for AudioSink {}
#[cfg(target_arch = "wasm32")]
unsafe impl Send for AudioSink {}

impl AudioSink {
    pub fn new(midi_out: MidiOutputConnection, stream_handle: &OutputStreamHandle) -> Self {
        let sink = Sink::try_new(stream_handle).unwrap();
        Self { midi_out, sink }
    }
}

impl wie_backend::AudioSink for AudioSink {
    fn play_wave(&self, channel: u8, sampling_rate: u32, wave_data: &[i16]) {
        let buffer = SamplesBuffer::new(channel as _, sampling_rate as _, wave_data);

        self.sink.append(buffer);
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
