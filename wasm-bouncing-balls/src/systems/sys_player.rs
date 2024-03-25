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

    let mut range = 0;

    for weapon in w.weapon.get_unchecked(&weapons_id) {
        if let Some(wp) = weapon {
            if wp.range > range {
                range = wp.range;
            }
        }
    }

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
    let collider_self = Collider::new(rect, *w.group.get_unchecked(&id), Vector2::zero());

    let rect = Rect::new(range as f64, range as f64);
    let collider_range = Collider::new(rect, *w.group.get_unchecked(&id), Vector2::zero());

    w.collider
        .register(entity, vec![collider_self, collider_range]);

    let c = Circle::new(BALL_SIZE);
    let draw_param = DrawParamater::new(js_color_rgba(0.0, 255.0, 255.0, 1.0), Shape::Circle(c));

    w.draw_param.register(entity, draw_param);
    w.destination.register(entity, vec![None]);

    let mut clock = Clock::new();
    clock.timer_create_and_set(0.0, p_timer::ALIVE_TIME);
    clock.timer_create_and_set(0.0, p_timer::TARGETING_TIME);

    w.clock.register(entity, clock);

    w.player_vars.register(entity, vars);
}

//現状簡易的な移動
//最終的には移動前に未来位置予測し、現在位置から未来位置までの間に目的地か障害物があったらそこでストップするようにする
pub fn player_move(w: &mut World) {
    let entities = collect_entities_from_group(w, &Group::Player);
    for entity_id in entities.iter() {
        let is_set_destination = player_next_destination_set(w, entity_id);

        let mut transform = w.transform.get_unchecked(entity_id).clone();
        let mut position = transform.position;

        if let Some(dest) = w.destination.get(entity_id) {
            if let Some(next_dest) = dest[0] {
                //到着処理
                if transform.position.distance(&next_dest) <= 2.0 {
                    w.destination.set(entity_id, Some(vec![None]));
                    transform.velocity = Vector2::zero();
                } else {
                    if is_set_destination {
                        let direction = (next_dest - transform.position).normalize();
                        transform.velocity = direction * 100.0;
                    }

                    //この辺で当たり判定して到着処理？
                }
                w.transform.set(entity_id, Some(transform));
            }
        }

        position.x = position.x - w.consts.canvas_width as f64 / 2.0;
        position.y = position.y - w.consts.canvas_height as f64 / 2.0;
        w.vars.camera_position = position;
    }
}

fn player_next_destination_set(w: &mut World, entity_id: &EntityId) -> bool {
    if !w.consts.is_click_detection {
        return false;
    }

    let mut dest = w.destination.get_unchecked(entity_id).clone();
    let mut next_dest: Option<Vector2> = None;

    if let Some(click_point) = w.consts.last_ingame_click_point {
        match dest[0] {
            Some(current_dest) => {
                if current_dest != click_point {
                    next_dest = Some(click_point);
                } else {
                    //次の目的地とクリック座標が同じ場合早期リターン
                    return false;
                }
            }
            None => next_dest = Some(click_point),
        }
    }

    if let Some(_) = next_dest {
        dest[0] = next_dest;
        w.destination.set(entity_id, Some(dest));
    }
    true
}

pub fn player_targeting(w: &mut World) {
    let entities = collect_entities_from_group(w, &Group::Player);
    for entity_id in entities.iter() {}
}

pub fn player_attack(w: &mut World) {
    let entities = collect_entities_from_group(w, &Group::Player);
    for player_entity_id in entities.iter() {
        let targeting_time = w.clock.get_unchecked(player_entity_id).timer[p_timer::TARGETING_TIME]
            .unwrap()
            .clone();

        let transform = w.transform.get_unchecked(player_entity_id);
        let collider = w.collider.get_unchecked(player_entity_id);

        let player_range_aabb = EntityAabb {
            entity_id: player_entity_id.clone(),
            position: transform.position,
            aabb: collider[p_collider::MAX_RANGE].aabb(transform.position),
        };

        let contact_entities =
            sys_collision::get_contact_with_group(w, player_range_aabb, Group::Enemy);
        let vars = w.player_vars.get_unchecked(player_entity_id);

        //暫定的に最もレンジの長い武器が接触判定したらすべての武器をアクティブにする
        //そもそもこの処理自体毎フレーム行うようなものじゃないので後でいい感じにしたい
        match contact_entities {
            Some(_) => {
                let mut weapons = w.weapon.take_unchecked(&vars.weapons_id);
                for weapon in weapons.iter_mut() {
                    if let Some(wp) = weapon {
                        wp.is_active = true;
                    }
                }
                w.weapon.set(&vars.weapons_id, Some(weapons));
            }
            None => {
                let mut weapons = w.weapon.take_unchecked(&vars.weapons_id);
                for weapon in weapons.iter_mut() {
                    if let Some(wp) = weapon {
                        wp.is_active = false;
                    }
                }
                w.weapon.set(&vars.weapons_id, Some(weapons));
            }
        }
    }
}

pub fn draw_player_range(w: &mut World, ctx: &CanvasRenderingContext2d) {
    ctx.set_stroke_style(&JsValue::from_str("rgba(255.0,255.0,255.0,0.4)"));
    ctx.set_line_width(2.0);

    let entities = collect_entities_from_group(w, &Group::Player);
    for entity_id in entities.iter() {
        let pos = w.transform.get_unchecked(entity_id).position;
        let aabb = w.collider.get_unchecked(entity_id)[p_collider::MAX_RANGE].aabb(pos);
        sys_draw::draw_aabb(ctx, &aabb, w.vars.camera_position);
    }
}

pub mod p_timer {
    pub const ALIVE_TIME: usize = 0;
    pub const TARGETING_TIME: usize = 1;
}
pub mod p_collider {
    pub const SELF: usize = 0;
    pub const MAX_RANGE: usize = 1;
}

pub struct PlayerVars {
    pub weapons_id: EntityId,
}
