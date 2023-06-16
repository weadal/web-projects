use std::clone;

use rand::Rng;

use crate::{
    structs::ecs::*,
    structs::util::*,
    systems::sys_main::*,
    user_consts::{self, *},
    utils::*,
};
//暫定的に矩形コライダーのみ対応
#[derive(Clone)]
pub struct Collider {
    pub shape: Rect,
    pub group: usize,
    pub offset: Vector2,
    pub targets: Option<Vec<EntityId>>,
}

impl Collider {
    pub fn new(shape: Rect, group: usize, offset: Vector2) -> Collider {
        Collider {
            shape,
            group,
            offset,
            targets: None,
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
