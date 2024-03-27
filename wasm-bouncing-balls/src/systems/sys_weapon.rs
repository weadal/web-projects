use rand::Rng;

use crate::{
    structs::ecs::*,
    structs::{structs_util::*, weapon::*},
    user_consts::*,
    utils::*,
    *,
};

use super::{sys_collision::*, sys_draw::*, sys_main::*};

pub fn create_weapons(w: &mut World, parent_id: EntityId) -> EntityId {
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    w.weapon
        .register(entity, vec![Some(Weapon::new(parent_id))]);

    id
}

pub fn time_increase(w: &mut World) {
    let entities = collect_entities_from_archetype(&w, &[w.weapon.id()]);

    for entity_id in entities.iter() {
        let weapons = w.weapon.get_unchecked_mut(entity_id);
        for weapon in weapons {
            if let Some(wp) = weapon {
                wp.elapsed_time += w.consts.delta_time;
            }
        }
    }
}

pub fn fire(w: &mut World) {
    let entities = collect_entities_from_archetype(&w, &[w.weapon.id()]);
    let mut bullet_params: Vec<BulletParamater> = vec![];
    for entity_id in entities.iter() {
        let weapons = w.weapon.get_mut(entity_id).unwrap();
        for weapon in weapons.iter_mut() {
            if let Some(wp) = weapon {
                if !wp.is_active {
                    continue;
                }

                if wp.elapsed_time >= (60.0 / wp.rate) * 1000.0 {
                    bullet_params.push(wp.bullet_param.clone());
                    wp.elapsed_time = 0.0;
                }
            }
        }
    }
    for bullet_param in bullet_params.iter() {
        create_bullet(w, bullet_param);
    }
}

fn burret_draw_param() -> DrawParamater {
    let c = Circle::new(BULLET_SIZE);
    DrawParamater {
        color: js_color_rgba(255.0, 255.0, 255.0, 1.0),
        shape: Shape::Circle(c),
    }
}
pub fn create_bullet(w: &mut World, bullet_param: &BulletParamater) -> EntityId {
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    let parent_transform = w.transform.get_unchecked_mut(&bullet_param.parent_id);

    let mut transform = parent_transform.clone();
    transform.set_parent(parent_transform);

    transform.id = id;

    let mut vel = Vector2::normalize(&Vector2 { x: 1.0, y: 0.0 });

    let angle = random_f64(0.0, 360.0);

    vel = vel.rotate(angle);

    transform.velocity = vel * 100.0;
    transform.scale = BULLET_SIZE;

    w.transform.register(entity, transform);

    let my_group = if *w.group.get(&bullet_param.parent_id).unwrap() == Group::Enemy {
        Group::EnemyBullet
    } else {
        Group::PlayerBullet
    };

    w.draw_param.register(entity, burret_draw_param());
    w.group.register(entity, my_group);

    let rect = Rect::new(BULLET_SIZE, BULLET_SIZE);
    let collider = Collider::new(rect, my_group, Vector2::zero());
    w.collider.register(entity, vec![collider]);
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
    w.group.register(entity, Group::PlayerBullet);
    w.collider.register(entity, vec![]);
}
