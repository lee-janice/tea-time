use crate::entity::Object;
use crate::inventory::Inventory;

pub struct Character {
    pub name: &'static str,
    pub desc: &'static str,
    pub inventory: Inventory,
    pub on_talk: &'static str,
    pub on_talk_again: &'static str,
    pub on_ask: &'static str,
    pub has_interacted: bool,
}

impl Character {
    pub fn has(&self, object_name: &str) -> bool {
        self.inventory.contains(object_name)
    }

    pub fn find_object_mut(&mut self, name: &str) -> Option<&mut Object> {
        self.inventory.find_object_mut(name)
    }

    pub fn give_object(&mut self, name: &str) -> Option<Object> {
        self.inventory.remove(name)
    }
}
