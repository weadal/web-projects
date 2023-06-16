use rand::Rng;

use crate::{
    structs::ecs::*,
    structs::util::*,
    user_consts::{self, *},
    utils::*,
};

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

    w.position.reserve(entity, pos);

    //velocity初期化
    rand_x = rng.gen_range(-1.0..1.0);
    rand_y = rng.gen_range(-1.0..1.0);
    let vel = Vector2::normalize(&Vector2 {
        x: rand_x,
        y: rand_y,
    });

    w.velocity.reserve(entity, vel * 100.0);

    let rect = Rect::new(BALL_SIZE, BALL_SIZE);
    let collider = Collider::new(rect, group::BALL, Vector2::zero());
    w.collider.reserve(entity, vec![collider]);

    let r = Rect::new(BALL_SIZE, BALL_SIZE);
    let draw_param = DrawParamater::new(js_color_rgba(255.0, 255.0, 255.0, 1.0), Shape::Rect(r));

    w.draw_param.reserve(entity, draw_param);
    w.group.reserve(entity, group::BALL);
    w.scale.reserve(entity, 1.0);
    w.timer_time.reserve_default(entity);
}

pub fn create_timer(w: &mut World) {
    let id = w.entities.instantiate_entity();

    w.timer_time.reserve(w.entities.get_mut(&id).unwrap(), 0.0);
    w.timer_alarm
        .reserve(w.entities.get_mut(&id).unwrap(), vec![0.0]);
}
pub fn update_timer(w: &mut World, delta_time: &f64) {
    let entities = collect_entities_from_archetype(&w, &[w.timer_time.id()]);

    for entity_id in entities.iter() {
        let now = w.timer_time.get(entity_id).unwrap().clone();
        w.timer_time.set(&entity_id, now + delta_time);
    }
}

pub fn position_update(w: &mut World) {
    let entities = collect_entities_from_archetype(&w, &[w.position.id(), w.velocity.id()]);

    for entity_id in entities.iter() {
        let mut pos = w.position.get(entity_id).unwrap().clone();
        let vel = w.velocity.get(entity_id).unwrap();

        pos.x += vel.x * w.consts.delta_time / 1000.0;
        pos.y += vel.y * w.consts.delta_time / 1000.0;
        w.position.set(entity_id, pos);
    }
}

pub fn ball_reflection(w: &mut World) {
    let entities = collect_entities_from_group(w, &group::BALL);
    for entity_id in entities.iter() {
        let pos = w.position.get(entity_id).unwrap();
        let mut vel = w.velocity.get(entity_id).unwrap().clone();

        let collider = w.collider.get_unchecked(entity_id)[0].clone();
        let width = collider.shape.width;
        let height = collider.shape.height;

        if pos.x <= width || pos.x >= w.consts.canvas_x as f64 - width {
            vel.x = -vel.x;
        }

        if pos.y <= height || pos.y >= w.consts.canvas_y as f64 - height {
            vel.y = -vel.y;
        }
        w.velocity.set(entity_id, vel);
    }
}

pub fn ball_move(w: &mut World) {
    let entities = collect_entities_from_group(w, &group::BALL);
    for entity_id in entities.iter() {
        let mut vel = w.velocity.get(entity_id).unwrap().clone();
        let vel_mag = vel.magnitude();
        let right = vel.right() * 0.02;
        vel = vel + right;
        w.velocity
            .set(entity_id, Vector2::normalize(&vel) * vel_mag);
    }
}

pub fn create_ball_by_time(w: &mut World) {
    let entities = collect_entities_from_archetype(&w, &[w.timer_time.id(), w.timer_alarm.id()]);
    let timer_id = &entities[0];

    let timer = w.timer_time.get_mut(timer_id).unwrap().clone();

    if timer > w.timer_alarm.get(timer_id).unwrap()[0] {
        for _ in 0..BALL_SPAWN_MULTIPRIER {
            create_ball(w);
        }

        let mut buffer = w.timer_alarm.get(timer_id).unwrap().clone();
        buffer[0] = timer + BALL_SPAWN_SPAN;

        w.timer_alarm.set(timer_id, buffer);
    }
}

pub fn ball_fire(w: &mut World) {
    let entities = collect_entities_from_group(w, &group::BALL);

    for entity_id in entities.iter() {
        let timer = w.timer_time.get(entity_id).unwrap().clone();
        if timer < BULLET_FIRE_SPAN {
            continue;
        }

        let target = nearest_target(w, entity_id, &group::BALL);
        if let Some(value) = target {
            let direction = value.1 - w.position.get(entity_id).unwrap().clone();

            create_aim_bullet(w, entity_id, &direction);
            w.timer_time.set(entity_id, timer - BULLET_FIRE_SPAN);
        }
    }
}
fn burret_draw_param() -> DrawParamater {
    let c = Circle::new(BULLET_SIZE);
    DrawParamater {
        color: js_color_rgba(255.0, 255.0, 255.0, 1.0),
        shape: Shape::Circle(c),
    }
}
pub fn create_bullet(w: &mut World, parent_id: &EntityId) {
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    w.parent.reserve(entity, parent_id.clone());

    let pos = w.position.get(parent_id).unwrap().clone();

    w.position.reserve(entity, pos);

    //velocity初期化

    let mut vel = Vector2::normalize(&Vector2 { x: 1.0, y: 0.0 });
    vel = vel.rotate(90.0);

    w.velocity.reserve(entity, vel * 0.5);

    w.draw_param.reserve(entity, burret_draw_param());
    w.group.reserve(entity, group::BULLET);
    w.collider.reserve(entity, vec![]);
    w.scale.reserve(entity, BULLET_SIZE);
}

pub fn create_aim_bullet(w: &mut World, parent_id: &EntityId, direction: &Vector2) {
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    w.parent.reserve(entity, parent_id.clone());

    //velocity初期化
    let vel = Vector2::normalize(direction);

    w.velocity.reserve(entity, vel * 0.5);

    let pos = w.position.get_unchecked(parent_id).clone();

    w.position.reserve(entity, pos + vel);

    w.draw_param.reserve(entity, burret_draw_param());
    w.group.reserve(entity, group::BULLET);
    w.collider.reserve(entity, vec![]);
    w.scale.reserve(entity, BULLET_SIZE);
}
pub fn create_bullet_8way(w: &mut World, parent_id: &EntityId) {
    let mut angle = 0.0;

    for _ in 0..8 {
        let id = w.entities.instantiate_entity();
        let entity = w.entities.get_mut(&id).unwrap();

        w.parent.reserve(entity, parent_id.clone());

        let pos = w.position.get(parent_id).unwrap().clone();

        w.position.reserve(entity, pos);

        //velocity初期化

        let mut vel = Vector2::normalize(&Vector2 { x: 1.0, y: 0.0 });
        vel = vel.rotate(angle);

        w.velocity.reserve(entity, vel * 0.5);

        w.draw_param.reserve(entity, burret_draw_param());
        w.group.reserve(entity, group::BULLET);
        w.collider.reserve(entity, vec![]);
        w.scale.reserve(entity, BULLET_SIZE);
        angle += 45.0;
    }
}

pub fn remove_out_of_bounds(w: &mut World) {
    //暫定的にコライダー持ちをすべて処理(将来的にコライダーを持ったフィールド外のオブジェクトが欲しくなるかも)
    let entities = collect_entities_from_archetype(&w, &[w.collider.id()]);
    for entity_id in entities.iter() {
        //ボールがコートの外に出たときに消滅させる
        let pos = w.position.get(entity_id).unwrap();
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

pub fn nearest_target(w: &World, self_id: &EntityId, group: &usize) -> Option<(EntityId, Vector2)> {
    let position = w.position.get(self_id).unwrap().clone();
    let mut nearest_distance = std::f64::MAX;
    let mut nearest_target_tupple: Option<(EntityId, Vector2)> = None;

    for value in w.position.items.iter() {
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

        let ref_distance = (value.item.clone() - position).sqr_magnitude();

        if nearest_distance > ref_distance {
            nearest_distance = ref_distance;
            nearest_target_tupple = Some((value.id, value.item.clone()));
        }
    }

    nearest_target_tupple
}

pub fn collect_entities_from_archetype(w: &World, values: &[ComponentId]) -> Vec<EntityId> {
    let arche = EntytyArcheType::create_archetype(values);
    w.entities.get_entities_from_archetype(&arche)
}

pub fn collect_entities_from_group(w: &World, group_id: &usize) -> Vec<EntityId> {
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
