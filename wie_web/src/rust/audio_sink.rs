use midir::MidiOutputConnection;
use rodio::{buffer::SamplesBuffer, OutputStreamHandle, Sink};
use spin::Mutex;

pub struct AudioSink {
    midi_out: Mutex<MidiOutputConnection>,
    sink: Sink,
}

// XXX we're on single thread
unsafe impl Sync for AudioSink {}
unsafe impl Send for AudioSink {}

impl AudioSink {
    pub fn new(midi_out: MidiOutputConnection, stream_handle: &OutputStreamHandle) -> Self {
        let sink = Sink::try_new(stream_handle).unwrap();
        Self {
            midi_out: Mutex::new(midi_out),
            sink,
        }
    }
}

impl wie_backend::AudioSink for AudioSink {
    fn play_wave(&self, channel: u8, sampling_rate: u32, wave_data: &[i16]) {
        let buffer = SamplesBuffer::new(channel as _, sampling_rate as _, wave_data);

        self.sink.append(buffer);
    }

    fn midi_note_on(&self, channel_id: u8, note: u8, velocity: u8) {
        self.midi_out.lock().send(&[0x90 | channel_id, note, velocity]).unwrap();
    }

    fn midi_note_off(&self, channel_id: u8, note: u8, velocity: u8) {
        self.midi_out.lock().send(&[0x80 | channel_id, note, velocity]).unwrap();
    }

    fn midi_control_change(&self, channel_id: u8, control: u8, value: u8) {
        self.midi_out.lock().send(&[0xB0 | channel_id, control, value]).unwrap()
    }

    fn midi_program_change(&self, channel_id: u8, program: u8) {
        self.midi_out.lock().send(&[0xC0 | channel_id, program]).unwrap()
    }
}
