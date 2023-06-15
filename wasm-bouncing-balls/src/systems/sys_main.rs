use rand::Rng;

use crate::{
    structs::ecs::*,
    structs::util::*,
    user_consts::{self, *},
};

pub fn create_ball(w: &mut World) {
    //ボールのentityを作成し、戻り値でentityのidを得る
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    //position初期化
    let mut rng = rand::thread_rng();
    let mut rand_x = rng.gen_range(2.0..w.consts.canvas_x as f64 - 2.0);
    let mut rand_y = rng.gen_range(2.0..w.consts.canvas_y as f64 - 2.0);

    let pos = Vector2 {
        x: rand_x as f64,
        y: rand_y as f64,
    };

    let pos2 = Vector2 {
        x: coat_size::X as f64 / 2.0,
        y: coat_size::Y as f64 / 2.0,
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

    w.draw_icon.reserve(entity, icon::BALL);
    w.group.reserve(entity, group::BALL);
    w.collider_target.reserve(entity, vec![]);
    w.scale.reserve(entity, BALL_SIZE);
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

        if pos.x <= 1.0 || pos.x >= w.consts.canvas_x as f64 - 1.0 {
            vel.x = -vel.x;
        }

        if pos.y <= 1.0 || pos.y >= w.consts.canvas_y as f64 - 1.0 {
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

pub fn send_scroll_message(w: &mut World, str: &str) {
    let entities = collect_entities_from_archetype(&w, &[w.velocity.id(), w.system_message.id()]);

    for entity_id in entities.iter() {
        let mut buffer = w.system_message.get(entity_id).unwrap().clone();

        buffer.push(String::from(str));
        if buffer.len() > MAX_SCROLL_MESSAGE {
            buffer.remove(0);
        }
        w.system_message.set(entity_id, buffer);
    }
}

pub fn create_scroll_message(w: &mut World) {
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    w.system_message.reserve_default(entity);
    w.velocity.reserve_default(entity);
}

pub fn create_static_message(w: &mut World) {
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    w.system_message.reserve_default(entity);
    w.position.reserve_default(entity);
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
pub fn update_static_message(w: &mut World) {
    let entities = collect_entities_from_archetype(&w, &[w.position.id(), w.system_message.id()]);

    let mut strvec: Vec<String> = vec![String::from(""); MAX_STATIC_MESSAGE];

    strvec[0] = format!("entities.len:{}", w.entities.len().to_string());
    strvec[1] = format!(
        "alive_entities:{}",
        w.entities.alive_entities_len().to_string()
    );

    //メッセージentityは一つしか存在しないので決め打ちしてしまう
    w.system_message.set(&entities[0], strvec);
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

    w.draw_icon.reserve(entity, icon::BULLET);
    w.group.reserve(entity, group::BULLET);
    w.collider_target.reserve(entity, vec![]);
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

    w.draw_icon.reserve(entity, icon::BULLET);
    w.group.reserve(entity, group::BULLET);
    w.collider_target.reserve(entity, vec![]);
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

        w.draw_icon.reserve(entity, icon::BULLET);
        w.group.reserve(entity, group::BULLET);
        w.collider_target.reserve(entity, vec![]);
        w.scale.reserve(entity, BULLET_SIZE);
        angle += 45.0;
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
