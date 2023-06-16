use std::sync::mpsc::Sender;

use wasm_bindgen::JsValue;

use super::{sys_collision::Shape, sys_main};
use crate::{
    structs::ecs::*,
    structs::util::*,
    systems::sys_main::*,
    user_consts::{self, *},
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
    let entities = collect_entities_from_archetype(&w, &[w.draw_param.id()]);

    for entity_id in entities.iter() {
        let pos = w.position.get_unchecked(entity_id);
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
