use rand::Rng;

use crate::{
    structs::ecs::*,
    structs::util::*,
    systems::sys_main::*,
    user_consts::{self, *},
    utils::*,
};
//暫定的に矩形コライダーのみ対応
pub struct Collider {
    id: EntityId,
    group: usize,
    grid: usize,
    offset: Vector2,
    shape: Rect,
}

pub struct Circle {
    radius: f64,
    offset: Vector2,
}
impl Circle {
    pub fn new(size: f64) -> Circle {
        Circle {
            radius: size,
            offset: Vector2::zero(),
        }
    }
}
pub struct Rect {
    pub width: f64,
    pub height: f64,
    pub rotation: f64,
    pub offset: Vector2,
}
impl Rect {
    pub fn new(width: f64, height: f64) -> Rect {
        Rect {
            width,
            height,
            rotation: 0.0,
            offset: Vector2::zero(),
        }
    }
}

pub enum Shape {
    Circle(Circle),
    Rect(Rect),
}

pub fn ball_collision(w: &mut World) {
    let entities = collect_entities_from_group(w, &group::BALL);

    let mut map: Vec<usize> = Vec::new();

    let mut balls = vec![];
    for entity_id in entities.iter() {
        //暫定的にグループ1同士のみ当たるということにする
        if w.group.get(entity_id).unwrap() == &group::BALL {
            balls.push(*entity_id);

            //グリッドでグループ分け とりあえず暫定的に4つに
            //また、とりあえず簡単のために境界付近での当たりについては一旦無視
            let pos = w.position.get(entity_id).unwrap().clone();
            if pos.x < coat_size::X as f64 / 2.0 {
                if pos.y < coat_size::Y as f64 / 2.0 {
                    map.push(0);
                } else {
                    map.push(1);
                }
            } else {
                if pos.y < coat_size::Y as f64 / 2.0 {
                    map.push(2);
                } else {
                    map.push(3);
                }
            }
        }
    }

    let mut checked_ball: &EntityId;

    //ここからマルチスレッドにしたいけどとりあえずシングルでのみ考える

    for (ball_index, ball_id) in balls.iter().enumerate() {
        let ball_pos = w.position.get_unchecked(ball_id).clone();

        let ball_scale = w.scale.get_unchecked(ball_id).clone();
        let mut hit_entities: Vec<EntityId> = vec![];

        checked_ball = ball_id;

        for (target_index, target_id) in balls.iter().enumerate() {
            if target_id == ball_id || target_id < checked_ball {
                continue;
            }
            //positionのグループが違ったらスルー
            if map[ball_index] != map[target_index] {
                continue;
            }

            let target_pos = w.position.get(target_id).unwrap();

            //let target_pos = &Vector2 { x: 2.0, y: 3.5 };

            let distance = ball_pos - *target_pos;

            //距離の２乗(処理簡略化のため)がそれぞれの半径を足した距離の２乗より小さければ衝突
            if distance.sqr_magnitude() <= (ball_scale / 2.0 + ball_scale / 2.0).powf(2.0) {
                //衝突したentityをバッファに書き込む

                let str = format!("id:{:?} がid:{:?} に衝突", ball_id.0, target_id.0);
                send_scroll_message(w, &str);

                hit_entities.push(*target_id);
            }
        }

        //最終書き込み
        if hit_entities.len() > 0 {
            w.collider_target.set(ball_id, hit_entities);
        } else {
            //前フレームに衝突していなかったらスルーし、衝突していたら空のvecで上書き
            if w.collider_target.get(ball_id).unwrap().len() > 0 {
                w.collider_target.set(ball_id, vec![]);
            }
        }
    }
}

pub fn collision(w: &mut World) {
    let mut colliders: Vec<Collider> = Vec::new();

    //Colliderを実装しているentityをgroupと一緒にタプルとして確保
    for entity in w.entities.entities.iter() {
        if let Some(value) = entity {
            if w.collider_target.get(&value.id) == None {
                continue;
            }

            if let Some(group) = w.group.get(&value.id) {
                //グリッド無視のマップをここに入れる

                let position = w.position.get_unchecked(&value.id).clone();
                let scale = w.scale.get_unchecked(&value.id).clone();
                let mut grid: usize = std::usize::MAX;

                if position.x < coat_size::X as f64 / 2.0 {
                    if position.y < coat_size::Y as f64 / 2.0 {
                        grid = 0;
                    } else {
                        grid = 1;
                    }
                } else {
                    if position.y < coat_size::Y as f64 / 2.0 {
                        grid = 2;
                    } else {
                        grid = 3;
                    }
                }
                let rect = Rect::new(scale, scale);
                colliders.push(Collider {
                    id: value.id,
                    group: group.clone(),
                    grid,
                    offset: position,
                    shape: rect,
                });
            }
        }
    }

    let mut checked_id: EntityId;

    //ここからマルチスレッドにしたいけどとりあえずシングルでのみ考える

    for collider in colliders.iter() {
        let mut hit_entities: Vec<EntityId> = vec![];

        checked_id = collider.id;

        for target in colliders.iter() {
            //ターゲットが自身であるか、もしくはすでにチェック済みであるならスルー
            if target.id == collider.id || target.id < checked_id {
                continue;
            }
            //positionのグループが違ったらスルー
            if collider.grid != target.grid {
                continue;
            }
            //超暫定的に弾同士は当たらないようにする
            if collider.group == group::BULLET && target.group == group::BULLET {
                continue;
            }

            let distance = collider.offset - target.offset;

            //距離の２乗(処理簡略化のため)がそれぞれの半径を足した距離の２乗より小さければ衝突
            if distance.sqr_magnitude() <= (collider.scale / 2.0 + target.scale / 2.0).powf(2.0) {
                //衝突したentityをバッファに書き込む

                let str = format!("id:{:?} がid:{:?} に衝突", collider.id, target.id);
                send_scroll_message(w, &str);

                hit_entities.push(target.id);
            }
        }

        //最終書き込み
        if hit_entities.len() > 0 {
            w.collider_target.set(&collider.id, hit_entities);
        } else {
            //前フレームに衝突していなかったらスルーし、衝突していたら空のvecで上書き
            if w.collider_target.get_unchecked(&collider.id).len() > 0 {
                w.collider_target.set(&collider.id, vec![]);
            }
        }
    }
}

pub fn ball_dead(w: &mut World) {
    let entities = collect_entities_from_archetype(
        &w,
        &[
            w.position.id(),
            w.velocity.id(),
            w.scale.id(),
            w.group.id(),
            w.collider_target.id(),
        ],
    );

    for entity_id in entities.iter() {
        let targets = w.collider_target.get(entity_id);

        //targetsが空のベクトルではなくNoneになっているということはすでに破棄されているので飛ばす
        if let None = targets {
            continue;
        }

        let targets = targets.unwrap().clone();

        //衝突が発生したら
        if targets.len() > 0 {
            //まず衝突相手をすべて破棄
            for target_id in targets.iter() {
                w.remove_entity(target_id);

                let str = format!("衝突対象のid:{:?}を破棄", target_id);
                send_scroll_message(w, &str);
            }

            //最後に自分を破棄
            w.remove_entity(entity_id);

            let str = format!("衝突後のid:{:?}を破棄", entity_id);
            send_scroll_message(w, &str);

            return;
        }

        //ボールがコートの外に出たときに消滅させる

        let pos = w.position.get(entity_id).unwrap();
        if pos.x > coat_size::X as f64 || pos.x < 0.0 || pos.y > coat_size::Y as f64 || pos.y < 0.0
        {
            w.remove_entity(entity_id);

            let str = format!("フィールド外に落ちたid:{:?}を破棄", entity_id);
            send_scroll_message(w, &str);
        }
    }
}
