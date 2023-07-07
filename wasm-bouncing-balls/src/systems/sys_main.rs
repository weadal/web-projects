use rand::Rng;

use crate::{structs::ecs::*, structs::structs_util::*, user_consts::*, utils::*};

use super::{
    sys_collision::{Circle, Collider, Rect, Shape},
    sys_draw::DrawParamater,
};

pub fn create_ball(w: &mut World) {
    //ボールのentityを作成し、戻り値でentityのidを得る
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    //position初期化
    let mut rng = rand::thread_rng();
    let mut rand_x = rng.gen_range(BALL_SIZE * 2.0..w.consts.canvas_x as f64 - BALL_SIZE * 2.0);
    let mut rand_y = rng.gen_range(BALL_SIZE * 2.0..w.consts.canvas_y as f64 - BALL_SIZE * 2.0);

    let pos = Vector2 {
        x: rand_x as f64,
        y: rand_y as f64,
    };

    //velocity初期化
    rand_x = rng.gen_range(-1.0..1.0);
    rand_y = rng.gen_range(-1.0..1.0);
    let vel = Vector2::normalize(&Vector2 {
        x: rand_x,
        y: rand_y,
    });

    let transform = Transform {
        id,
        position: pos,
        scale: 1.0,
        velocity: vel * 100.0,
        parent: None,
        children: None,
    };

    w.transform.register(entity, transform);

    let rect = Rect::new(BALL_SIZE, BALL_SIZE);
    let collider = Collider::new(rect, Group::Ball, Vector2::zero());
    w.collider.register(entity, vec![collider]);

    let r = Rect::new(BALL_SIZE, BALL_SIZE);
    let draw_param = DrawParamater::new(js_color_rgba(255.0, 255.0, 255.0, 1.0), Shape::Rect(r));

    w.draw_param.register(entity, draw_param);
    w.group.register(entity, Group::Ball);
    w.clock.register(entity, Clock::new());
}

pub fn update_timer(w: &mut World) {
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
        w.transform.set(entity_id, transform);
    }
}

pub fn ball_reflection(w: &mut World) {
    let entities = collect_entities_from_group(w, &Group::Ball);
    for entity_id in entities.iter() {
        let mut transform = w.transform.get(entity_id).unwrap().clone();

        let collider = w.collider.get_unchecked(entity_id)[0].clone();
        let width = collider.shape.width;
        let height = collider.shape.height;

        if transform.position.x <= width || transform.position.x >= w.consts.canvas_x as f64 - width
        {
            transform.velocity.x = -transform.velocity.x;
        }

        if transform.position.y <= height
            || transform.position.y >= w.consts.canvas_y as f64 - height
        {
            transform.velocity.y = -transform.velocity.y;
        }
        w.transform.set(entity_id, transform);
    }
}

pub fn ball_move(w: &mut World) {
    let entities = collect_entities_from_group(w, &Group::Ball);
    for entity_id in entities.iter() {
        let mut transform = w.transform.get(entity_id).unwrap().clone();
        let vel_mag = transform.velocity.magnitude();
        let right = transform.velocity.right() * 0.02;
        transform.velocity = transform.velocity + right;
        transform.velocity = Vector2::normalize(&transform.velocity) * vel_mag;
        w.transform.set(entity_id, transform);
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

fn burret_draw_param() -> DrawParamater {
    let c = Circle::new(BULLET_SIZE);
    DrawParamater {
        color: js_color_rgba(255.0, 255.0, 255.0, 1.0),
        shape: Shape::Circle(c),
    }
}
pub fn create_bullet(w: &mut World, parent_id: &EntityId) -> EntityId {
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    let parent_transform = w.transform.get_unchecked_mut(parent_id);

    let mut transform = parent_transform.clone();
    transform.set_parent(parent_transform);

    transform.id = id;

    let mut vel = Vector2::normalize(&Vector2 { x: 1.0, y: 0.0 });
    vel = vel.rotate(90.0);

    transform.velocity = vel * 100.0;
    transform.scale = BULLET_SIZE;

    w.transform.register(entity, transform);

    w.draw_param.register(entity, burret_draw_param());
    w.group.register(entity, Group::Bullet);
    w.collider.register(entity, vec![]);
    id
}

pub fn create_aim_bullet(w: &mut World, parent_id: &EntityId, direction: &Vector2) {
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    let vel = Vector2::normalize(direction);

    let parent_transform = w.transform.get_unchecked_mut(parent_id);
    let mut transform = parent_transform.clone();
    transform.set_parent(parent_transform);

    transform.id = id;
    transform.position = transform.position + vel;
    transform.velocity = vel * 0.5;
    transform.scale = BULLET_SIZE;

    w.transform.register(entity, transform);

    w.draw_param.register(entity, burret_draw_param());
    w.group.register(entity, Group::Bullet);
    w.collider.register(entity, vec![]);
}

pub fn remove_out_of_bounds(w: &mut World) {
    //暫定的にコライダー持ちをすべて処理(将来的にコライダーを持ったフィールド外のオブジェクトが欲しくなるかも)
    let entities = collect_entities_from_archetype(&w, &[w.collider.id()]);
    for entity_id in entities.iter() {
        //ボールがコートの外に出たときに消滅させる
        let pos = w.transform.get(entity_id).unwrap().position;
        let aabb = w.draw_param.get(entity_id).unwrap().shape.local_aabb();

        //とりあえず描画のAABBが画面外に出たら破棄する(コライダーのAABBは描画のAABBより小さいものとする)
        if pos.x > w.consts.canvas_x as f64 + aabb.x_max
            || pos.x < aabb.x_min
            || pos.y > w.consts.canvas_y as f64 + aabb.y_max
            || pos.y < aabb.y_min
        {
            w.remove_entity(entity_id);
            crate::log(&format!("領域外に落ちたentity(id:{:?})を破棄", entity_id))
        }
    }
}

pub fn nearest_target(w: &World, self_id: &EntityId, group: &Group) -> Option<(EntityId, Vector2)> {
    let position = w.transform.get(self_id).unwrap().clone().position;
    let mut nearest_distance = std::f64::MAX;
    let mut nearest_target_tupple: Option<(EntityId, Vector2)> = None;

    for value in w.transform.items.iter() {
        if value.id == *self_id {
            continue;
        }

        match w.group.get(&value.id) {
            None => continue,
            Some(ref_group) => {
                if ref_group != group {
                    continue;
                }
            }
        }

        let ref_distance = (value.item.position - position).sqr_magnitude();

        if nearest_distance > ref_distance {
            nearest_distance = ref_distance;
            nearest_target_tupple = Some((value.id, value.item.position));
        }
    }

    nearest_target_tupple
}

pub fn check_gameover(w: &mut World) {
    //暫定的に全エンティティがいなくなったらゲームオーバー
    let alive_entities = w.entities.get_alive_entities();
    if let None = alive_entities {
        w.vars.state = GameState::GameOver;
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
