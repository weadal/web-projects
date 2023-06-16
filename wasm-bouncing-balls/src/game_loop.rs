use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

use crate::{
    draw_old,
    draw_old::DrawMap,
    html_cast::*,
    structs::ecs::*,
    systems::{sys_draw, *},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    console, CanvasRenderingContext2d, DomRect, HtmlButtonElement, HtmlCanvasElement,
    HtmlInputElement, HtmlParagraphElement, MouseEvent, Performance,
};

fn game_loop() {
    let mut frame_time = Instant::now();

    let (mut tx_draw, rx_draw) = mpsc::channel::<DrawMap>();
    spawn(move || draw_old::draw_loop(rx_draw));

    let mut world = World::new();

    for _ in 0..2 {
        sys_main::create_ball(&mut world);
    }

    sys_main::create_timer(&mut world);

    let mut previous_frame_time = Instant::now();
    let mut now_frame_time;

    loop {
        //systems::create_ball_by_time(&mut world);
        //systems::ball_move(&mut world);
        //systems::ball_reflection(&mut world);
        sys_main::position_update(&mut world);
        //systems::ball_fire(&mut world);
        //systems::ball_collision(&mut world);
        //systems::collision(&mut world);
        //systems::ball_dead(&mut world);

        //systems::update_static_message(&mut world);
        //draw_system::create_draw_map(&mut world, &mut tx_draw);

        frame_time += Duration::from_nanos(16_666_667);
        //frame_time += Duration::from_nanos(8_333_334);
        let sleep_duration = frame_time.duration_since(Instant::now());
        sleep(sleep_duration);

        now_frame_time = Instant::now();
        let delta_time = now_frame_time
            .duration_since(previous_frame_time)
            .as_secs_f64();

        sys_main::update_timer(&mut world, &delta_time);

        previous_frame_time = Instant::now();
    }
}

fn main() {
    game_loop();
}

pub fn tick(w: &mut World) {
    //sys_main::create_ball_by_time(world);

    let canvas = query_selector_to::<HtmlCanvasElement>("canvas").unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    sys_draw::draw(w, &ctx);
    sys_main::position_update(w);
    //sys_main::ball_reflection(w);
    sys_main::remove_out_of_bounds(w);
}
