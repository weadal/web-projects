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
    pub world: World,
    pub state: GameState,
}
impl GameManager {
    pub fn new() -> Self {
        GameManager {
            world: World::new(),
            state: GameState::Title,
        }
    }
}
