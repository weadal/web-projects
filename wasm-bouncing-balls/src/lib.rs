pub mod draw_old;
pub mod game_loop;
pub mod structs;
pub mod systems;
pub mod user_consts;

mod html_cast;
mod utils;
use std::{
    cell::{Ref, RefCell, RefMut},
    clone,
    rc::{self, Rc},
};

use html_cast::*;
use js_sys::Math;
use structs::ecs::World;
use systems::*;
use utils::*;
use wasm_bindgen::prelude::*;

use web_sys::{
    console, CanvasRenderingContext2d, DomRect, HtmlButtonElement, HtmlCanvasElement,
    HtmlInputElement, HtmlParagraphElement, MouseEvent, Performance,
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
    let input = Rc::new(RefCell::new(Input::new()));

    //canvas設定

    let canvas = query_selector_to::<HtmlCanvasElement>("canvas").unwrap();

    let width = 960;
    canvas.set_width(width as u32);
    let height = 720;
    canvas.set_height(height as u32);

    //canvasが存在する矩形領域を取得する
    let bounding_rect = canvas.get_bounding_client_rect();

    //縮小や拡大などされているかもしれないので相対スケールを確保しておく
    let scale_x = canvas.width() as f64 / bounding_rect.width();
    let scale_y = canvas.height() as f64 / bounding_rect.height();

    let input_clone = input.clone();

    //canvas上をクリックすることでキャンバス上の座標を取得するイベントハンドラ
    let closure = Closure::wrap(Box::new(move |e: MouseEvent| {
        //クリックされた絶対位置から矩形領域の位置を引いてローカル座標を取得する また、相対スケールも掛けておく
        let local_x = (e.client_x() as f64 - bounding_rect.left()) * scale_x;
        let local_y = (e.client_y() as f64 - bounding_rect.top()) * scale_y;

        input_clone.borrow_mut().click_x = Some(local_x);
        input_clone.borrow_mut().click_y = Some(local_y);

        log(&format!("click! local_x:{},local_y:{}", local_x, local_y));
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    //canvas設定ここまで

    let balls: Vec<Ball> = Vec::new();
    let balls_rc = Rc::new(RefCell::new(balls));

    let balls_size = Number(
        &query_selector_to::<HtmlInputElement>(".ball-field")
            .unwrap()
            .value(),
    );

    balls_init(&balls_rc, balls_size);

    let input_clone = input.clone();
    //一時停止ボタン
    {
        let play_button = query_selector_to::<HtmlButtonElement>(".play-pause").unwrap();
        let closure: Closure<dyn FnMut()> =
            Closure::new(move || input_clone.borrow_mut().toggle_is_playing());
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

    let input_clone = input.clone();
    //ウィンドウの最小化時など画面が完全に隠れたときににポーズするように
    //これやっとかないとポーズしてないままロジックが止まってdelta_timeの異常加算から挙動がおかしくなる
    {
        let window = web_sys::window().unwrap();
        let doc = window.document().unwrap();

        let closure: Closure<dyn FnMut()> = Closure::new(move || {
            input_clone.borrow_mut().is_playing = false;
            log("pause");
        });
        doc.add_event_listener_with_callback("visibilitychange", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    main_loop(balls_rc.clone(), input.clone());

    Ok(())
}

fn main_loop(balls_rc: Rc<RefCell<Vec<Ball>>>, input: Rc<RefCell<Input>>) {
    let closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let closure_clone = closure.clone();

    let mut fps = Fps::new();

    input.borrow_mut().is_playing = true;

    let mut world = World::new();

    let canvas = query_selector_to::<HtmlCanvasElement>("canvas").unwrap();
    world.consts.canvas_x = canvas.width();
    world.consts.canvas_y = canvas.height();

    for _ in 0..25 {
        sys_main::create_ball(&mut world);
    }

    let input_rc_clone = input.clone();
    *closure_clone.borrow_mut() = Some(Closure::new(move || {
        if input_rc_clone.borrow().is_playing {
            update(&mut balls_rc.borrow_mut(), &mut world);
            game_loop::tick(&mut world);
        }
        fps.render();
        world.consts.delta_time = fps.delta_time;
        request_animation_frame(&closure);
    }));

    request_animation_frame(&closure_clone);
}

fn update(balls: &mut RefMut<Vec<Ball>>, world: &mut World) {
    let canvas = query_selector_to::<HtmlCanvasElement>("canvas").unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    ctx.set_fill_style(&js_color_rgba(0.0, 0.0, 0.0, 1.0));
    ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    let mut ballrefs: Vec<&Ball> = vec![];
    for ball in balls.iter() {
        ballrefs.push(ball.clone());
    }

    ctx.set_stroke_style(&JsValue::from_str("rgba(0.0,255.0,255.0,0.2)"));
    ctx.set_line_width(2.0);

    let loot_node = create_tree(ballrefs.clone(), &ctx, false);

    // let balls_with_possible_contact = get_contact_with(
    //     loot_node.left_child.as_ref().unwrap(),
    //     loot_node.right_child.as_ref().unwrap(),
    //     Rc::new(RefCell::new(vec![])),
    // );

    let loot_node_box = Box::new(loot_node);
    let balls_with_possible_contact: Rc<RefCell<Vec<(&Ball, &Ball)>>> =
        Rc::new(RefCell::new(vec![]));

    for ball in balls.iter() {
        get_contact_with(&ball, &loot_node_box, balls_with_possible_contact.clone());
    }

    ctx.set_stroke_style(&JsValue::from_str("rgba(255.0,0.0,0.0,1)"));
    ctx.set_line_width(4.0);

    for balls in balls_with_possible_contact.borrow().iter() {
        let aabb = Aabb::from_ballrefs(&vec![balls.0, balls.1]);
        draw_aabb(&ctx, &aabb);
    }

    //ここで狭域当たり判定

    let delta_time = world.consts.delta_time;

    for ball in balls.iter_mut() {
        ball.draw(&ctx);
        ball.moving(
            world.consts.canvas_x as f64,
            world.consts.canvas_y as f64,
            &delta_time,
        );
    }
}

fn get_contact_with<'a>(
    ball: &'a Ball,
    node: &Box<Node<'a>>,
    balls_with_possible_contact: Rc<RefCell<Vec<(&'a Ball, &'a Ball)>>>,
) {
    let ball_aabb = Aabb::from_circle(ball.x, ball.y, ball.size);

    //接触なし
    if !ball_aabb.is_intersects(&node.aabb) {
        return;
    }

    //接触
    if node.balls.len() == 1 {
        if node.balls[0].id <= ball.id {
            return;
        }

        balls_with_possible_contact
            .borrow_mut()
            .push((ball, node.balls[0]));

        return;
    }
    //リーフノードでない場合、再帰的にツリーを降下する
    else if node.balls.len() > 1 {
        get_contact_with(
            ball,
            node.left_child.as_ref().unwrap(),
            balls_with_possible_contact.clone(),
        );

        get_contact_with(
            ball,
            node.right_child.as_ref().unwrap(),
            balls_with_possible_contact,
        );
    }
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
    //丸め誤差対策のために中心から+-0.5の範囲をセンターに入れてしまう 近接領域の当たりで多少の誤差が発生するけど1ピクセルより小さい領域での話なので事実上誤差は無いものとできるはず
    let mut left_balls: Vec<&Ball> = vec![];
    let mut right_balls: Vec<&Ball> = vec![];
    let mut center_balls: Vec<&Ball> = vec![];

    //フラグで分割する軸を変更
    if y_axis_division {
        let parent_center_y = (aabb.y_max + aabb.y_min) / 2.0;

        for ball in balls.iter() {
            if ball.y < parent_center_y + 0.5 && ball.y > parent_center_y - 0.5 {
                center_balls.push(ball.clone());
            } else if ball.y < parent_center_y {
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
            if ball.x < parent_center_x + 0.5 && ball.x > parent_center_x - 0.5 {
                center_balls.push(ball.clone());
            } else if ball.x < parent_center_x {
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
        let size = random_f64(5.0, 10.0);
        let ball = Ball::new(
            random_f64(0.0 + size, canvas.width() as f64 - size),
            random_f64(0.0 + size, canvas.height() as f64 - size),
            random_f64(-2.0, 2.0),
            random_f64(-2.0, 2.0),
            random_rgb(),
            size,
            i,
        );

        balls_rc.borrow_mut().push(ball);
    }
}

#[derive(Debug)]
pub struct Ball {
    x: f64,
    y: f64,
    vel_x: f64,
    vel_y: f64,
    color: JsValue,
    size: f64,
    id: i32,
}
impl Ball {
    fn new(x: f64, y: f64, vel_x: f64, vel_y: f64, color: JsValue, size: f64, id: i32) -> Ball {
        Ball {
            x,
            y,
            vel_x,
            vel_y,
            color,
            size,
            id,
        }
    }
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.set_fill_style(&self.color);
        ctx.arc(self.x, self.y, self.size, 0.0, 2.0 * std::f64::consts::PI)
            .unwrap();
        ctx.fill();
    }

    fn moving(&mut self, canvas_width: f64, canvas_height: f64, delta_time: &f64) {
        if self.x + self.size >= canvas_width {
            self.x = canvas_width - self.size;
            self.vel_x = -self.vel_x;
        }
        if self.x - self.size <= 0.0 {
            self.x = self.size;
            self.vel_x = -self.vel_x;
        }
        if self.y + self.size >= canvas_height {
            self.y = canvas_height - self.size;
            self.vel_y = -self.vel_y;
        }
        if self.y - self.size <= 0.0 {
            self.y = self.size;
            self.vel_y = -self.vel_y;
        }
        self.x += self.vel_x * delta_time / 20.0;
        self.y += self.vel_y * delta_time / 20.0;
    }

    fn aabb(&self) -> Aabb {
        Aabb::from_circle(self.x, self.y, self.size)
    }
}
