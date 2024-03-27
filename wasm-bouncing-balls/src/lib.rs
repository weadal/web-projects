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
use structs::{
    ecs::{EntityId, World},
    metagames::{self, GameManager},
    structs_util::{GameState, Vector2},
};
use systems::{sys_collision::EntityAabb, *};
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

    canvas_setting(input.clone());

    html_ui_setting(input.clone());

    input.borrow_mut().is_playing = true;

    main_loop(input.clone());

    Ok(())
}

fn canvas_setting(input: Rc<RefCell<Input>>) {
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
    let bounding_rect_clone = bounding_rect.clone();

    let mouse_down_closure = Closure::wrap(Box::new(move |e: MouseEvent| {
        let mut local_coordinate = Vector2::zero();
        local_coordinate.x = (e.client_x() as f64 - bounding_rect_clone.left()) * scale_x;
        local_coordinate.y = (e.client_y() as f64 - bounding_rect_clone.top()) * scale_y;

        input_clone.borrow_mut().mouse_down_point = Some(local_coordinate);

        input_clone.borrow_mut().is_mouse_down = true;
        log(&format!(
            "mouse_down start x:{},y:{}",
            local_coordinate.x, local_coordinate.y
        ));
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("mousedown", mouse_down_closure.as_ref().unchecked_ref())
        .unwrap();
    mouse_down_closure.forget();

    let input_clone = input.clone();

    //canvas上をクリックすることでキャンバス上の座標を取得するイベントハンドラ
    let mouse_up_closure = Closure::wrap(Box::new(move |e: MouseEvent| {
        //クリックされた絶対位置から矩形領域の位置を引いてローカル座標を取得する また、相対スケールも掛けておく
        let mut local_coordinate = Vector2::zero();
        local_coordinate.x = (e.client_x() as f64 - bounding_rect.left()) * scale_x;
        local_coordinate.y = (e.client_y() as f64 - bounding_rect.top()) * scale_y;

        input_clone.borrow_mut().click_point = Some(local_coordinate);

        let mouse_delta = input_clone.borrow().mouse_down_point.unwrap() - local_coordinate;

        let mouse_delta_normalize = mouse_delta.normalize();
        log(&format!(
            "click! local_x:{},local_y:{}",
            local_coordinate.x, local_coordinate.y
        ));
        log(&format!(
            "mouse_delta x:{},y:{}",
            mouse_delta_normalize.x, mouse_delta_normalize.y
        ));
        log(&format!(
            "mousedown time :{}",
            input_clone.borrow().mouse_down_time
        ));

        input_clone.borrow_mut().is_mouse_down = false;
        input_clone.borrow_mut().mouse_down_time = 0.0;

        input_clone.borrow_mut().mouse_down_point = None;
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("mouseup", mouse_up_closure.as_ref().unchecked_ref())
        .unwrap();
    mouse_up_closure.forget();

    //canvas設定ここまで
}
fn html_ui_setting(input: Rc<RefCell<Input>>) {
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
}

fn main_loop(input: Rc<RefCell<Input>>) {
    let closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let closure_clone = closure.clone();

    let mut fps = Fps::new();

    let mut game_manager = metagames::GameManager::new();

    let input_rc_clone = input.clone();
    *closure_clone.borrow_mut() = Some(Closure::new(move || {
        update(&mut game_manager, &input_rc_clone);
        fps.render();

        game_manager.vars.delta_time = fps.delta_time;

        if input_rc_clone.borrow().is_mouse_down {
            input_rc_clone.borrow_mut().mouse_down_time += fps.delta_time;
        }

        request_animation_frame(&closure);
    }));

    request_animation_frame(&closure_clone);
}

fn update(manager: &mut GameManager, input: &Rc<RefCell<Input>>) {
    if input.borrow().is_playing {
        let canvas = query_selector_to::<HtmlCanvasElement>("canvas").unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        match manager.state {
            GameState::Title => update_title(manager, input, &ctx),
            GameState::Main => update_main(manager, input, &ctx),
            GameState::GameOver => update_gameover(manager, input, &ctx),
            _ => (),
        }
    }
    input_postprocess(manager, input);
}

fn update_title(
    manager: &mut GameManager,
    input: &Rc<RefCell<Input>>,
    ctx: &CanvasRenderingContext2d,
) {
    let canvas = query_selector_to::<HtmlCanvasElement>("canvas").unwrap();

    ctx.set_fill_style(&js_color_rgba(0.0, 0.0, 0.0, 1.0));
    ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    log("Title");

    let mut input = input.borrow_mut();
    if input.click_point != None {
        input.clear_click_point();

        manager.vars.canvas_width = canvas.width();
        manager.vars.canvas_height = canvas.height();

        manager.world.consts.canvas_width = canvas.width();
        manager.world.consts.canvas_height = canvas.height();

        for _ in 0..1 {
            sys_enemy::create_ball(&mut manager.world);
        }

        sys_player::create_player(&mut manager.world);

        log("Game Start!");
        manager.state = GameState::Main;
    }
}

fn update_gameover(
    manager: &mut GameManager,
    input: &Rc<RefCell<Input>>,
    ctx: &CanvasRenderingContext2d,
) {
    let mut input = input.borrow_mut();

    if input.click_point != None {
        log("Return to Title...");
        input.clear_click_point();

        manager.world = World::new();
        manager.state = GameState::Title;
    }
}

fn update_main(
    manager: &mut GameManager,
    input: &Rc<RefCell<Input>>,
    ctx: &CanvasRenderingContext2d,
) {
    ctx.set_fill_style(&js_color_rgba(50.0, 30.0, 10.0, 1.0));
    ctx.fill_rect(
        0.0,
        0.0,
        manager.world.consts.canvas_width as f64,
        manager.world.consts.canvas_height as f64,
    );

    input_to_manager(manager, input);
    input_to_world(manager);

    //log(&format!("frame:{}", manager.world.consts.frame));

    game_loop::tick(&mut manager.world, ctx);

    manager.world.consts.frame += 1;

    if manager.world.vars.is_gameover {
        manager.state = GameState::GameOver;
        log("Game Over! Click or Tap to Title");
        log(&format!(
            "倒した敵の数:{}",
            manager.world.vars.defeated_enemy_count
        ));
    }
}

fn input_to_manager(manager: &mut GameManager, input: &Rc<RefCell<Input>>) {
    if let Some(point) = input.borrow().click_point {
        manager.vars.is_click_detection = true;

        manager.vars.last_screen_click_point = Some(point);
    }

    manager.vars.mouse_down_time = input.borrow().mouse_down_time;
}
fn input_to_world(manager: &mut GameManager) {
    manager.world.consts.delta_time = manager.vars.delta_time;

    if manager.vars.is_click_detection {
        manager.world.consts.last_screen_click_point = manager.vars.last_screen_click_point;

        let pos =
            manager.vars.last_screen_click_point.unwrap() + manager.world.vars.camera_position;

        log(&format!("ingame_click_point:{:?}", pos));

        manager.world.consts.last_ingame_click_point = Some(pos);
        manager.world.consts.is_click_detection = true;
    }

    if manager.vars.mouse_down_time > 200.0 {
        manager.world.vars.is_stop = true;
    }
}
fn input_postprocess(manager: &mut GameManager, input: &Rc<RefCell<Input>>) {
    manager.vars.is_click_detection = false;
    manager.world.consts.is_click_detection = false;
    input.borrow_mut().clear_click_point();
}
