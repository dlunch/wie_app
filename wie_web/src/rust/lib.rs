#![no_std]
extern crate alloc;

mod audio_sink;
mod database;
mod window;

use alloc::{
    boxed::Box,
    rc::Rc,
    string::{String, ToString},
};
use core::cell::Cell;

use tracing_subscriber::{filter::LevelFilter, fmt::time::UtcTime, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use tracing_web::MakeConsoleWriter;
use wasm_bindgen::{prelude::*, JsError};
use web_sys::HtmlCanvasElement;

use wie_backend::{extract_zip, App, Archive, Event, Instant, KeyCode, Platform, Screen};
use wie_ktf::KtfArchive;
use wie_lgt::LgtArchive;
use wie_skt::SktArchive;

use self::{audio_sink::AudioSink, database::DatabaseRepository, window::WindowImpl};

struct WieWebPlatform {
    database_repository: DatabaseRepository,
    window: Box<dyn Screen>,
}

impl WieWebPlatform {
    fn new(app_id: &str, window: Box<dyn Screen>) -> Self {
        Self {
            database_repository: DatabaseRepository::new(app_id),
            window,
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
        Box::new(AudioSink)
    }
}

#[wasm_bindgen]
pub struct WieWeb {
    app: Box<dyn App>,
    should_redraw: Rc<Cell<bool>>,
}

#[wasm_bindgen]
impl WieWeb {
    #[wasm_bindgen(constructor)]
    pub fn new(buf: &[u8], canvas: HtmlCanvasElement) -> Result<WieWeb, JsError> {
        (move || {
            let files = extract_zip(buf)?;

            let archive: Box<dyn Archive> = if KtfArchive::is_ktf_archive(&files) {
                Box::new(KtfArchive::from_zip(files)?)
            } else if LgtArchive::is_lgt_archive(&files) {
                Box::new(LgtArchive::from_zip(files)?)
            } else if SktArchive::is_skt_archive(&files) {
                Box::new(SktArchive::from_zip(files)?)
            } else {
                anyhow::bail!("Unknown archive format");
            };

            let should_redraw = Rc::new(Cell::new(true));
            let window = WindowImpl::new(canvas, should_redraw.clone());

            let platform = WieWebPlatform::new(&archive.id(), Box::new(window));

            let mut app = archive.load_app(Box::new(platform))?;

            app.start()?;

            anyhow::Ok(Self { app, should_redraw })
        })()
        .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn update(&mut self) -> Result<(), JsError> {
        if self.should_redraw.get() {
            self.app.on_event(Event::Redraw);
            self.should_redraw.set(false);
        }

        self.app.tick().map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn send_key(&mut self, key: String) -> Result<(), JsError> {
        let key = KeyCode::parse(&key);

        self.app.on_event(Event::Keydown(key));
        self.app.on_event(Event::Keyup(key));

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
