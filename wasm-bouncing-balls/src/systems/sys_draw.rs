use std::sync::mpsc::Sender;

use wasm_bindgen::JsValue;

use super::{sys_collision::Shape, sys_main};
use crate::{
    log,
    structs::{ecs::*, structs_util::*},
    systems::sys_main::*,
    user_consts::{self, *},
    utils::*,
};
use web_sys::{
    console, CanvasRenderingContext2d, DomRect, HtmlButtonElement, HtmlCanvasElement,
    HtmlInputElement, HtmlParagraphElement, MouseEvent, Performance,
};

#[derive(Debug, Clone)]
pub struct DrawParamater {
    pub color: JsValue,
    pub shape: Shape,
}
impl DrawParamater {
    pub fn new(color: JsValue, shape: Shape) -> DrawParamater {
        DrawParamater { color, shape }
    }
}

pub fn draw(w: &mut World, ctx: &CanvasRenderingContext2d) {
    // ctx.set_fill_style(&js_color_rgba(0.0, 0.0, 0.0, 1.0));
    // ctx.fill_rect(
    //     0.0,
    //     0.0,
    //     w.consts.canvas_width as f64,
    //     w.consts.canvas_height as f64,
    // );

    let entities = collect_entities_from_archetype(&w, &[w.draw_param.id()]);

    for entity_id in entities.iter() {
        let pos = w.transform.get_unchecked(entity_id).position - w.vars.camera_position;
        let param = w.draw_param.get_unchecked(entity_id);

        match &param.shape {
            Shape::Circle(c) => {
                ctx.begin_path();
                ctx.set_fill_style(&param.color);
                ctx.arc(
                    pos.x + c.offset.x,
                    pos.y + c.offset.y,
                    c.radius,
                    0.0,
                    2.0 * std::f64::consts::PI,
                )
                .unwrap();
                ctx.fill();
            }
            Shape::Rect(r) => {
                ctx.set_fill_style(&param.color);
                ctx.fill_rect(
                    pos.x + (-r.width / 2.0 + r.offset.x),
                    pos.y + (-r.height / 2.0 + r.offset.y),
                    r.width,
                    r.height,
                );
            }
        }
    }
}
pub fn draw_once(
    w: &mut World,
    ctx: &CanvasRenderingContext2d,
    param: &DrawParamater,
    pos: &Vector2,
) {
}

pub fn draw_background(w: &mut World, ctx: &CanvasRenderingContext2d) {
    let mut width: i32 = -(w.vars.camera_position.x as i32 % 200) - 400;
    let mut height: i32 = -(w.vars.camera_position.y as i32 % 200) - 400;

    ctx.set_fill_style(&js_color_rgba(255.0, 248.0, 220.0, 0.1)); //cornsilk色

    let mut column = 0;

    while height < w.consts.canvas_height as i32 {
        while width < w.consts.canvas_width as i32 {
            ctx.fill_rect(width as f64, height as f64, 100.0, 100.0);
            width += 200;
        }
        height += 100;
        column += 100;
        width = column % 200 - w.vars.camera_position.x as i32 % 200 - 400;
    }
}

pub fn draw_aabb(ctx: &CanvasRenderingContext2d, aabb: &Aabb, camera_pos: Vector2) {
    ctx.begin_path();

    ctx.move_to(aabb.x_min - camera_pos.x, aabb.y_min - camera_pos.y);
    ctx.line_to(aabb.x_max - camera_pos.x, aabb.y_min - camera_pos.y);
    ctx.line_to(aabb.x_max - camera_pos.x, aabb.y_max - camera_pos.y);
    ctx.line_to(aabb.x_min - camera_pos.x, aabb.y_max - camera_pos.y);
    ctx.line_to(aabb.x_min - camera_pos.x, aabb.y_min - camera_pos.y);

    ctx.stroke();
}
