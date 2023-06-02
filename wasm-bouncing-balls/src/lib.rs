mod html_cast;
mod utils;
use std::{
    cell::{Ref, RefCell, RefMut},
    clone,
    rc::{self, Rc},
};

use html_cast::*;
use js_sys::Math;
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Event, HtmlButtonElement, HtmlCanvasElement, HtmlInputElement,
    HtmlParagraphElement, Performance,
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

    fn Number(s: &str) -> i32;

}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let canvas = query_selector_to::<HtmlCanvasElement>("canvas").unwrap();

    let width = 960.0;
    canvas.set_width(width as u32);
    let height = 720.0;
    canvas.set_height(height as u32);

    let balls: Vec<Ball> = Vec::new();
    let balls_rc = Rc::new(RefCell::new(balls));

    let balls_size = Number(
        &query_selector_to::<HtmlInputElement>(".ball-field")
            .unwrap()
            .value(),
    );

    balls_init(&balls_rc, balls_size);

    let is_playing_rc = Rc::new(RefCell::new(true));

    //一時停止ボタン
    {
        let play_button = query_selector_to::<HtmlButtonElement>(".play-pause").unwrap();
        let is_playing_rc_clone = is_playing_rc.clone();
        let closure: Closure<dyn FnMut()> = Closure::new(move || {
            play_pause(&is_playing_rc_clone);
        });
        play_button
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    //ボール数変更UI
    {
        let balls_size_submit = query_selector_to::<HtmlInputElement>(".ball-submit").unwrap();

        let balls_rc_clone = balls_rc.clone();
        let closure: Closure<dyn FnMut()> = Closure::new(move || {
            let balls_size = Number(
                &query_selector_to::<HtmlInputElement>(".ball-field")
                    .unwrap()
                    .value(),
            );
            balls_init(&balls_rc_clone, balls_size);
        });
        balls_size_submit
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

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

fn play_pause(is_playing_rc: &Rc<RefCell<bool>>) {
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

    let mut ballrefs: Vec<&Ball> = vec![];
    for ball in balls.iter() {
        ballrefs.push(ball.clone());
    }

    ctx.set_stroke_style(&JsValue::from_str("rgba(0.0,255.0,255.0,0.2)"));
    ctx.set_line_width(2.0);

    let loot_node = create_tree(ballrefs, &ctx, false);
    let balls_with_possible_contact = get_contact_with(
        loot_node.left_child.as_ref().unwrap(),
        loot_node.right_child.as_ref().unwrap(),
        Rc::new(RefCell::new(vec![])),
    );

    log(&format!(
        "collision_count:{}",
        balls_with_possible_contact.len()
    ));

    for ball in balls.iter_mut() {
        ball.draw(&ctx);
        ball.moving(canvas.width() as f64, canvas.height() as f64);
    }
}

fn get_contact_with<'a>(
    node: &Box<Node<'a>>,
    other: &Box<Node<'a>>,
    balls_with_possible_contact: Rc<RefCell<Vec<(&'a Ball, &'a Ball)>>>,
) -> Vec<(&'a Ball, &'a Ball)> {
    if !node.aabb.is_intersects(&other.aabb) {
        if node.balls.len() > 1 {
            get_contact_with(
                node.left_child.as_ref().unwrap(),
                node.right_child.as_ref().unwrap(),
                balls_with_possible_contact.clone(),
            );
        }
        if other.balls.len() > 1 {
            get_contact_with(
                other.left_child.as_ref().unwrap(),
                other.right_child.as_ref().unwrap(),
                balls_with_possible_contact.clone(),
            );
        }

        return balls_with_possible_contact.borrow().clone();
    }

    //接触
    if node.balls.len() == 1 && other.balls.len() == 1 {
        balls_with_possible_contact
            .borrow_mut()
            .push((node.balls[0], other.balls[0]));
        // log(&format!(
        //     "node:{:?},other:{:?},contact_list_ren:{}",
        //     node.balls[0].name,
        //     other.balls[0].name,
        //     balls_with_possible_contact.borrow().len()
        // ));

        {
            let canvas = query_selector_to::<HtmlCanvasElement>("canvas").unwrap();
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            node.balls[0].draw(&ctx);
            other.balls[0].draw(&ctx);

            ctx.set_stroke_style(&JsValue::from_str("rgba(255.0,0.0,0.0,1)"));
            ctx.set_line_width(4.0);

            draw_aabb(&ctx, &Aabb::from_aabbs(vec![node.aabb, other.aabb]));
        }

        return balls_with_possible_contact.borrow().clone();
    }

    if node.balls.len() > 1 && (other.balls.len() == 1 || (node.aabb.size() >= other.aabb.size())) {
        let balls_with_possible_contact_clone = balls_with_possible_contact.clone();
        let left_child_result = get_contact_with(
            node.left_child.as_ref().unwrap(),
            other,
            balls_with_possible_contact_clone,
        );

        // balls_with_possible_contact
        //     .borrow_mut()
        //     .extend(left_child_result);

        let balls_with_possible_contact_clone = balls_with_possible_contact.clone();

        get_contact_with(
            node.right_child.as_ref().unwrap(),
            other,
            balls_with_possible_contact_clone,
        );
    }

    if other.balls.len() > 1 && (node.balls.len() == 1 || (node.aabb.size() < other.aabb.size())) {
        let balls_with_possible_contact_clone = balls_with_possible_contact.clone();

        let other_left_child_result = get_contact_with(
            node,
            other.left_child.as_ref().unwrap(),
            balls_with_possible_contact_clone,
        );

        // balls_with_possible_contact
        //     .borrow_mut()
        //     .extend(other_left_child_result);

        let balls_with_possible_contact_clone = balls_with_possible_contact.clone();

        get_contact_with(
            node,
            other.right_child.as_ref().unwrap(),
            balls_with_possible_contact_clone,
        );
    }

    balls_with_possible_contact.borrow().clone()
}

fn create_tree<'a>(
    balls: Vec<&'a Ball>,
    ctx: &'a CanvasRenderingContext2d,
    y_axis_division: bool,
) -> Node<'a> {
    let aabb = Aabb::from_ballrefs(&balls);
    draw_aabb(&ctx, &aabb);
    //オブジェクト数が1つのAABBはそれ以上分類できないので決め打ちで葉要素として最終処理
    if balls.len() == 1 {
        return Node {
            left_child: None,
            right_child: None,
            balls,
            aabb,
        };
    }

    //中点をAABBから取る関係上自身の軸サイズと自身が所属するAABBの軸サイズが一致すると右にも左にも分類できないオブジェクトが発生する
    //例えば小さいオブジェクトが大きいオブジェクトの影に隠れる(同一y軸にはいる)形になると、大きいオブジェクトのmax_x,min_xがAABB全体のmax_x,min_xになってしまう
    //そうしたときに間違って両方を同じサイドのchildに入れてしまうと無限ループが発生する(再帰呼び出しした先でも同じサイドのchildに入れられる)
    let mut left_balls: Vec<&Ball> = vec![];
    let mut right_balls: Vec<&Ball> = vec![];
    let mut center_balls: Vec<&Ball> = vec![];

    //フラグで分割する軸を変更
    if y_axis_division {
        let parent_center_y = (aabb.y_max + aabb.y_min) / 2.0;

        for ball in balls.iter() {
            if ball.y < parent_center_y {
                left_balls.push(ball.clone());
            } else if ball.y > parent_center_y {
                right_balls.push(ball.clone());
            } else {
                center_balls.push(ball.clone());
            }
        }
    } else {
        let parent_center_x = (aabb.x_max + aabb.x_min) / 2.0;
        for ball in balls.iter() {
            if ball.x < parent_center_x {
                left_balls.push(ball.clone());
            } else if ball.x > parent_center_x {
                right_balls.push(ball.clone());
            } else {
                center_balls.push(ball.clone());
            }
        }
    }

    //分類できないオブジェクトは、左右のchildを見て少ない方に入れることでオブジェクト数が2つだけのAABBになった場合のループを回避する
    for ball in center_balls {
        if left_balls.len() <= right_balls.len() {
            left_balls.push(ball);
        } else {
            right_balls.push(ball);
        }
    }

    let mut left_child = None;
    let mut right_child = None;

    if left_balls.len() > 0 {
        //次回の分割方向は今回とは別の軸を使う(!y_axis_division)
        left_child = Some(Box::new(create_tree(left_balls, &ctx, !y_axis_division)));
    }
    if right_balls.len() > 0 {
        right_child = Some(Box::new(create_tree(right_balls, &ctx, !y_axis_division)));
    }

    let node = Node {
        left_child,
        right_child,
        balls,
        aabb,
    };

    node
}

struct Node<'a> {
    left_child: Option<Box<Node<'a>>>,
    right_child: Option<Box<Node<'a>>>,
    balls: Vec<&'a Ball>,
    aabb: Aabb,
}

fn draw_aabb(ctx: &CanvasRenderingContext2d, aabb: &Aabb) {
    ctx.begin_path();

    ctx.move_to(aabb.x_min, aabb.y_min);
    ctx.line_to(aabb.x_max, aabb.y_min);
    ctx.line_to(aabb.x_max, aabb.y_max);
    ctx.line_to(aabb.x_min, aabb.y_max);
    ctx.line_to(aabb.x_min, aabb.y_min);

    ctx.stroke();
}

fn balls_init(balls_rc: &Rc<RefCell<Vec<Ball>>>, balls_size: i32) {
    let canvas = query_selector_to::<HtmlCanvasElement>("canvas").unwrap();
    balls_rc.borrow_mut().clear();

    for i in 0..balls_size {
        let size = random_f64(5.0, 40.0);
        let ball = Ball::new(
            random_f64(0.0 + size, canvas.width() as f64 - size),
            random_f64(0.0 + size, canvas.height() as f64 - size),
            random_f64(-2.0, 2.0),
            random_f64(-2.0, 2.0),
            &random_rgb(),
            size,
            i,
        );

        balls_rc.borrow_mut().push(ball);
    }
}

#[derive(Clone, Copy)]
struct Aabb {
    x_max: f64,
    x_min: f64,
    y_max: f64,
    y_min: f64,
}
impl Aabb {
    fn from_circle(x: f64, y: f64, size: f64) -> Aabb {
        Aabb {
            x_max: x + size,
            x_min: x - size,
            y_max: y + size,
            y_min: y - size,
        }
    }

    fn from_aabbs(aabbs: Vec<Aabb>) -> Aabb {
        let mut x_max = -f64::INFINITY;
        let mut x_min = f64::INFINITY;
        let mut y_max = -f64::INFINITY;
        let mut y_min = f64::INFINITY;

        for aabb in aabbs {
            if aabb.x_max > x_max {
                x_max = aabb.x_max;
            }
            if aabb.x_min < x_min {
                x_min = aabb.x_min;
            }
            if aabb.y_max > y_max {
                y_max = aabb.y_max;
            }
            if aabb.y_min < y_min {
                y_min = aabb.y_min
            }
        }

        Aabb {
            x_max,
            x_min,
            y_max,
            y_min,
        }
    }

    fn from_balls(balls: &Vec<Ball>) -> Aabb {
        let mut x_max = -f64::INFINITY;
        let mut x_min = f64::INFINITY;
        let mut y_max = -f64::INFINITY;
        let mut y_min = f64::INFINITY;

        for ball in balls {
            let aabb = ball.aabb();
            if aabb.x_max > x_max {
                x_max = aabb.x_max;
            }
            if aabb.x_min < x_min {
                x_min = aabb.x_min;
            }
            if aabb.y_max > y_max {
                y_max = aabb.y_max;
            }
            if aabb.y_min < y_min {
                y_min = aabb.y_min
            }
        }

        Aabb {
            x_max,
            x_min,
            y_max,
            y_min,
        }
    }
    fn from_ballrefs(balls: &Vec<&Ball>) -> Aabb {
        let mut x_max = -f64::INFINITY;
        let mut x_min = f64::INFINITY;
        let mut y_max = -f64::INFINITY;
        let mut y_min = f64::INFINITY;

        for ball in balls {
            let aabb = ball.aabb();
            if aabb.x_max > x_max {
                x_max = aabb.x_max;
            }
            if aabb.x_min < x_min {
                x_min = aabb.x_min;
            }
            if aabb.y_max > y_max {
                y_max = aabb.y_max;
            }
            if aabb.y_min < y_min {
                y_min = aabb.y_min
            }
        }

        Aabb {
            x_max,
            x_min,
            y_max,
            y_min,
        }
    }

    fn is_intersects(&self, other: &Aabb) -> bool {
        if self.x_min > other.x_max {
            return false;
        }
        if self.x_max < other.x_min {
            return false;
        }
        if self.y_min > other.y_max {
            return false;
        }
        if self.y_max < other.y_min {
            return false;
        }

        return true;
    }

    fn size(&self) -> f64 {
        let x_size = self.x_max - self.x_min;
        let y_size = self.y_max - self.y_min;

        x_size + y_size
    }
}
#[derive(Debug)]
struct Ball {
    x: f64,
    y: f64,
    vel_x: f64,
    vel_y: f64,
    color: String,
    size: f64,
    name: i32,
}
impl Ball {
    fn new(x: f64, y: f64, vel_x: f64, vel_y: f64, color: &str, size: f64, name: i32) -> Ball {
        Ball {
            x,
            y,
            vel_x,
            vel_y,
            color: color.to_string(),
            size,
            name,
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

    fn aabb(&self) -> Aabb {
        Aabb::from_circle(self.x, self.y, self.size)
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
