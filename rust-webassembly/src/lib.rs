use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! println {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct IntervalHandle {
    interval_id: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Drop for IntervalHandle {
    fn drop(&mut self) {
        clearInterval(self.interval_id);
    }
}

pub fn draw_text(text: &str, x: u32, y: u32, 
                 context: &web_sys::CanvasRenderingContext2d, 
                 canvas: &web_sys::HtmlCanvasElement) {
    context.clear_rect(0_f64, 0_f64, canvas.width() as f64, canvas.height() as f64);
    context.fill_text(text, x as f64, y as f64).unwrap();
}

#[wasm_bindgen]
pub fn run() -> IntervalHandle {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let wv_width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let wv_height = window.inner_height().unwrap().as_f64().unwrap() as u32;

    canvas.set_width(wv_width);
    canvas.set_height(wv_height);
    context.set_font("48px serif");

    let text = "A";
    let text_metrics = context.measure_text(text).unwrap();

    let text_x = wv_width / 2 - text_metrics.width() as u32;
    let text_y = wv_height / 2;
    context.set_fill_style(&JsValue::from_str("#ffffff"));
    draw_text(text, text_x, text_y, &context, &canvas);

    let cb = Closure::wrap(Box::new(move || {
        println!("text_x: {}", text_x);
        draw_text("Marcell", text_x, text_y, &context, &canvas);
    }) as Box<dyn FnMut()>);

    let interval_id = setInterval(&cb, 1_000);

    IntervalHandle {
        interval_id,
        _closure: cb,
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    run(); 
}
