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

pub fn create_ball(w: &mut World) {
    //ボールのentityを作成し、戻り値でentityのidを得る
    let id = w.entities.instantiate_entity();
    let entity = w.entities.get_mut(&id).unwrap();

    //position初期化
    let mut rng = rand::thread_rng();
    let mut rand_x =
        rng.gen_range(ENEMY_SIZE * 2.0..w.consts.canvas_width as f64 - ENEMY_SIZE * 2.0);
    let mut rand_y =
        rng.gen_range(ENEMY_SIZE * 2.0..w.consts.canvas_height as f64 - ENEMY_SIZE * 2.0);

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

    let rect = Rect::new(ENEMY_SIZE, ENEMY_SIZE);
    let collider = Collider::new(rect, Group::Enemy, Vector2::zero());
    w.collider.register(entity, vec![collider]);

    let r = Rect::new(ENEMY_SIZE, ENEMY_SIZE);
    let draw_param = DrawParamater::new(js_color_rgba(255.0, 255.0, 255.0, 1.0), Shape::Rect(r));

    w.draw_param.register(entity, draw_param);
    w.group.register(entity, Group::Enemy);
    w.clock.register(entity, Clock::new());
}

pub fn ball_reflection(w: &mut World) {
    let screen_left = w.vars.camera_position.x;
    let screen_right = w.consts.canvas_width as f64 + w.vars.camera_position.x;
    let screen_top = w.vars.camera_position.y;
    let screen_bottom = w.consts.canvas_height as f64 + w.vars.camera_position.y;

    let entities = collect_entities_from_group(w, &Group::Enemy);
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

pub fn ball_move(w: &mut World) {
    let entities = collect_entities_from_group(w, &Group::Enemy);
    for entity_id in entities.iter() {
        let mut transform = w.transform.get(entity_id).unwrap().clone();

        let pos = transform.position;

        let players = collect_entities_from_group(w, &Group::Player);
        let mut closest_player_pos = Vector2 {
            x: f64::INFINITY,
            y: f64::INFINITY,
        };
        for player in players.iter() {
            let player_pos = w.transform.get(player).unwrap().position;
            if player_pos.magnitude() < closest_player_pos.magnitude() {
                closest_player_pos = player_pos;
            }
        }

        transform.velocity = (closest_player_pos - pos).normalize() * 50.0;

        w.transform.set(entity_id, Some(transform));
    }
}

pub fn enemy_attack(w: &mut World) {}
