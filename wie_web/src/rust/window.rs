use alloc::{rc::Rc, vec::Vec};
use core::cell::Cell;

use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use wie_backend::{canvas::Image, Screen};

pub struct WindowImpl {
    canvas: HtmlCanvasElement,
    should_redraw: Rc<Cell<bool>>,
}

impl WindowImpl {
    pub fn new(canvas: HtmlCanvasElement, should_redraw: Rc<Cell<bool>>) -> Self {
        Self { canvas, should_redraw }
    }
}

impl Screen for WindowImpl {
    fn request_redraw(&self) -> anyhow::Result<()> {
        self.should_redraw.set(true);

        Ok(())
    }

    fn paint(&mut self, image: &dyn Image) {
        let context = self
            .canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let image_data = image.colors().into_iter().flat_map(|x| [x.r, x.g, x.b, x.a]).collect::<Vec<_>>();
        let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&image_data), self.width(), self.height()).unwrap();

        context.put_image_data(&data, 0.0, 0.0).unwrap();
    }

    fn width(&self) -> u32 {
        self.canvas.width()
    }

    fn height(&self) -> u32 {
        self.canvas.height()
    }
}
