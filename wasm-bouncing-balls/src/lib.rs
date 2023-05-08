mod html_cast;
mod utils;
use std::{cell::RefCell, rc::Rc};

use html_cast::*;
use js_sys::Math;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement, Window};

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
    let window = web_sys::window().unwrap();

    let width = Math::floor(window.inner_width().unwrap().as_f64().unwrap());
    canvas.set_width(width as u32);
    let height = Math::floor(window.inner_height().unwrap().as_f64().unwrap());
    canvas.set_height(height as u32);

    let mut ball = Ball::new(50.0, 100.0, 4.0, 4.0, "blue", 10.0);

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

    let ctx_rc = Rc::new(RefCell::new(
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap(),
    ));

    let balls_rc = Rc::new(RefCell::new(balls));

    main_loop(ctx_rc, balls_rc, width, height);

    Ok(())
}

fn main_loop(
    ctx_rc: Rc<RefCell<CanvasRenderingContext2d>>,
    balls_rc: Rc<RefCell<Vec<Ball>>>,
    canvas_width: f64,
    canvas_height: f64,
) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        ctx_rc
            .borrow()
            .set_fill_style(&JsValue::from_str("rgba(0,0,0,1)"));

        ctx_rc
            .borrow()
            .fill_rect(0.0, 0.0, canvas_width, canvas_height);

        let mut balls = balls_rc.borrow_mut();

        //balls[0].draw();
        balls[0].update(canvas_width, canvas_height);

        for ball in balls.iter_mut() {
            ball.draw(ctx_rc.clone());
            ball.update(canvas_width, canvas_height);
        }

        web_sys::window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }));

    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
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
    fn draw(&self, ctx_rc: Rc<RefCell<CanvasRenderingContext2d>>) {
        let ctx = ctx_rc.borrow();
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str(&self.color));
        ctx.arc(self.x, self.y, self.size, 0.0, 2.0 * std::f64::consts::PI)
            .unwrap();
        ctx.fill();
    }

    fn update(&mut self, canvas_width: f64, canvas_height: f64) {
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
