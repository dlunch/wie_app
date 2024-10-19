#![no_std]
extern crate alloc;

mod audio_sink;
mod database;
mod window;

use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
};
use core::{
    str,
    sync::atomic::{AtomicBool, Ordering},
};

use rodio::{OutputStream, OutputStreamHandle};
use tracing_subscriber::{filter::LevelFilter, fmt::time::UtcTime, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use tracing_web::MakeConsoleWriter;
use wasm_bindgen::{prelude::*, JsError};
use web_sys::HtmlCanvasElement;

use wie_backend::{extract_zip, Emulator, Event, Instant, KeyCode, Platform, Screen};
use wie_j2me::J2MEEmulator;
use wie_ktf::KtfEmulator;
use wie_lgt::LgtEmulator;
use wie_skt::SktEmulator;

use self::{audio_sink::AudioSink, database::DatabaseRepository, window::WindowImpl};

#[wasm_bindgen]
#[derive(Clone)]
pub struct WieWebBridge {
    midi_note_on: js_sys::Function,
    midi_note_off: js_sys::Function,
    midi_control_change: js_sys::Function,
    midi_program_change: js_sys::Function,
}

#[wasm_bindgen]
impl WieWebBridge {
    #[wasm_bindgen(constructor)]
    pub fn new(
        midi_note_on: js_sys::Function,
        midi_note_off: js_sys::Function,
        midi_control_change: js_sys::Function,
        midi_program_change: js_sys::Function,
    ) -> Self {
        Self {
            midi_note_on,
            midi_note_off,
            midi_control_change,
            midi_program_change,
        }
    }
}

struct WieWebPlatform {
    bridge: WieWebBridge,
    database_repository: DatabaseRepository,
    window: Box<dyn Screen>,
    _output_stream: OutputStream,
    output_stream_handle: OutputStreamHandle,
}

// XXX we're on single thread
unsafe impl Sync for WieWebPlatform {}
unsafe impl Send for WieWebPlatform {}

impl WieWebPlatform {
    fn new(window: Box<dyn Screen>, bridge: WieWebBridge) -> Self {
        let (output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        Self {
            bridge,
            database_repository: DatabaseRepository::new(),
            window,
            _output_stream: output_stream,
            output_stream_handle,
        }
    }
}

impl Platform for WieWebPlatform {
    fn screen(&mut self) -> &mut dyn Screen {
        self.window.as_mut()
    }

    fn now(&self) -> Instant {
        let date = js_sys::Date::new_0();
        let millis = date.value_of();

        Instant::from_epoch_millis(millis as _)
    }

    fn database_repository(&self) -> &dyn wie_backend::DatabaseRepository {
        &self.database_repository
    }

    fn audio_sink(&self) -> Box<dyn wie_backend::AudioSink> {
        Box::new(AudioSink::new(&self.output_stream_handle, self.bridge.clone()))
    }

    fn write_stdout(&self, data: &[u8]) {
        let string = str::from_utf8(data).unwrap();
        tracing::info!("{}", string);
    }

    fn write_stderr(&self, data: &[u8]) {
        let string = str::from_utf8(data).unwrap();
        tracing::info!("{}", string);
    }
}

#[wasm_bindgen]
pub struct WieWeb {
    emulator: Box<dyn Emulator>,
    should_redraw: Arc<AtomicBool>,
}

#[wasm_bindgen]
impl WieWeb {
    #[wasm_bindgen(constructor)]
    pub fn new(filename: &str, buf: &[u8], canvas: HtmlCanvasElement, bridge: WieWebBridge) -> Result<WieWeb, JsError> {
        (move || {
            let should_redraw = Arc::new(AtomicBool::new(true));
            let window = WindowImpl::new(canvas, should_redraw.clone());
            let platform = Box::new(WieWebPlatform::new(Box::new(window), bridge));

            let emulator: Box<dyn Emulator> = if filename.ends_with("zip") {
                let files = extract_zip(buf).unwrap();

                if KtfEmulator::loadable_archive(&files) {
                    Box::new(KtfEmulator::from_archive(platform, files)?)
                } else if LgtEmulator::loadable_archive(&files) {
                    Box::new(LgtEmulator::from_archive(platform, files)?)
                } else if SktEmulator::loadable_archive(&files) {
                    Box::new(SktEmulator::from_archive(platform, files)?)
                } else {
                    anyhow::bail!("Unknown archive format");
                }
            } else if filename.ends_with("jar") {
                let filename_without_ext = filename.trim_end_matches(".jar");

                if KtfEmulator::loadable_jar(buf) {
                    Box::new(KtfEmulator::from_jar(platform, filename, buf.to_vec(), filename_without_ext, None)?)
                } else if LgtEmulator::loadable_jar(buf) {
                    Box::new(LgtEmulator::from_jar(platform, filename, buf.to_vec(), filename_without_ext, None)?)
                } else if SktEmulator::loadable_jar(buf) {
                    Box::new(SktEmulator::from_jar(platform, filename, buf.to_vec(), filename_without_ext, None)?)
                } else {
                    Box::new(J2MEEmulator::from_jar(platform, filename_without_ext, buf.to_vec())?)
                }
            } else {
                anyhow::bail!("Unknown file format");
            };

            anyhow::Ok(Self { emulator, should_redraw })
        })()
        .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn update(&mut self) -> Result<(), JsError> {
        if self.should_redraw.load(Ordering::SeqCst) {
            self.emulator.handle_event(Event::Redraw);
            self.should_redraw.store(false, Ordering::SeqCst)
        }

        self.emulator.tick().map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn key_down(&mut self, key: String) -> Result<(), JsError> {
        let key = KeyCode::parse(&key);

        self.emulator.handle_event(Event::Keydown(key));

        Ok(())
    }

    pub fn key_up(&mut self, key: String) -> Result<(), JsError> {
        let key = KeyCode::parse(&key);

        self.emulator.handle_event(Event::Keyup(key));

        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(MakeConsoleWriter)
        .with_filter(LevelFilter::ERROR);

    tracing_subscriber::registry().with(fmt_layer).init();
}
