use std::clone;

use rand::Rng;

use crate::{
    structs::ecs::*,
    structs::structs_util::*,
    systems::sys_main::*,
    user_consts::{self, *},
    utils::*,
    Node, *,
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
    pub targets: Option<Vec<EntityId>>,
}

impl Collider {
    pub fn new(shape: Rect, group: Group, offset: Vector2) -> Collider {
        Collider {
            shape,
            group,
            offset,
            targets: None,
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

pub fn Collision(w: &mut World, ctx: &CanvasRenderingContext2d) {
    //グループごとにvec<collisionTemp>を作成
    //それを使ってグループごとのBVHを作成しworldに格納
    create_bvh(w);

    let entitis_with_possible_contact: Rc<RefCell<Vec<(EntityId, EntityId)>>> =
        Rc::new(RefCell::new(vec![]));

    //colliderコンポーネントへ衝突対象を書き込み
    let entities = collect_entities_from_archetype(&w, &[w.collider.id()]);
    for entity_id in entities.iter() {
        let position = w.transform.get_unchecked(entity_id).position;
        //暫定的に複数コライダーには非対応
        let aabb = w.collider.get_unchecked(entity_id)[0].aabb(position);

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
                        entitis_with_possible_contact.clone();
                    let bvh_box = Box::new(n);

                    get_contact_with(&entity_aabb, &bvh_box, entities_with_possible_contact_clone);

                    w.vars.bvh[i] = Some(*bvh_box);
                }
                None => w.vars.bvh[i] = None,
            }
        }
    }

    draw_bvh(w, ctx);
    //各々の当たり判定処理は別のsystemで行う
}

fn draw_bvh(w: &mut World, ctx: &CanvasRenderingContext2d) {
    ctx.set_stroke_style(&JsValue::from_str("rgba(255.0,255.0,0.0,0.4)"));
    ctx.set_line_width(2.0);
    let node = w.vars.bvh[Group::Enemy as usize].take().unwrap();
    let node_box = Box::new(node);

    draw_bvh_inner(&ctx, &node_box);

    w.vars.bvh[Group::Enemy as usize] = Some(*node_box);
}
fn draw_bvh_inner<'a>(ctx: &'a CanvasRenderingContext2d, node: &Box<EcsNode>) {
    draw_aabb(ctx, &node.aabb);

    //リーフノードでない場合、再帰的にツリーを降下する
    if node.entitiy_aabbs.len() > 1 {
        draw_bvh_inner(ctx, node.left_child.as_ref().unwrap());

        draw_bvh_inner(ctx, node.right_child.as_ref().unwrap());
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

        //w.vars.bvh[i] = Some(create_tree(entity_aabb_refs, &ctx, false));
        let bvh = Some(create_tree(entity_aabbs, false));
        w.vars.bvh[i] = bvh;
    }
}
fn get_contact_with<'a>(
    entity_aabb: &'a EntityAabb,
    node: &Box<EcsNode>,
    entities_with_possible_contact: Rc<RefCell<Vec<(EntityId, EntityId)>>>,
) {
    //接触なし
    if !entity_aabb.aabb.is_intersects(&node.aabb) {
        return;
    }

    //接触
    if node.entitiy_aabbs.len() == 1 {
        if node.entitiy_aabbs[0].entity_id <= entity_aabb.entity_id {
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

fn create_tree(entity_aabbs: Vec<EntityAabb>, y_axis_division: bool) -> EcsNode {
    let aabbs = entity_aabbs.iter().map(|e| e.aabb).collect();

    let aabb = Aabb::from_aabbs(aabbs);

    //オブジェクト数が1つのAABBはそれ以上分類できないので決め打ちで葉要素として最終処理
    if entity_aabbs.len() == 1 {
        return EcsNode {
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

    let node = EcsNode {
        left_child,
        right_child,
        entitiy_aabbs: entity_aabbs,
        aabb,
    };

    node
}
