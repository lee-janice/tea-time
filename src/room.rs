use crate::character::Character;
use crate::door::Door;
use crate::entity::Object;
use crate::inventory::Inventory;
use crate::util::get_room_name_border;

pub struct Room {
    pub name: &'static str,
    pub desc: &'static str,
    pub doors: Vec<Door>,
    pub inventory: Inventory,
    pub characters: Vec<Character>,
}

impl Room {
    pub fn get_display_name(&self) -> String {
        let border = get_room_name_border();
        format!("{}\n{}\n{}", border, self.name, border)
    }

    pub fn get_door(&self, direction: &str) -> Option<&Door> {
        self.doors.iter().find(|door| door.direction == direction)
    }

    /// -----------------------------
    /// methods for inventory/objects
    /// -----------------------------
    pub fn has(&self, object_name: &str) -> bool {
        self.inventory.contains(object_name)
    }

    pub fn find_object(&self, name: &str) -> Option<&Object> {
        self.inventory.find_object(name)
    }

    pub fn find_object_mut(&mut self, name: &str) -> Option<&mut Object> {
        self.inventory.find_object_mut(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<Object> {
        self.inventory.remove(name)
    }

    /// ----------------------
    /// methods for characters
    /// ----------------------
    pub fn find_character(&self, name: &str) -> Option<&Character> {
        self.characters
            .iter()
            .find(|character| character.name == name)
    }

    pub fn find_character_mut(&mut self, name: &str) -> Option<&mut Character> {
        self.characters
            .iter_mut()
            .find(|character| character.name == name)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct RoomID(pub usize);
