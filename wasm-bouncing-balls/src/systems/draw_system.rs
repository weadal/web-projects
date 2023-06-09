use std::sync::mpsc::Sender;

use crate::{
    draw_old::{DrawAddress, DrawMap},
    structs::ecs::*,
};

use super::sys_main;

pub fn create_draw_map(w: &mut World, tx: &mut Sender<DrawMap>) {
    let arche = EntytyArcheType::create_archetype(&[w.position.id(), w.draw_icon.id()]);
    let entities = w.entities.get_entities_from_archetype(&arche);

    let mut drawmap: DrawMap = DrawMap::new();

    for entity in entities.iter() {
        let draw_pos = DrawAddress::from_position(w.position.get(entity).unwrap());

        //option型の返り値を持つball_posがnoneでない場合、以下の処理をする

        if let Some(addr) = draw_pos {
            drawmap.string_map[addr.y as usize][addr.x as usize] =
                String::from(*w.draw_icon.get(entity).unwrap());
        }
    }

    //scroll_messageを呼ぶ
    let entities =
        sys_main::collect_entities_from_archetype(&w, &[w.velocity.id(), w.system_message.id()]);

    //scroll_messageのentityは一つしか無いはずなので決め打ち
    if w.system_message.get(&entities[0]).unwrap().len() > 0 {
        for i in 0..w.system_message.get(&entities[0]).unwrap().len() {
            drawmap.scroll_message[i] = w.system_message.get(&entities[0]).unwrap()[i].clone();
        }
    }

    //static_messageを呼ぶ
    let entities =
        sys_main::collect_entities_from_archetype(&w, &[w.position.id(), w.system_message.id()]);
    //static_messageのentityは一つしか無いはずなので決め打ち
    if w.system_message.get(&entities[0]).unwrap().len() > 0 {
        for i in 0..w.system_message.get(&entities[0]).unwrap().len() {
            drawmap.static_message[i] = w.system_message.get(&entities[0]).unwrap()[i].clone();
        }
    }
    tx.send(drawmap).unwrap();
}
