use rodio::{buffer::SamplesBuffer, OutputStreamHandle, Sink};
use wasm_bindgen::JsValue;

use super::WieWebBridge;

pub struct AudioSink {
    bridge: WieWebBridge,
    sink: Sink,
}

// XXX we're on single thread
unsafe impl Sync for AudioSink {}
unsafe impl Send for AudioSink {}

impl AudioSink {
    pub fn new(stream_handle: &OutputStreamHandle, bridge: WieWebBridge) -> Self {
        let sink = Sink::try_new(stream_handle).unwrap();
        Self { bridge, sink }
    }
}

impl wie_backend::AudioSink for AudioSink {
    fn play_wave(&self, channel: u8, sampling_rate: u32, wave_data: &[i16]) {
        let buffer = SamplesBuffer::new(channel as _, sampling_rate as _, wave_data);

        self.sink.append(buffer);
    }

    fn midi_note_on(&self, channel_id: u8, note: u8, velocity: u8) {
        self.bridge
            .midi_note_on
            .call3(&JsValue::null(), &channel_id.into(), &note.into(), &velocity.into())
            .unwrap();
    }

    fn midi_note_off(&self, channel_id: u8, note: u8, velocity: u8) {
        self.bridge
            .midi_note_off
            .call3(&JsValue::null(), &channel_id.into(), &note.into(), &velocity.into())
            .unwrap();
    }

    fn midi_control_change(&self, channel_id: u8, control: u8, value: u8) {
        self.bridge
            .midi_control_change
            .call3(&JsValue::null(), &channel_id.into(), &control.into(), &value.into())
            .unwrap();
    }

    fn midi_program_change(&self, channel_id: u8, program: u8) {
        self.bridge
            .midi_program_change
            .call2(&JsValue::null(), &channel_id.into(), &program.into())
            .unwrap();
    }
}
