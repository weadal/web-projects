use rand::Rng;

use crate::{
    log,
    structs::{ecs::*, structs_util::*, weapon::BulletParamater},
    user_consts::*,
    utils::*,
};

use super::{
    sys_collision::{Circle, Collider, Rect, Shape},
    sys_draw::DrawParamater,
};

pub fn create_building(w: &mut World, position: &Vector2) {
    //ボールのentityを作成し、戻り値でentityのidを得る
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    //position初期化

    let pos = Vector2 {
        x: position.x,
        y: position.y,
    };

    //velocity初期化

    let transform = Transform {
        id,
        position: pos,
        scale: 1.0,
        velocity: Vector2::zero(),
        parent: None,
        children: None,
    };

    w.transform.register(entity, transform);

    let rect = Rect::new(100.0, 100.0);
    let collider = Collider::new(rect, Group::Building, Vector2::zero());
    w.collider.register(entity, vec![collider]);

    let rect = Rect::new(100.0, 100.0);
    let draw_param = DrawParamater::new(js_color_rgba(255.0, 255.0, 0.0, 1.0), Shape::Rect(rect));

    w.draw_param.register(entity, draw_param);
    w.group.register(entity, Group::Building);
    w.clock.register(entity, Clock::new());
}
pub fn update_timer(w: &mut World) {
    w.vars.ingame_time += w.consts.delta_time;

    let entities = collect_entities_from_archetype(&w, &[w.clock.id()]);

    for entity_id in entities.iter() {
        let clock = w.clock.get(entity_id).unwrap().timer.clone();
        for (index, time) in clock.iter().enumerate() {
            if let Some(t) = time {
                w.clock.get_unchecked_mut(entity_id).timer[index] = Some(t + w.consts.delta_time);
            }
        }
    }
}

pub fn position_update(w: &mut World) {
    let entities = collect_entities_from_archetype(&w, &[w.transform.id()]);

    for entity_id in entities.iter() {
        let mut transform = w.transform.get(entity_id).unwrap().clone();

        let vel = transform.velocity;

        transform.position.x += vel.x * w.consts.delta_time / 1000.0;
        transform.position.y += vel.y * w.consts.delta_time / 1000.0;
        w.transform.set(entity_id, Some(transform));
    }
}
pub fn player_reflection(w: &mut World) {
    let screen_left = w.vars.camera_position.x;
    let screen_right = w.consts.canvas_width as f64 + w.vars.camera_position.x;
    let screen_top = w.vars.camera_position.y;
    let screen_bottom = w.consts.canvas_height as f64 + w.vars.camera_position.y;

    let entities = collect_entities_from_group(w, &Group::Player);
    for entity_id in entities.iter() {
        let mut transform = w.transform.get(entity_id).unwrap().clone();

        let collider = w.collider.get_unchecked(entity_id)[0].clone();
        let width = collider.shape.width;
        let height = collider.shape.height;

        //左端
        if transform.position.x <= screen_left + width {
            transform.position.x = screen_left + width;
            transform.velocity.x = -transform.velocity.x;
        }
        //右端
        if transform.position.x >= screen_right - width {
            transform.position.x = screen_right - width;
            transform.velocity.x = -transform.velocity.x;
        }
        //上端
        if transform.position.y <= screen_top + height {
            transform.position.y = screen_top + height;
            transform.velocity.y = -transform.velocity.y;
        }
        //下端
        if transform.position.y >= screen_bottom - height {
            transform.position.y = screen_bottom - height;
            transform.velocity.y = -transform.velocity.y;
        }
        w.transform.set(entity_id, Some(transform));
    }
}

// pub fn create_ball_by_time(w: &mut World) {
//     let entities = collect_entities_from_archetype(&w, &[w.clock.id(), w.timer_alarm.id()]);
//     let timer_id = &entities[0];

//     let timer = w.clock.get_mut(timer_id).unwrap().clone();

//     if timer > w.timer_alarm.get(timer_id).unwrap()[0] {
//         for _ in 0..BALL_SPAWN_MULTIPRIER {
//             create_ball(w);
//         }

//         let mut buffer = w.timer_alarm.get(timer_id).unwrap().clone();
//         buffer[0] = timer + BALL_SPAWN_SPAN;

//         w.timer_alarm.set(timer_id, buffer);
//     }
// }

// pub fn ball_fire(w: &mut World) {
//     let entities = collect_entities_from_group(w, &group::BALL);

//     for entity_id in entities.iter() {
//         let timer = w.clock.get(entity_id).unwrap().clone();
//         if timer < BULLET_FIRE_SPAN {
//             continue;
//         }

//         let target = nearest_target(w, entity_id, &group::BALL);
//         if let Some(value) = target {
//             let direction = value.1 - w.transform.get(entity_id).unwrap().clone().position;

//             create_aim_bullet(w, entity_id, &direction);
//             w.clock.set(entity_id, timer - BULLET_FIRE_SPAN);
//         }
//     }
// }

pub fn remove_out_of_bounds(w: &mut World) {
    let screen_left = w.vars.camera_position.x;
    let screen_right = w.consts.canvas_width as f64 + w.vars.camera_position.x;
    let screen_top = w.vars.camera_position.y;
    let screen_bottom = w.consts.canvas_height as f64 + w.vars.camera_position.y;

    //暫定的にコライダー持ちをすべて処理(将来的にコライダーを持ったフィールド外のオブジェクトが欲しくなるかも)
    let entities = collect_entities_from_archetype(&w, &[w.collider.id()]);
    for entity_id in entities.iter() {
        //ボールがコートの外に出たときに消滅させる
        let pos = w.transform.get(entity_id).unwrap().position;
        let aabb = w.draw_param.get(entity_id).unwrap().shape.local_aabb();

        //とりあえず描画のAABBが画面外に出たら破棄する(コライダーのAABBは描画のAABBより小さいものとする)
        if pos.x > screen_right + aabb.x_max
            || pos.x < screen_left + aabb.x_min
            || pos.y > screen_bottom + aabb.y_max
            || pos.y < screen_top + aabb.y_min
        {
            w.remove_entity(entity_id);
            //crate::log(&format!("領域外に落ちたentity(id:{:?})を破棄", entity_id))
        }
    }
}

// pub fn nearest_target(w: &World, self_id: &EntityId, group: &Group) -> Option<(EntityId, Vector2)> {
//     let position = w.transform.get(self_id).unwrap().clone().position;
//     let mut nearest_distance = std::f64::MAX;
//     let mut nearest_target_tupple: Option<(EntityId, Vector2)> = None;

//     for value in w.transform.items.iter() {
//         if value.id == *self_id {
//             continue;
//         }

//         match w.group.get(&value.id) {
//             None => continue,
//             Some(ref_group) => {
//                 if ref_group != group {
//                     continue;
//                 }
//             }
//         }

//         let ref_distance = (value.item.position - position).sqr_magnitude();

//         if nearest_distance > ref_distance {
//             nearest_distance = ref_distance;
//             nearest_target_tupple = Some((value.id, value.item.position));
//         }
//     }

//     nearest_target_tupple
// }

pub fn check_gameover(w: &mut World) {
    //制限時間になったらゲームオーバー デバッグ用に一時コメントアウト
    // if w.vars.ingame_time > 10000.0 {
    //     log(&format!("制限時間"));

    //     w.vars.is_gameover = true;
    // }

    //暫定的に全エンティティがいなくなったらゲームオーバー
    let alive_entities = w.entities.get_alive_entities();
    if let None = alive_entities {
        w.vars.is_gameover = true;
    }
}

pub fn collect_entities_from_archetype(w: &World, values: &[ComponentId]) -> Vec<EntityId> {
    let arche = EntytyArcheType::create_archetype(values);
    w.entities.get_entities_from_archetype(&arche)
}

pub fn collect_entities_from_group(w: &World, group_id: &Group) -> Vec<EntityId> {
    let mut group_entities = vec![];

    for entity in w.entities.entities.iter() {
        if let Some(value) = entity {
            if let Some(group) = w.group.get(&value.id) {
                if group == group_id {
                    group_entities.push(value.id)
                }
            }
        }
    }
    group_entities
}
