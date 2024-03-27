use std::{clone, fmt::format};

use rand::Rng;

use crate::{
    structs::ecs::*,
    structs::structs_util::*,
    systems::sys_main::*,
    user_consts::{self, *},
    utils::*,
    *,
};
use web_sys::{
    console, CanvasRenderingContext2d, DomRect, HtmlButtonElement, HtmlCanvasElement,
    HtmlInputElement, HtmlParagraphElement, MouseEvent, Performance,
};

//暫定的に矩形コライダーのみ対応
#[derive(Clone)]
pub struct Collider {
    pub shape: Rect,
    pub group: Group,
    pub offset: Vector2,
    pub targets: Vec<EntityId>,
    pub target_infos: Vec<CollisionInfo>,
    pub targets_temp: Vec<EntityId>,
    pub targets_enter: Vec<EntityId>,
    pub targets_left: Vec<EntityId>,
}

impl Collider {
    pub fn new(shape: Rect, group: Group, offset: Vector2) -> Collider {
        Collider {
            shape,
            group,
            offset,
            targets: vec![],
            target_infos: vec![],
            targets_temp: vec![],
            targets_enter: vec![],
            targets_left: vec![],
        }
    }
    pub fn aabb(&self, pos: Vector2) -> Aabb {
        Aabb {
            x_max: pos.x + (self.shape.width / 2.0),
            x_min: pos.x - (self.shape.width / 2.0),
            y_max: pos.y + (self.shape.height / 2.0),
            y_min: pos.y - (self.shape.height / 2.0),
        }
    }
    pub fn local_aabb(&self) -> Aabb {
        Aabb {
            x_max: self.shape.width / 2.0,
            x_min: -(self.shape.width / 2.0),
            y_max: self.shape.height / 2.0,
            y_min: -(self.shape.height / 2.0),
        }
    }

    pub fn add_target(&mut self, target: &EntityId) {
        self.targets_temp.push(target.clone());
    }

    pub fn targets_submit(&mut self) {
        self.targets_enter.clear();
        self.targets_left.clear();

        if self.targets.len() == 0 {
            self.targets = self.targets_temp.clone();
            self.targets_enter = self.targets_temp.clone();
            self.targets_temp.clear();
            return;
        }

        let mut next_targets: Vec<EntityId> = vec![];
        let mut next_target_infos: Vec<CollisionInfo> = vec![];

        for t in self.targets_temp.iter() {
            match self.targets.binary_search(t) {
                Ok(index) => {
                    next_targets.push(*t);
                    self.targets.remove(index);
                }
                Err(_) => {
                    next_targets.push(*t);

                    self.targets_enter.push(*t);
                }
            }
        }

        self.targets_left = self.targets.clone();
        self.targets = next_targets;
        self.targets_temp.clear();
    }
}

#[derive(Clone, Debug)]
pub struct Circle {
    pub radius: f64,
    pub offset: Vector2,
}
impl Circle {
    pub fn new(size: f64) -> Circle {
        Circle {
            radius: size,
            offset: Vector2::zero(),
        }
    }
}
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Shape {
    Circle(Circle),
    Rect(Rect),
}
impl Shape {
    pub fn local_aabb(&self) -> Aabb {
        match self {
            Shape::Circle(c) => Aabb::from_circle(0.0, 0.0, c.radius),
            //暫定的にRectのRotationを考慮せずにAABBを作成する(回転させたくなったら各頂点でAABBを作ることになるはず)
            Shape::Rect(r) => Aabb {
                x_max: r.width / 2.0,
                x_min: -r.width / 2.0,
                y_max: r.height / 2.0,
                y_min: -r.height / 2.0,
            },
        }
    }
}
#[derive(Clone)]
pub struct EntityAabb {
    pub entity_id: EntityId,
    pub position: Vector2,
    pub aabb: Aabb,
}

#[derive(Clone, Debug, Copy)]
pub struct CollisionInfo {
    pub target_id: EntityId,
    pub target_group: Group,
    pub target_aabb: Aabb,
    pub point: Vector2,
    pub direction: Direction,
}

pub fn collision(w: &mut World, ctx: &CanvasRenderingContext2d) {
    //グループごとにvec<collisionTemp>を作成
    //それを使ってグループごとのBVHを作成しworldに格納
    create_bvh(w);

    let entities_with_possible_contact: Rc<RefCell<Vec<(EntityId, EntityId)>>> =
        Rc::new(RefCell::new(vec![]));

    //colliderコンポーネントへ衝突対象を書き込み
    let entities = collect_entities_from_archetype(&w, &[w.collider.id()]);

    for entity_id in entities.iter() {
        let position = w.transform.get_unchecked(entity_id).position;

        //暫定的に複数コライダーには非対応
        let col = &mut w.collider.get_unchecked_mut(entity_id)[0];
        let aabb = col.aabb(position);

        let entity_aabb = EntityAabb {
            entity_id: *entity_id,
            position,
            aabb,
        };

        //すべてのグループのBVHに対して衝突問い合わせをする
        for i in 0..MAX_GROUP {
            //unwrap()による二重参照を回避するためにOption.take()で所有権をぶんどる
            let node = w.vars.bvh[i].take();
            match node {
                Some(n) => {
                    let entities_with_possible_contact_clone =
                        entities_with_possible_contact.clone();
                    let bvh_box = Box::new(n);

                    get_contact_with(&entity_aabb, &bvh_box, entities_with_possible_contact_clone);

                    w.vars.bvh[i] = Some(*bvh_box);
                }
                None => w.vars.bvh[i] = None,
            }
        }

        //colliderを拾ってきたタイミングで先にinfoを初期化しておく
        col.target_infos.clear();
    }

    //狭域当たり判定　頑張れば上の方の処理とまとめてもうちょい参照の回数減らせそうだけど暫定的に冗長性持たせとく
    for pair in entities_with_possible_contact.borrow().iter() {
        let collision_info = narrow_collision_check(w, &pair.1, &pair.0);

        match collision_info {
            Ok(info) => {
                w.collider.get_unchecked_mut(&pair.1)[0]
                    .target_infos
                    .push(info);
                w.collider.get_unchecked_mut(&pair.1)[0].add_target(&pair.0);
            }
            Err(str) => (),
        }
    }

    for i in entities {
        let a = w.collider.get_unchecked_mut(&i);

        a[0].targets_submit();
    }

    //log(&format!("{:?}", entities_with_possible_contact.borrow()));

    //draw_bvh(w, ctx);
    //各々の当たり判定処理は別のsystemで行う
}

pub fn physics_collision_solve_add(w: &mut World) {
    let entities = collect_entities_from_archetype(&w, &[w.collider.id()]);

    for entity_id in entities.iter() {
        let entity_colliders = w.collider.take(entity_id).unwrap();
        let mut entity_transform = w.transform.take(entity_id).unwrap();
        let targets = entity_colliders[0].targets.clone();

        for (index, target_id) in targets.iter().enumerate() {
            // log(&format!(
            //     "{:?}",
            //     entity_colliders[0].target_infos[index].direction
            // ));

            if target_id <= entity_id {
                continue;
            }

            let mut target_transform = w.transform.take(target_id).unwrap();

            match entity_colliders[0].target_infos[index].direction {
                Direction::North => {
                    entity_transform.velocity.y += 60.0;
                    target_transform.velocity.y -= 60.0;
                }
                Direction::East => {
                    entity_transform.velocity.x -= 60.0;
                    target_transform.velocity.x += 60.0;
                }
                Direction::South => {
                    entity_transform.velocity.y -= 60.0;
                    target_transform.velocity.y += 60.0;
                }
                Direction::West => {
                    entity_transform.velocity.x += 60.0;
                    target_transform.velocity.x -= 60.0;
                }
            }

            w.transform.set(target_id, Some(target_transform));
        }

        w.collider.set(entity_id, Some(entity_colliders));
        w.transform.set(entity_id, Some(entity_transform));
    }
}

pub fn physics_collision_solve_add_simple(w: &mut World) {
    let entities = collect_entities_from_archetype(&w, &[w.collider.id()]);

    for entity_id in entities.iter() {
        let entity_colliders = w.collider.take(entity_id).unwrap();
        let mut entity_transform = w.transform.take(entity_id).unwrap();
        let targets = entity_colliders[0].targets.clone();

        for (index, target_id) in targets.iter().enumerate() {
            if target_id <= entity_id {
                continue;
            }

            let mut target_transform = w.transform.take(target_id).unwrap();

            let direction = (target_transform.position - entity_transform.position).normalize();
            target_transform.velocity = target_transform.velocity + direction * 60.0;
            entity_transform.velocity = entity_transform.velocity - direction * 60.0;

            w.transform.set(target_id, Some(target_transform));
        }

        w.collider.set(entity_id, Some(entity_colliders));
        w.transform.set(entity_id, Some(entity_transform));
    }
}

fn narrow_collision_check(
    w: &World,
    entity: &EntityId,
    target: &EntityId,
) -> Result<CollisionInfo, String> {
    //接触しないグループ同士だったら早期リターン
    let entity_group = w.group.get(entity).unwrap();
    let target_group = w.group.get(target).unwrap();
    if !entity_group.is_possible_contact_by_group(*target_group) {
        let message = format!(
            "接触なし entity:{:?}({:?}), target:{:?}({:?})",
            entity, entity_group, target, target_group
        );
        return Err(message);
    }

    //暫定的に1つ目のコライダーだけを処理
    let entity_collider = w.collider.get(&entity).unwrap()[0].clone();
    let entity_transform = w.transform.get(&entity).unwrap().clone();
    let entity_aabb = entity_collider.aabb(entity_transform.position);
    let target_collider = w.collider.get(&target).unwrap()[0].clone();
    let target_transform = w.transform.get_unchecked(&target).clone();
    let target_aabb = target_collider.aabb(target_transform.position);

    let mut collision_info = CollisionInfo {
        target_id: *target,
        target_group: target_collider.group,
        target_aabb,
        point: Vector2::zero(),
        direction: Direction::North,
    };

    let mut is_possible_axis: (bool, bool) = (false, false); //(x,y)

    let mut diffs: Vec<i32> = vec![i32::MAX, i32::MAX, i32::MAX, i32::MAX];

    //それぞれの辺が接触する可能性があるかをチェックし、可能性があるならフラグ立てて辺同士の距離を取得　後で比較する必要があるのでi32にキャスト(f64だと比較できない)　１ピクセル以下の誤差は無視して大丈夫だと思われる
    if entity_aabb.y_min <= target_aabb.y_max && entity_aabb.y_min >= target_aabb.y_min {
        is_possible_axis.1 = true;
        let diff_n = (target_aabb.y_max - entity_aabb.y_min) as i32;
        diffs[Direction::North as usize] = diff_n;
    }

    if entity_aabb.x_max <= target_aabb.x_max && entity_aabb.x_max >= target_aabb.x_min {
        is_possible_axis.0 = true;
        let diff_e = (entity_aabb.x_max - target_aabb.x_min) as i32;
        diffs[Direction::East as usize] = diff_e;
    }

    if entity_aabb.y_max <= target_aabb.y_max && entity_aabb.y_max >= target_aabb.y_min {
        is_possible_axis.1 = true;
        let diff_s = (entity_aabb.y_max - target_aabb.y_min) as i32;
        diffs[Direction::South as usize] = diff_s;
    }
    if entity_aabb.x_min <= target_aabb.x_max && entity_aabb.x_min >= target_aabb.x_min {
        is_possible_axis.0 = true;
        let diff_w = (target_aabb.x_max - entity_aabb.x_min) as i32;
        diffs[Direction::West as usize] = diff_w;
    }

    //仮組み　自分より相手のcolliderが小さい場合に、中に入り込んだときの処理
    if entity_aabb.y_min < target_aabb.y_min && entity_aabb.y_max > target_aabb.y_max {
        is_possible_axis.1 = true;

        let diff_n = (target_aabb.y_max - entity_aabb.y_min) as i32;
        diffs[Direction::North as usize] = diff_n;

        let diff_s = (entity_aabb.y_max - target_aabb.y_min) as i32;
        diffs[Direction::South as usize] = diff_s;
    }

    if entity_aabb.x_min < target_aabb.x_min && entity_aabb.x_max > target_aabb.x_max {
        is_possible_axis.0 = true;

        let diff_e = (entity_aabb.x_max - target_aabb.x_min) as i32;
        diffs[Direction::East as usize] = diff_e;
        let diff_w = (target_aabb.x_max - entity_aabb.x_min) as i32;
        diffs[Direction::West as usize] = diff_w;
    }

    //log(&format!("diffs:{:?}", diffs));

    //x軸とy軸両方が接触可能性がある場合接触
    if is_possible_axis.0 && is_possible_axis.1 {
        //diffsが一番小さい方向が接触している
        let min_diff = diffs.iter().min().unwrap();
        let index = diffs.iter().position(|x| x == min_diff).unwrap();

        match index {
            0 => {
                collision_info.direction = Direction::North;
                collision_info.point.x =
                    (entity_transform.position.x + target_transform.position.x) / 2.0;
                collision_info.point.y = target_aabb.y_max;

                return Ok(collision_info);
            }
            1 => {
                collision_info.direction = Direction::East;
                collision_info.point.x = target_aabb.x_min;
                collision_info.point.y =
                    (entity_transform.position.y + target_transform.position.y) / 2.0;

                return Ok(collision_info);
            }
            2 => {
                collision_info.direction = Direction::South;
                collision_info.point.x =
                    (entity_transform.position.x + target_transform.position.x) / 2.0;
                collision_info.point.y = target_aabb.y_min;

                return Ok(collision_info);
            }
            3 => {
                collision_info.direction = Direction::West;
                collision_info.point.x = target_aabb.x_max;
                collision_info.point.y =
                    (entity_transform.position.y + target_transform.position.y) / 2.0;

                return Ok(collision_info);
            }
            _ => (),
        }
    }

    let message = format!(
        "接触なし entity:{:?}({:?}), target:{:?}({:?})",
        entity, entity_group, target, target_group
    );
    Err(message)
}

fn draw_bvh(w: &mut World, ctx: &CanvasRenderingContext2d) {
    for i in 0..MAX_GROUP {
        let node = w.vars.bvh[i].take();
        match node {
            Some(n) => {
                let group = Group::get_from_index(i);

                match group {
                    Group::Enemy => {
                        ctx.set_stroke_style(&JsValue::from_str("rgba(255.0,255.0,0.0,0.4)"));
                        ctx.set_line_width(2.0);
                    }
                    Group::PlayerBullet => {
                        ctx.set_stroke_style(&JsValue::from_str("rgba(255.0,0.0,255.0,0.4)"));
                        ctx.set_line_width(2.0);
                    }
                    Group::Building => {
                        ctx.set_stroke_style(&JsValue::from_str("rgba(0.0,255.0,0.0,0.4)"));
                        ctx.set_line_width(2.0);
                    }
                    _ => (),
                }

                let node_box = Box::new(n);
                draw_bvh_inner(&ctx, &node_box, w.vars.camera_position);
                w.vars.bvh[i] = Some(*node_box);
            }
            None => w.vars.bvh[i] = None,
        }
    }
}
fn draw_bvh_inner<'a>(ctx: &'a CanvasRenderingContext2d, node: &Box<BvhNode>, camera_pos: Vector2) {
    sys_draw::draw_aabb(ctx, &node.aabb, camera_pos);

    //リーフノードでない場合、再帰的にツリーを降下する
    if node.entitiy_aabbs.len() > 1 {
        draw_bvh_inner(ctx, node.left_child.as_ref().unwrap(), camera_pos);

        draw_bvh_inner(ctx, node.right_child.as_ref().unwrap(), camera_pos);
    }
}

fn create_bvh(w: &mut World) {
    for i in 0..MAX_GROUP {
        let group = Group::get_from_index(i);

        if !group.is_need_bvh() {
            continue;
        }

        let entities = collect_entities_from_group(w, &group);

        if entities.len() == 0 {
            w.vars.bvh[i] = None;
            continue;
        }
        let mut entity_aabbs: Vec<EntityAabb> = vec![];

        for entity_id in entities.iter() {
            if !w.entities.has_component(entity_id, &w.collider.id()) {
                continue;
            }

            let position = w.transform.get_unchecked(entity_id).position;
            //暫定的に複数コライダーには非対応
            let aabb = w.collider.get_unchecked(entity_id)[0].aabb(position);

            let entity_aabb = EntityAabb {
                entity_id: *entity_id,
                position,
                aabb,
            };

            entity_aabbs.push(entity_aabb);
        }

        let bvh = Some(create_tree(entity_aabbs, false));
        w.vars.bvh[i] = bvh;
    }
}

fn get_contact_with<'a>(
    entity_aabb: &'a EntityAabb,
    node: &Box<BvhNode>,
    entities_with_possible_contact: Rc<RefCell<Vec<(EntityId, EntityId)>>>,
) {
    //接触なし
    if !entity_aabb.aabb.is_intersects(&node.aabb) {
        return;
    }

    //接触
    if node.entitiy_aabbs.len() == 1 {
        //自身との当たり判定を無視する
        if node.entitiy_aabbs[0].entity_id == entity_aabb.entity_id {
            return;
        }

        entities_with_possible_contact
            .borrow_mut()
            .push((entity_aabb.entity_id, node.entitiy_aabbs[0].entity_id));

        return;
    }
    //リーフノードでない場合、再帰的にツリーを降下する
    else if node.entitiy_aabbs.len() > 1 {
        get_contact_with(
            entity_aabb,
            node.left_child.as_ref().unwrap(),
            entities_with_possible_contact.clone(),
        );

        get_contact_with(
            entity_aabb,
            node.right_child.as_ref().unwrap(),
            entities_with_possible_contact,
        );
    }
}

fn create_tree(entity_aabbs: Vec<EntityAabb>, y_axis_division: bool) -> BvhNode {
    let aabbs = entity_aabbs.iter().map(|e| e.aabb).collect();

    let aabb = Aabb::from_aabbs(aabbs);

    //オブジェクト数が1つのAABBはそれ以上分類できないので決め打ちで葉要素として最終処理
    if entity_aabbs.len() == 1 {
        return BvhNode {
            left_child: None,
            right_child: None,
            entitiy_aabbs: entity_aabbs,
            aabb,
        };
    }

    //中点をAABBから取る関係上自身の軸サイズと自身が所属するAABBの軸サイズが一致すると右にも左にも分類できないオブジェクトが発生する
    //例えば小さいオブジェクトが大きいオブジェクトの影に隠れる(同一y軸にはいる)形になると、大きいオブジェクトのmax_x,min_xがAABB全体のmax_x,min_xになってしまう
    //そうしたときに間違って両方を同じサイドのchildに入れてしまうと無限ループが発生する(再帰呼び出しした先でも同じサイドのchildに入れられる)
    //丸め誤差対策のために中心から+-0.5の範囲をセンターに入れてしまう 近接領域の当たりで多少の誤差が発生するけど1ピクセルより小さい領域での話なので事実上誤差は無いものとできるはず
    let mut left_entities: Vec<EntityAabb> = vec![];
    let mut right_entities: Vec<EntityAabb> = vec![];
    let mut center_entities: Vec<EntityAabb> = vec![];

    //フラグで分割する軸を変更
    if y_axis_division {
        let parent_center_y = (aabb.y_max + aabb.y_min) / 2.0;

        for entity_aabb in entity_aabbs.iter() {
            if entity_aabb.position.y < parent_center_y + 0.5
                && entity_aabb.position.y > parent_center_y - 0.5
            {
                center_entities.push(entity_aabb.clone());
            } else if entity_aabb.position.y < parent_center_y {
                left_entities.push(entity_aabb.clone());
            } else if entity_aabb.position.y > parent_center_y {
                right_entities.push(entity_aabb.clone());
            } else {
                center_entities.push(entity_aabb.clone());
            }
        }
    } else {
        let parent_center_x = (aabb.x_max + aabb.x_min) / 2.0;
        for ball in entity_aabbs.iter() {
            if ball.position.x < parent_center_x + 0.5 && ball.position.x > parent_center_x - 0.5 {
                center_entities.push(ball.clone());
            } else if ball.position.x < parent_center_x {
                left_entities.push(ball.clone());
            } else if ball.position.x > parent_center_x {
                right_entities.push(ball.clone());
            } else {
                center_entities.push(ball.clone());
            }
        }
    }

    //分類できないオブジェクトは、左右のchildを見て少ない方に入れることでオブジェクト数が2つだけのAABBになった場合のループを回避する
    for ball in center_entities {
        if left_entities.len() <= right_entities.len() {
            left_entities.push(ball);
        } else {
            right_entities.push(ball);
        }
    }

    let mut left_child = None;
    let mut right_child = None;

    if left_entities.len() > 0 {
        //次回の分割方向は今回とは別の軸を使う(!y_axis_division)
        left_child = Some(Box::new(create_tree(left_entities, !y_axis_division)));
    }
    if right_entities.len() > 0 {
        right_child = Some(Box::new(create_tree(right_entities, !y_axis_division)));
    }

    let node = BvhNode {
        left_child,
        right_child,
        entitiy_aabbs: entity_aabbs,
        aabb,
    };

    node
}

pub fn get_contact_with_group(
    w: &mut World,
    entity_aabb: EntityAabb,
    group: Group,
) -> Option<Vec<EntityId>> {
    let mut contact_entities: Vec<EntityId> = vec![];
    let entitis_with_possible_contact: Rc<RefCell<Vec<(EntityId, EntityId)>>> =
        Rc::new(RefCell::new(vec![]));

    //unwrap()による二重参照を回避するためにOption.take()で所有権をぶんどる
    let node = w.vars.bvh[group as usize].take();
    match node {
        Some(n) => {
            let entities_with_possible_contact_clone = entitis_with_possible_contact.clone();
            let bvh_box = Box::new(n);

            get_contact_with(&entity_aabb, &bvh_box, entities_with_possible_contact_clone);

            w.vars.bvh[group as usize] = Some(*bvh_box);
        }
        None => w.vars.bvh[group as usize] = None,
    }

    if entitis_with_possible_contact.borrow().len() > 0 {
        for i in entitis_with_possible_contact.borrow().iter() {
            contact_entities.push(i.1);
        }
        return Some(contact_entities);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::structs::structs_util::Group;
    use crate::systems::sys_collision;

    #[test]
    #[ignore = "reason"]
    fn target_submit_test() {
        let rect = Rect::new(100.0, 100.0);
        let mut collider = Collider::new(rect, Group::Building, Vector2::zero());

        collider.add_target(&EntityId(1));
        collider.add_target(&EntityId(3));
        collider.add_target(&EntityId(5));
        collider.add_target(&EntityId(7));

        collider.targets_submit();

        println!("temp{:?}", collider.targets_temp);
        println!("now{:?}", collider.targets);
        println!("enter{:?}", collider.targets_enter);
        println!("left{:?}", collider.targets_left);

        collider.add_target(&EntityId(2));
        collider.add_target(&EntityId(3));
        collider.add_target(&EntityId(4));
        collider.add_target(&EntityId(5));

        collider.targets_submit();

        println!("temp{:?}", collider.targets_temp);
        println!("now{:?}", collider.targets);
        println!("enter{:?}", collider.targets_enter);
        println!("left{:?}", collider.targets_left);
    }

    pub fn test_collision(w: &mut World) {
        //グループごとにvec<collisionTemp>を作成
        //それを使ってグループごとのBVHを作成しworldに格納
        create_bvh(w);

        let entities_with_possible_contact: Rc<RefCell<Vec<(EntityId, EntityId)>>> =
            Rc::new(RefCell::new(vec![]));

        //colliderコンポーネントへ衝突対象を書き込み
        let entities = collect_entities_from_archetype(&w, &[w.collider.id()]);

        for entity_id in entities.iter() {
            let position = w.transform.get_unchecked(entity_id).position;

            //暫定的に複数コライダーには非対応
            let col = &mut w.collider.get_unchecked_mut(entity_id)[0];
            let aabb = col.aabb(position);

            let entity_aabb = EntityAabb {
                entity_id: *entity_id,
                position,
                aabb,
            };

            //すべてのグループのBVHに対して衝突問い合わせをする
            for i in 0..MAX_GROUP {
                //unwrap()による二重参照を回避するためにOption.take()で所有権をぶんどる
                let node = w.vars.bvh[i].take();
                match node {
                    Some(n) => {
                        let entities_with_possible_contact_clone =
                            entities_with_possible_contact.clone();
                        let bvh_box = Box::new(n);

                        get_contact_with(
                            &entity_aabb,
                            &bvh_box,
                            entities_with_possible_contact_clone,
                        );

                        w.vars.bvh[i] = Some(*bvh_box);
                    }
                    None => w.vars.bvh[i] = None,
                }
            }

            //colliderを拾ってきたタイミングで先にinfoを初期化しておく
            col.target_infos.clear();
        }

        //狭域当たり判定　頑張れば上の方の処理とまとめてもうちょい参照の回数減らせそうだけど暫定的に冗長性持たせとく
        for pair in entities_with_possible_contact.borrow().iter() {
            let collision_info = narrow_collision_check(w, &pair.1, &pair.0);

            match collision_info {
                Ok(info) => {
                    w.collider.get_unchecked_mut(&pair.1)[0]
                        .target_infos
                        .push(info);
                    w.collider.get_unchecked_mut(&pair.1)[0].add_target(&pair.0);
                }
                Err(str) => (),
            }
        }

        for i in entities {
            let a = w.collider.get_unchecked_mut(&i);

            a[0].targets_submit();
        }

        //log(&format!("{:?}", entities_with_possible_contact.borrow()));
        println!("{:?}", entities_with_possible_contact.borrow());

        //draw_bvh(w, ctx);
        //各々の当たり判定処理は別のsystemで行う
    }

    pub fn test_create_ball_random(w: &mut World) {
        //ボールのentityを作成し、戻り値でentityのidを得る
        let id = w.entities.instantiate_entity();
        let entity = w.entities.get_mut(&id).unwrap();

        //position初期化
        let mut rng = rand::thread_rng();
        let mut rand_x =
            rng.gen_range(BALL_SIZE * 2.0..w.consts.canvas_width as f64 - BALL_SIZE * 2.0);
        let mut rand_y =
            rng.gen_range(BALL_SIZE * 2.0..w.consts.canvas_height as f64 - BALL_SIZE * 2.0);

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
        let collider = Collider::new(rect, Group::Enemy, Vector2::zero());
        w.collider.register(entity, vec![collider]);

        w.group.register(entity, Group::Enemy);
        w.clock.register(entity, Clock::new());
    }

    pub fn test_create_static_ball_pos(w: &mut World, pos: &Vector2) {
        let id = w.entities.instantiate_entity();
        let entity = w.entities.get_mut(&id).unwrap();

        let pos = Vector2 {
            x: pos.x as f64,
            y: pos.y as f64,
        };

        let transform = Transform {
            id,
            position: pos,
            scale: 1.0,
            velocity: Vector2::zero(),
            parent: None,
            children: None,
        };

        w.transform.register(entity, transform);

        let rect = Rect::new(BALL_SIZE, BALL_SIZE);
        let collider = Collider::new(rect, Group::Enemy, Vector2::zero());
        w.collider.register(entity, vec![collider]);

        w.group.register(entity, Group::Enemy);
        w.clock.register(entity, Clock::new());
        pub fn test_create_static_ball_pos(w: &mut World, pos: &Vector2) {
            let id = w.entities.instantiate_entity();
            let entity = w.entities.get_mut(&id).unwrap();

            let pos = Vector2 {
                x: pos.x as f64,
                y: pos.y as f64,
            };

            let transform = Transform {
                id,
                position: pos,
                scale: 1.0,
                velocity: Vector2::zero(),
                parent: None,
                children: None,
            };

            w.transform.register(entity, transform);

            let rect = Rect::new(BALL_SIZE, BALL_SIZE);
            let collider = Collider::new(rect, Group::Enemy, Vector2::zero());
            w.collider.register(entity, vec![collider]);

            w.group.register(entity, Group::Enemy);
            w.clock.register(entity, Clock::new());
        }
    }

    pub fn test_create_static_bullet_pos(w: &mut World, pos: &Vector2) {
        let id = w.entities.instantiate_entity();
        let entity = w.entities.get_mut(&id).unwrap();

        let pos = Vector2 {
            x: pos.x as f64,
            y: pos.y as f64,
        };

        let transform = Transform {
            id,
            position: pos,
            scale: 1.0,
            velocity: Vector2::zero(),
            parent: None,
            children: None,
        };

        w.transform.register(entity, transform);

        let rect = Rect::new(BALL_SIZE, BALL_SIZE);
        let collider = Collider::new(rect, Group::PlayerBullet, Vector2::zero());
        w.collider.register(entity, vec![collider]);

        w.group.register(entity, Group::PlayerBullet);
        w.clock.register(entity, Clock::new());
        pub fn test_create_static_ball_pos(w: &mut World, pos: &Vector2) {
            let id = w.entities.instantiate_entity();
            let entity = w.entities.get_mut(&id).unwrap();

            let pos = Vector2 {
                x: pos.x as f64,
                y: pos.y as f64,
            };

            let transform = Transform {
                id,
                position: pos,
                scale: 1.0,
                velocity: Vector2::zero(),
                parent: None,
                children: None,
            };

            w.transform.register(entity, transform);

            let rect = Rect::new(BALL_SIZE, BALL_SIZE);
            let collider = Collider::new(rect, Group::Enemy, Vector2::zero());
            w.collider.register(entity, vec![collider]);

            w.group.register(entity, Group::Enemy);
            w.clock.register(entity, Clock::new());
        }
    }
    #[test]
    fn world_collision_test() {
        let mut w = crate::structs::ecs::World::new();
        w.consts.canvas_width = 200;
        w.consts.canvas_height = 200;

        // for _ in 0..10 {
        //     test_create_ball_random(&mut w);
        // }
        let v1 = Vector2 { x: -10.0, y: -10.0 };
        let v2 = Vector2 { x: 10.0, y: 10.0 };

        test_create_static_bullet_pos(&mut w, &v1);
        test_create_static_ball_pos(&mut w, &v2);

        println!("Frame:1");
        test_collision(&mut w);
        println!("================================");

        test_remove_out_of_bounds(&mut w);

        let v2 = Vector2 { x: -5.0, y: -5.0 };
        w.transform.get_unchecked_mut(&EntityId(1)).position = v2;

        println!("================================");
        println!("Frame:2");
        test_collision(&mut w);
    }

    pub fn test_remove_out_of_bounds(w: &mut World) {
        println!("canvas_width:{:?}", w.consts.canvas_width as f64);

        //暫定的にコライダー持ちをすべて処理(将来的にコライダーを持ったフィールド外のオブジェクトが欲しくなるかも)
        let entities = collect_entities_from_archetype(&w, &[w.collider.id()]);
        for entity_id in entities.iter() {
            //ボールがコートの外に出たときに消滅させる
            let pos = w.transform.get(entity_id).unwrap().position;
            let aabb = w.collider.get(entity_id).unwrap()[0].local_aabb();
            println!("{:?},aabb{:?}", entity_id, aabb);

            //とりあえず描画のAABBが画面外に出たら破棄する(コライダーのAABBは描画のAABBより小さいものとする)
            if pos.x > w.consts.canvas_width as f64 + aabb.x_max
                || pos.x < aabb.x_min
                || pos.y > w.consts.canvas_height as f64 + aabb.y_max
                || pos.y < aabb.y_min
            {
                w.remove_entity(entity_id);
                println!("領域外に落ちたentity(id:{:?})を破棄", entity_id);
            }
        }
    }
}
