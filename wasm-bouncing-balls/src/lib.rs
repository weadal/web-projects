mod html_cast;
mod utils;
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::{self, Rc},
};

use html_cast::*;
use js_sys::Math;
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Event, HtmlButtonElement, HtmlCanvasElement, HtmlParagraphElement,
    Performance,
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let canvas = query_selector_to::<HtmlCanvasElement>("canvas").unwrap();

    let width = 960.0;
    canvas.set_width(width as u32);
    let height = 720.0;
    canvas.set_height(height as u32);

    let mut balls: Vec<Ball> = Vec::new();

    for _ in 0..25 {
        let size = random_f64(10.0, 20.0);
        let ball = Ball::new(
            random_f64(0.0 + size, width as f64 - size),
            random_f64(0.0 + size, height as f64 - size),
            random_f64(-7.0, 7.0),
            random_f64(-7.0, 7.0),
            &random_rgb(),
            size,
        );

        balls.push(ball);
    }

    let balls_rc = Rc::new(RefCell::new(balls));

    let play_button = query_selector_to::<HtmlButtonElement>(".play-pause").unwrap();

    let is_playing_rc = Rc::new(RefCell::new(true));
    let mut is_playing_rc_clone = is_playing_rc.clone();

    let closure: Closure<dyn FnMut()> = Closure::new(move || {
        play_pause(&mut is_playing_rc_clone);
    });
    play_button
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    main_loop(balls_rc.clone(), &is_playing_rc);

    Ok(())
}

fn main_loop(balls_rc: Rc<RefCell<Vec<Ball>>>, is_playing_rc: &Rc<RefCell<bool>>) {
    let closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let closure_clone = closure.clone();

    let mut fps = Fps::new();

    let is_playing_rc_clone = is_playing_rc.clone();
    *closure_clone.borrow_mut() = Some(Closure::new(move || {
        if *is_playing_rc_clone.borrow() {
            update(&mut balls_rc.borrow_mut());

            fps.render();
        }
        request_animation_frame(&closure);
    }));

    request_animation_frame(&closure_clone);
}

fn play_pause(is_playing_rc: &mut Rc<RefCell<bool>>) {
    let state = is_playing_rc.borrow().clone();

    if state {
        *is_playing_rc.borrow_mut() = false;
    } else {
        *is_playing_rc.borrow_mut() = true;
    }
}

fn update(balls: &mut RefMut<Vec<Ball>>) {
    let canvas = query_selector_to::<HtmlCanvasElement>("canvas").unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    ctx.set_fill_style(&JsValue::from_str("rgba(0,0,0,1)"));
    ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    for ball in balls.iter_mut() {
        ball.draw(&ctx);
        ball.moving(canvas.width() as f64, canvas.height() as f64);
    }
}

struct Ball {
    x: f64,
    y: f64,
    vel_x: f64,
    vel_y: f64,
    color: String,
    size: f64,
}
impl Ball {
    fn new(x: f64, y: f64, vel_x: f64, vel_y: f64, color: &str, size: f64) -> Ball {
        Ball {
            x,
            y,
            vel_x,
            vel_y,
            color: color.to_string(),
            size,
        }
    }
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str(&self.color));
        ctx.arc(self.x, self.y, self.size, 0.0, 2.0 * std::f64::consts::PI)
            .unwrap();
        ctx.fill();
    }

    fn moving(&mut self, canvas_width: f64, canvas_height: f64) {
        if self.x + self.size >= canvas_width {
            self.vel_x = -self.vel_x;
        }
        if self.x - self.size <= 0.0 {
            self.vel_x = -self.vel_x;
        }
        if self.y + self.size >= canvas_height {
            self.vel_y = -self.vel_y;
        }
        if self.y - self.size <= 0.0 {
            self.vel_y = -self.vel_y;
        }
        self.x += self.vel_x;
        self.y += self.vel_y;
    }
}

fn random_f64(min: f64, max: f64) -> f64 {
    Math::floor(Math::random() * (max - min + 1.0) as f64) + min
}

fn random_rgb() -> String {
    format!(
        "rgb({},{},{})",
        random_f64(0.0, 255.0) as u32,
        random_f64(0.0, 255.0) as u32,
        random_f64(0.0, 255.0) as u32
    )
}

fn request_animation_frame(closure_rc: &Rc<RefCell<Option<Closure<dyn FnMut()>>>>) -> i32 {
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

struct Fps {
    body: HtmlParagraphElement,
    frames: Vec<f64>,
    performance: web_sys::Performance,
    last_frame_timestamp: f64,
}
impl Fps {
    fn new() -> Fps {
        let body = query_selector_to::<HtmlParagraphElement>(".fps-counter").unwrap();
        let frames: Vec<f64> = vec![];
        let performance = web_sys::window().unwrap().performance().unwrap();
        let last_frame_timestamp = performance.now();

        Fps {
            body,
            frames,
            performance,
            last_frame_timestamp,
        }
    }

    fn render(&mut self) {
        let now = self.performance.now();
        let delta = now - self.last_frame_timestamp;
        self.last_frame_timestamp = now;

        let fps = 1000.0 / delta;

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
