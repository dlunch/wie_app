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

    fn midi_note_on(&self, _channel_id: u8, _note: u8, _velocity: u8) {
        self.bridge.midi_event.call1(&JsValue::null(), &0.into()).unwrap();
    }

    fn midi_note_off(&self, _channel_id: u8, _note: u8, _velocity: u8) {
        self.bridge.midi_event.call1(&JsValue::null(), &1.into()).unwrap();
    }

    fn midi_control_change(&self, _channel_id: u8, _control: u8, _value: u8) {
        self.bridge.midi_event.call1(&JsValue::null(), &2.into()).unwrap();
    }

    fn midi_program_change(&self, _channel_id: u8, _program: u8) {
        self.bridge.midi_event.call1(&JsValue::null(), &3.into()).unwrap();
    }
}
