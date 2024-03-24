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
    pub last_screen_click_point: Option<Vector2>,

    pub world: World,
    pub state: GameState,
}
impl GameManager {
    pub fn new() -> Self {
        GameManager {
            last_screen_click_point: None,
            world: World::new(),
            state: GameState::Title,
        }
    }
}
