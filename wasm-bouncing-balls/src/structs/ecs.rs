use crate::structs::util::*;
use crate::user_consts::MAX_COMPONENTS;

use std::collections::HashMap;

use std::ops::Deref;
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

    fn remove(&mut self, entity_id: &EntityId) {
        self.entities[entity_id.0] = None;
    }
}

pub struct CompItem<T> {
    pub id: EntityId,
    pub item: T,
}

pub struct Component<TestCompItem> {
    id: ComponentId,
    id_index_map: HashMap<EntityId, usize>,
    pub items: Vec<TestCompItem>,
}
impl<T> Component<CompItem<T>> {
    pub fn get(&self, entity_id: &EntityId) -> Option<&T> {
        let index = self.id_index_map.get(entity_id);

        match index {
            Some(value) => return Some(&self.items[*value].item),
            None => return None,
        }
    }

    pub fn get_mut(&mut self, entity_id: &EntityId) -> Option<&mut T> {
        let index = self.id_index_map.get(entity_id);

        match index {
            Some(value) => return Some(&mut self.items[*value].item),
            None => return None,
        }
    }

    pub fn get_unchecked(&self, entity_id: &EntityId) -> &T {
        let index = self.id_index_map.get(entity_id);
        &self.items[*index.unwrap()].item
    }
    pub fn get_unchecked_mut(&mut self, entity_id: &EntityId) -> &mut T {
        let index = self.id_index_map.get(entity_id);
        &mut self.items[*index.unwrap()].item
    }

    pub fn get_from_index(&self, index: usize) -> Option<&T> {
        if self.items.len() <= index {
            return None;
        }
        return Some(&self.items[index].item);
    }

    pub fn get_mut_from_index(&mut self, index: usize) -> Option<&T> {
        if self.items.len() <= index {
            return None;
        }
        return Some(&mut self.items[index].item);
    }

    pub fn set(&mut self, entity_id: &EntityId, value: T) {
        let index = self.id_index_map.get(&entity_id);
        if let Some(v) = index {
            self.items[*v].item = value;
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
            item: (T::default()),
        });

        self.items.len() - 1
    }
    fn push_with_item(&mut self, entity_id: EntityId, item: T) -> usize {
        self.items.push(CompItem {
            id: entity_id,
            item: (item),
        });

        self.items.len() - 1
    }
    pub fn reserve_default(&mut self, entity: &mut Entity)
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
    pub fn reserve(&mut self, entity: &mut Entity, item: T) {
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
    pub entities: EntityManager,
    pub is_alive: Component<CompItem<bool>>,
    pub position: Component<CompItem<Vector2>>,
    pub velocity: Component<CompItem<Vector2>>,
    pub scale: Component<CompItem<f64>>,
    pub draw_icon: Component<CompItem<&'static str>>,
    pub collider_target: Component<CompItem<Vec<EntityId>>>,
    pub group: Component<CompItem<usize>>,
    pub timer_time: Component<CompItem<f64>>,
    pub timer_alarm: Component<CompItem<Vec<f64>>>,
    pub system_message: Component<CompItem<Vec<String>>>,
    pub parent: Component<CompItem<EntityId>>,
}
impl World {
    pub fn new() -> Self {
        let mut id_iter = (0..MAX_COMPONENTS).into_iter();

        World {
            consts: WorldConsts::new(),
            entities: EntityManager { entities: vec![] },
            is_alive: Component::new(id_iter.next()),
            position: Component::new(id_iter.next()),
            velocity: Component::new(id_iter.next()),
            scale: Component::new(id_iter.next()),
            draw_icon: Component::new(id_iter.next()),
            collider_target: Component::new(id_iter.next()),
            group: Component::new(id_iter.next()),
            timer_time: Component::new(id_iter.next()),
            timer_alarm: Component::new(id_iter.next()),
            system_message: Component::new(id_iter.next()),
            parent: Component::new(id_iter.next()),
        }
    }

    fn remove_from_entity(&mut self, entity_id: &EntityId) {
        let entity = self.entities.get_mut(entity_id);
        if let None = entity {
            return;
        }

        let entity = entity.unwrap();

        self.is_alive.remove(entity);
        self.position.remove(entity);
        self.velocity.remove(entity);
        self.scale.remove(entity);
        self.draw_icon.remove(entity);
        self.collider_target.remove(entity);
        self.group.remove(entity);
        self.timer_time.remove(entity);
        self.timer_alarm.remove(entity);
        self.system_message.remove(entity);
        self.parent.remove(entity);
        //....
    }

    pub fn remove_entity(&mut self, entity_id: &EntityId) {
        self.remove_from_entity(entity_id);
        self.entities.remove(entity_id);
    }
}
pub struct WorldConsts {
    pub canvas_x: u32,
    pub canvas_y: u32,
    pub delta_time: f64,
}
impl WorldConsts {
    fn new() -> Self {
        WorldConsts {
            canvas_x: 0,
            canvas_y: 0,
            delta_time: 0.0,
        }
    }
}
