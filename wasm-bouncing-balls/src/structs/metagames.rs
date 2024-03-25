use crate::structs::{structs_util::*, weapon::*};
use crate::systems::sys_collision::{Collider, EntityAabb};
use crate::systems::sys_draw::DrawParamater;
use crate::systems::sys_player::PlayerVars;
use crate::user_consts::MAX_COMPONENTS;
use crate::BvhNode;

use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;
use std::vec;

use super::ecs::World;

pub struct GameManager {
    pub vars: ManagerVars,

    pub world: World,
    pub state: GameState,
}
impl GameManager {
    pub fn new() -> Self {
        GameManager {
            vars: ManagerVars::new(),
            world: World::new(),
            state: GameState::Title,
        }
    }
}
pub struct ManagerVars {
    pub last_screen_click_point: Option<Vector2>,
    pub is_click_detection: bool,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub delta_time: f64,
    pub mouse_down_time: f64,
}
impl ManagerVars {
    pub fn new() -> Self {
        ManagerVars {
            last_screen_click_point: None,
            is_click_detection: false,
            canvas_width: 0,
            canvas_height: 0,
            delta_time: 0.0,
            mouse_down_time: 0.0,
        }
    }
}
