use rand::Rng;

use crate::{
    structs::ecs::*,
    structs::structs_util,
    systems::{sys_draw, *},
    user_consts::*,
    utils::*,
    *,
};

pub struct Weapon {
    pub parent_id: EntityId,
    pub is_active: bool,
    pub name: String,
    pub damage: i32,
    pub range: u32,
    pub rate: f64,
    pub elapsed_time: f64,
    pub bullet_param: BulletParamater,
}
impl Weapon {
    pub fn new(parent_id: EntityId) -> Self {
        let mut weapon = Weapon {
            parent_id,
            is_active: false,
            name: String::from("noname"),
            damage: 100,
            range: 100,
            rate: 600.0,
            elapsed_time: 0.0,
            bullet_param: BulletParamater::new(parent_id),
        };

        weapon.bullet_param.damage = weapon.damage;
        weapon.bullet_param.scale = BULLET_SIZE;
        weapon.bullet_param.velocity = 100.0;

        log(&format!("weapon_create_for:{:?}", parent_id));

        weapon
    }
}

#[derive(Clone)]
pub struct BulletParamater {
    pub parent_id: EntityId,
    pub damage: i32,
    pub scale: f64,
    pub direction: Vector2,
    pub velocity: f64,
}
impl BulletParamater {
    pub fn new(parent_id: EntityId) -> Self {
        BulletParamater {
            parent_id,
            damage: 0,
            scale: 1.0,
            direction: Vector2::zero(),
            velocity: 1.0,
        }
    }
}
