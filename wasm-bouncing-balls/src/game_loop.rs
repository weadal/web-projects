use crate::{
    log,
    structs::ecs::*,
    systems::{sys_draw, *},
};

use web_sys::CanvasRenderingContext2d;

pub fn tick(w: &mut World, ctx: &CanvasRenderingContext2d) {
    //sys_main::create_ball_by_time(world);

    sys_draw::draw_background(w, ctx);
    sys_draw::draw(w, ctx);
    sys_player::draw_player_range(w, ctx);

    if w.vars.is_stop {
        log(&format!("a"));

        if w.consts.is_click_detection {
            log(&format!("b"));
            sys_main::create_building(w, &w.consts.last_ingame_click_point.unwrap());
            w.vars.is_stop = false;
        }
        return;
    }

    sys_collision::collision(w, ctx);
    sys_main::position_update(w);
    sys_player::player_move(w);
    sys_player::player_attack(w);

    sys_weapon::time_increase(w);
    sys_weapon::fire(w);

    sys_enemy::ball_reflection(w);
    sys_main::player_reflection(w);
    sys_main::remove_out_of_bounds(w);

    sys_main::update_timer(w);
    //log(&format!("time:{:?}", w.vars.ingame_time));

    sys_main::check_gameover(w);
}
