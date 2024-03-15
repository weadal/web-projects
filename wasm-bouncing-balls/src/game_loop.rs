use crate::{
    structs::ecs::*,
    systems::{sys_draw, *},
};

use web_sys::CanvasRenderingContext2d;

pub fn tick(w: &mut World, ctx: &CanvasRenderingContext2d) {
    //sys_main::create_ball_by_time(world);

    sys_draw::draw(w, ctx);
    sys_player::draw_player_range(w, ctx);

    if w.vars.is_stop {
        return;
    }

    sys_collision::collision(w, ctx);
    sys_main::position_update(w);
    sys_player::player_move(w);
    sys_player::player_attack(w);

    sys_weapon::time_increase(w);
    sys_weapon::fire(w);

    sys_main::ball_reflection(w);
    sys_main::player_reflection(w);
    sys_main::remove_out_of_bounds(w);
    sys_main::check_gameover(w);

    sys_main::update_timer(w);
}
