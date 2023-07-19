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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentId(pub usize);

#[derive(Clone, Debug)]
pub struct EntytyArcheType {
    pub components: Vec<ComponentId>,
}
impl EntytyArcheType {
    pub fn create_empty() -> EntytyArcheType {
        EntytyArcheType { components: vec![] }
    }

    pub fn create_archetype(values: &[ComponentId]) -> EntytyArcheType {
        let mut vec = vec![];

        for value in values {
            vec.push(value.clone());
        }
        EntytyArcheType { components: vec }
    }

    //渡されたアーキタイプに含まれるコンポーネントがすべて自身の持つコンポーネントにあるか？
    pub fn is_archetype_include(&self, archetype_ref: &EntytyArcheType) -> bool {
        //println!("archetype_ref:{:?}", archetype_ref.components);
        //println!("self_archetype:{:?}", self.components);

        let mut include_comp_count = 0;
        //渡されたアーキタイプのコンポーネントを見ていく
        for ref_comp in archetype_ref.components.iter() {
            //println!("ref_comp:{}", ref_comp);

            //自身のアーキタイプのコンポーネントを見ていく
            for own_comp in self.components.iter() {
                // println!("own_comp:{}", own_comp);

                //少なくとも１つコンポーネントが合致する
                if ref_comp == own_comp {
                    //println!("合致！");
                    include_comp_count += 1;
                    break;
                }
            }
        }

        //println!("include_comp_count:{}", include_comp_count);

        if include_comp_count == archetype_ref.components.len() {
            return true;
        }
        false
    }

    pub fn is_component_include(&self, ref_id: ComponentId) -> bool {
        for component in self.components.iter() {
            if ref_id == *component {
                return true;
            }
        }
        return false;
    }
}

#[derive(Clone)]
pub struct Entity {
    pub id: EntityId,
    pub archetype: EntytyArcheType,
}

pub struct EntityManager {
    pub entities: Vec<Option<Entity>>,
}
impl EntityManager {
    pub fn instantiate_entity(&mut self) -> EntityId {
        //破棄されたentityがあればそこに追加
        for (index, entity) in self.entities.iter().enumerate() {
            if let None = entity {
                self.entities[index] = Some(Entity {
                    id: EntityId(index),
                    archetype: EntytyArcheType::create_empty(),
                });

                return EntityId(index);
            }
        }

        //廃棄されたentityがなければ、vecの一番最後に追加

        let entity_id = self.entities.len();

        self.entities.push(Some(Entity {
            id: EntityId(entity_id),
            archetype: EntytyArcheType::create_empty(),
        }));

        EntityId(entity_id)
    }
    pub fn get_mut(&mut self, id: &EntityId) -> Option<&mut Entity> {
        self.entities[id.0].as_mut()
    }

    pub fn get_entities_from_archetype(&self, ref_archetype: &EntytyArcheType) -> Vec<EntityId> {
        let mut entities = vec![];

        //存在するentityをすべて見ていく
        for entity in self.entities.iter() {
            match entity {
                Some(value) => {
                    if value.archetype.is_archetype_include(&ref_archetype) {
                        entities.push(value.id);
                    }
                }
                None => (),
            }
        }
        entities
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn alive_entities_len(&self) -> usize {
        let mut count = 0;
        for entitiy in self.entities.iter() {
            if let Some(_) = entitiy {
                count += 1
            };
        }

        count
    }

    pub fn get_alive_entities(&self) -> Option<Vec<EntityId>> {
        let mut entities = vec![];
        for entity in self.entities.iter() {
            if let Some(value) = entity {
                entities.push(value.id);
            }
        }

        if entities.len() > 0 {
            return Some(entities);
        }

        None
    }

    pub fn has_component(&self, entity_id: &EntityId, comp_id: &ComponentId) -> bool {
        let entity = &self.entities[entity_id.0];

        match entity {
            Some(e) => return e.archetype.is_component_include(*comp_id),
            None => return false,
        }
    }

    fn remove(&mut self, entity_id: &EntityId) {
        self.entities[entity_id.0] = None;
    }
}

pub struct CompItem<T> {
    pub id: EntityId,
    pub item: Option<T>,
}

pub struct Component<CompItem> {
    id: ComponentId,
    id_index_map: HashMap<EntityId, usize>,
    pub items: Vec<CompItem>,
}
impl<T> Component<CompItem<T>> {
    pub fn get(&self, entity_id: &EntityId) -> Option<&T> {
        let index = self.id_index_map.get(entity_id);
        match index {
            Some(i) => return self.items[*i].item.as_ref(),
            None => return None,
        }
    }

    pub fn get_mut(&mut self, entity_id: &EntityId) -> Option<&mut T> {
        let index = self.id_index_map.get(entity_id);

        match index {
            Some(i) => return self.items[*i].item.as_mut(),
            None => return None,
        }
    }

    pub fn get_unchecked(&self, entity_id: &EntityId) -> &T {
        let index = self.id_index_map.get(entity_id);
        self.items[*index.unwrap()].item.as_ref().unwrap()
    }
    pub fn get_unchecked_mut(&mut self, entity_id: &EntityId) -> &mut T {
        let index = self.id_index_map.get(entity_id);
        self.items[*index.unwrap()].item.as_mut().unwrap()
    }
    pub fn take(&mut self, entity_id: &EntityId) -> Option<T> {
        let index = self.id_index_map.get(entity_id);

        match index {
            Some(i) => {
                let item = self.items[*i].item.take();
                return item;
            }
            None => return None,
        }
    }
    pub fn take_unchecked(&mut self, entity_id: &EntityId) -> T {
        let index = self.id_index_map.get(entity_id);
        let item = self.items[*index.unwrap()].item.take().unwrap();
        return item;
    }

    pub fn get_from_index(&self, index: usize) -> Option<&T> {
        if self.items.len() <= index {
            return None;
        }
        return self.items[index].item.as_ref();
    }

    pub fn get_mut_from_index(&mut self, index: usize) -> Option<&mut T> {
        if self.items.len() <= index {
            return None;
        }
        return self.items[index].item.as_mut();
    }

    pub fn set(&mut self, entity_id: &EntityId, value: Option<T>) {
        let index = self.id_index_map.get(&entity_id);
        if let Some(i) = index {
            self.items[*i].item = value;
            return;
        }

        panic!(
            "存在しないidが渡されました:{:?} index:{}",
            entity_id,
            index.unwrap()
        )
    }

    fn push_default(&mut self, entity_id: EntityId) -> usize
    where
        T: Default,
    {
        self.items.push(CompItem {
            id: entity_id,
            item: Some(T::default()),
        });

        self.items.len() - 1
    }
    fn push_with_item(&mut self, entity_id: EntityId, item: T) -> usize {
        self.items.push(CompItem {
            id: entity_id,
            item: Some(item),
        });

        self.items.len() - 1
    }
    pub fn register_default(&mut self, entity: &mut Entity)
    where
        T: Default,
    {
        //渡されたentityがすでにmapに存在しているかを確認する
        if let Some(_) = self.id_index_map.get(&entity.id) {
            panic!("渡されたentityに紐づいたitemがすでに存在します 二重に確保することはできません")
        }

        //entity_idからitemsの添字を得るHashmapに追加する
        let index = self.push_default(entity.id);
        self.id_index_map.insert(entity.id, index);

        //渡されたentityのアーキタイプに自身のidを追加する
        entity.archetype.components.push(self.id);
    }
    pub fn register(&mut self, entity: &mut Entity, item: T) {
        //渡されたentityがすでにmapに存在しているかを確認する
        if let Some(_) = self.id_index_map.get(&entity.id) {
            panic!("渡されたentityに紐づいたitemがすでに存在します 二重に確保することはできません")
        }

        //entity_idからitemsの添字を得るHashmapに追加する
        let index = self.push_with_item(entity.id, item);
        self.id_index_map.insert(entity.id, index);

        //渡されたentityのアーキタイプに自身のidを追加する
        entity.archetype.components.push(self.id);
    }

    fn remove(&mut self, entity: &Entity) {
        let index = self.id_index_map.get(&entity.id);
        if let Some(value) = index {
            self.items.remove(*value);

            let id = entity.id;
            let mut index = self.id_index_map.get(&entity.id).unwrap().clone();

            self.id_index_map.remove(&id);
            //compitemを破棄すると、そのitemより後ろにあるitemの添字がすべて前にずれる
            //そのためHashmapの参照先も全部ずれるので、破棄されたitemの添字より大きい値を持つデータを全て-1する
            self.id_index_map
                .iter_mut()
                .filter(|(_, v)| *v > &mut index)
                .for_each(|(_, v)| *v -= 1);
        }
    }

    fn new(id: Option<usize>) -> Self {
        Component {
            id: ComponentId(id.unwrap()),
            id_index_map: HashMap::new(),
            items: vec![],
        }
    }
    pub fn id(&self) -> ComponentId {
        self.id
    }
}

pub struct World {
    pub consts: WorldConsts,
    pub vars: WorldVariables,
    pub entities: EntityManager,
    pub transform: Component<CompItem<Transform>>,
    pub destination: Component<CompItem<Vec<Option<Vector2>>>>,
    pub draw_param: Component<CompItem<DrawParamater>>,
    pub collider: Component<CompItem<Vec<Collider>>>,
    pub group: Component<CompItem<Group>>,
    pub clock: Component<CompItem<Clock>>,
    pub target: Component<CompItem<Vec<Option<EntityId>>>>,
    pub weapon: Component<CompItem<Vec<Option<Weapon>>>>,
    pub player_vars: Component<CompItem<PlayerVars>>,
}
impl World {
    pub fn new() -> Self {
        let mut id_iter = (0..MAX_COMPONENTS).into_iter();

        World {
            consts: WorldConsts::new(),
            vars: WorldVariables::new(),
            entities: EntityManager { entities: vec![] },
            transform: Component::new(id_iter.next()),
            draw_param: Component::new(id_iter.next()),
            collider: Component::new(id_iter.next()),
            group: Component::new(id_iter.next()),
            clock: Component::new(id_iter.next()),
            destination: Component::new(id_iter.next()),
            target: Component::new(id_iter.next()),
            weapon: Component::new(id_iter.next()),
            player_vars: Component::new(id_iter.next()),
        }
    }

    fn remove_from_entity(&mut self, entity_id: &EntityId) {
        let entity = self.entities.get_mut(entity_id);
        if let None = entity {
            return;
        }

        let entity = entity.unwrap();

        self.transform.remove(entity);

        self.draw_param.remove(entity);
        self.collider.remove(entity);
        self.group.remove(entity);
        self.clock.remove(entity);
        self.destination.remove(entity);
        self.target.remove(entity);
        self.weapon.remove(entity);
        self.player_vars.remove(entity);
        //....
    }

    pub fn remove_entity(&mut self, entity_id: &EntityId) {
        self.remove_from_entity(entity_id);
        self.entities.remove(entity_id);
    }
}
pub struct WorldConsts {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub delta_time: f64,
}
impl WorldConsts {
    fn new() -> Self {
        WorldConsts {
            canvas_width: 0,
            canvas_height: 0,
            delta_time: 0.0,
        }
    }
}
pub struct WorldVariables {
    pub is_playing: bool,
    pub last_click_point: Option<Vector2>,
    pub is_click_detection: bool,
    pub state: GameState,
    pub bvh: Vec<Option<BvhNode>>,
}

impl WorldVariables {
    fn new() -> Self {
        WorldVariables {
            is_playing: false,
            last_click_point: None,
            is_click_detection: false,
            state: GameState::Title,
            bvh: vec![None; Group::None as usize],
        }
    }
}
