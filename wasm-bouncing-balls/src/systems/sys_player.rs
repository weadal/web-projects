use rand::Rng;

use crate::{
    structs::ecs::*,
    structs::{structs_util::*, weapon::*},
    user_consts::*,
    utils::*,
    *,
};

use super::{
    sys_collision::{Circle, Collider, Rect, Shape},
    sys_draw::DrawParamater,
    sys_main::*,
};

pub fn create_player(w: &mut World) {
    let id = w.entities.instantiate_entity();

    let weapons_id = sys_weapon::create_weapons(w, id);
    let vars = PlayerVars { weapons_id };

    let entity = w.entities.get_mut(&id).unwrap();

    let position = Vector2::new(
        w.consts.canvas_width as f64 / 2.0,
        w.consts.canvas_height as f64 / 2.0,
    );

    let transform = Transform {
        id,
        position,
        scale: 1.0,
        velocity: Vector2::zero(),
        parent: None,
        children: None,
    };

    w.transform.register(entity, transform);
    w.group.register(entity, Group::Player);

    let rect = Rect::new(BALL_SIZE, BALL_SIZE);
    let collider = Collider::new(rect, *w.group.get_unchecked(&id), Vector2::zero());
    w.collider.register(entity, vec![collider]);

    let c = Circle::new(BALL_SIZE);
    let draw_param = DrawParamater::new(js_color_rgba(0.0, 255.0, 255.0, 1.0), Shape::Circle(c));

    w.draw_param.register(entity, draw_param);
    w.destination.register(entity, vec![None]);

    let mut clock = Clock::new();
    clock.timer_create_and_set(0.0, p_timer::ALIVE_TIME);
    clock.timer_create_and_set(0.0, p_timer::TARGETING_TIME);
    clock.timer_create_and_set(0.0, p_timer::ATTACK_DURATION_TIME);

    w.clock.register(entity, clock);

    w.player_vars.register(entity, vars);
}

//現状簡易的な移動
//最終的には移動前に未来位置予測し、現在位置から未来位置までの間に目的地か障害物があったらそこでストップするようにする
pub fn player_move(w: &mut World) {
    let entities = collect_entities_from_group(w, &Group::Player);
    for entity_id in entities.iter() {
        player_next_destination_set(w, entity_id);

        if let Some(dest) = w.destination.get(entity_id) {
            if let Some(next_dest) = dest[0] {
                let mut transform = w.transform.get_unchecked(entity_id).clone();

                //到着処理
                if transform.position.distance(&next_dest) <= 2.0 {
                    w.destination.set(entity_id, vec![None]);
                    transform.velocity = Vector2::zero();
                } else {
                    let direction = (next_dest - transform.position).normalize();
                    transform.velocity = direction * 100.0;
                }
                w.transform.set(entity_id, transform);
            }
        }
    }
}

fn player_next_destination_set(w: &mut World, entity_id: &EntityId) {
    if !w.vars.is_click_detection {
        return;
    }

    let mut dest = w.destination.get_unchecked(entity_id).clone();
    let mut next_dest: Option<Vector2> = None;

    if let Some(click_point) = w.vars.last_click_point {
        match dest[0] {
            Some(current_dest) => {
                if current_dest != click_point {
                    next_dest = Some(click_point);
                } else {
                    //次の目的地とクリック座標が同じ場合早期リターン
                    return;
                }
            }
            None => next_dest = Some(click_point),
        }
    }

    if let Some(_) = next_dest {
        dest[0] = next_dest;
        w.destination.set(entity_id, dest);
    }
}

pub fn player_targeting(w: &mut World) {
    let entities = collect_entities_from_group(w, &Group::Player);
    for entity_id in entities.iter() {}
}

// pub fn player_attack(w: &mut World) {
//     let entities = collect_entities_from_group(w, &Group::Player);
//     for entity_id in entities.iter() {
//         let targeting_time = w.clock.get_unchecked(entity_id).timer[p_timer::TARGETING_TIME]
//             .unwrap()
//             .clone();

//         let attack_time = w.clock.get_unchecked(entity_id).timer[p_timer::ATTACK_DURATION_TIME]
//             .unwrap()
//             .clone();

//         if attack_time > 1000.0 {
//             create_bullet(w, entity_id);
//             w.clock
//                 .get_unchecked_mut(entity_id)
//                 .timer_reset(p_timer::ATTACK_DURATION_TIME);
//         }
//     }
// }

pub mod p_timer {
    pub const ALIVE_TIME: usize = 0;
    pub const TARGETING_TIME: usize = 1;
    pub const ATTACK_DURATION_TIME: usize = 2;
}

pub struct PlayerVars {
    pub weapons_id: EntityId,
}
