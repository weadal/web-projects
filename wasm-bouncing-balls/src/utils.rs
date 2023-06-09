use std::{cell::RefCell, rc::Rc};

use crate::html_cast::*;
use js_sys::Math;

use wasm_bindgen::prelude::*;
use web_sys::{console, HtmlParagraphElement};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub struct Input {
    pub click_x: Option<f64>,
    pub click_y: Option<f64>,
    pub is_playing: bool,
}
impl Input {
    pub fn new() -> Self {
        Input {
            click_x: None,
            click_y: None,
            is_playing: false,
        }
    }

    pub fn toggle_is_playing(&mut self) {
        if self.is_playing {
            self.is_playing = false;
        } else {
            self.is_playing = true;
        }
    }
}

pub fn request_animation_frame(closure_rc: &Rc<RefCell<Option<Closure<dyn FnMut()>>>>) -> i32 {
    web_sys::window()
        .unwrap()
        .request_animation_frame(
            closure_rc
                .borrow()
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        )
        .unwrap()
}

pub struct Fps {
    body: HtmlParagraphElement,
    frames: Vec<f64>,
    performance: web_sys::Performance,
    last_frame_timestamp: f64,
    pub delta_time: f64,
}
impl Fps {
    pub fn new() -> Fps {
        let body = query_selector_to::<HtmlParagraphElement>(".fps-counter").unwrap();
        let frames: Vec<f64> = vec![];
        let performance = web_sys::window().unwrap().performance().unwrap();
        let last_frame_timestamp = performance.now();
        let delta_time = 0.0;

        Fps {
            body,
            frames,
            performance,
            last_frame_timestamp,
            delta_time,
        }
    }

    pub fn render(&mut self) {
        let now = self.performance.now();
        self.delta_time = now - self.last_frame_timestamp;
        self.last_frame_timestamp = now;

        let fps = 1000.0 / self.delta_time;

        self.frames.push(fps);
        if self.frames.len() > 100 {
            self.frames.remove(0);
        }

        let mut min = f64::INFINITY;
        let mut max = -f64::INFINITY;
        let mut sum = 0.0;

        for i in 0..self.frames.len() {
            sum += self.frames[i];
            min = Math::min(min, self.frames[i]);
            max = Math::max(max, self.frames[i]);
        }

        let mean = sum / self.frames.len() as f64;
        self.body.set_inner_html(
            &format!(
                "fps counter<br>latest = {}<br>avg of last 100 = {}<br>min of last 100 = {}<br>max of last 100 = {}",
                Math::round(fps),
                Math::round(mean),
                Math::round(min),
                Math::round(max)
            )
            .trim(),
        );
    }
}

//生成時からDropまでにかかった時間を測定するタイマー web_sys::consoleの出力を使ってるのでブラウザのコンソールに表示される
pub struct Timer<'a> {
    name: &'a str,
}
impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}
impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    Math::floor(Math::random() * (max - min + 1.0) as f64) + min
}
pub fn random_rgb() -> String {
    format!(
        "rgb({},{},{})",
        random_f64(0.0, 255.0) as u32,
        random_f64(0.0, 255.0) as u32,
        random_f64(0.0, 255.0) as u32
    )
}
