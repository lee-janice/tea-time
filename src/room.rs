use crate::entity::Character;
use crate::entity::Inventory;
use crate::entity::Object;

pub struct Room {
    pub name: &'static str,
    pub desc: &'static str,
    pub doors: Vec<Door>,
    pub inventory: Inventory,
    pub characters: Vec<Character>,
}

impl Room {
    pub fn get_door(&self, direction: &str) -> Option<&Door> {
        self.doors.iter().find(|door| door.direction == direction)
    }

    pub fn find_object(&self, name: &str) -> Option<&Object> {
        self.inventory.find_object(name)
    }

    pub fn find_object_mut(&mut self, name: &str) -> Option<&mut Object> {
        self.inventory.find_object_mut(name)
    }

    pub fn find_object_pos(&self, name: &str) -> Option<usize> {
        self.inventory.find_object_pos(name)
    }

    // // Need this because find_object borrows self.inventory immutably
    // pub fn find_object_clone(&self, name: &str) -> Option<Item> {
    //   let result = self.inventory.iter().find(|object| object.name == name);
    //   match result {
    //     Some(object) => Some(object.clone()),
    //     None => None,
    //   }
    // }

    pub fn give_object(&mut self, name: &str) -> Option<Object> {
        self.inventory.give_object(name)
    }

    pub fn find_character_mut(&mut self, name: &str) -> Option<&mut Character> {
        self.characters
            .iter_mut()
            .find(|character| character.name == name)
    }

    pub fn has(&self, object_name: &str) -> bool {
        self.inventory.contains(object_name)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct RoomID(pub usize);

#[derive(Debug)]
pub struct Door {
    pub target: RoomID,
    pub direction: &'static str,
    pub is_open: bool,
    pub msg_on_open: Option<&'static str>,
    pub msg_on_closed: Option<&'static str>,
}

impl Default for Door {
    fn default() -> Door {
        Door {
            target: RoomID(0),
            direction: "",
            is_open: true,
            msg_on_open: None,
            msg_on_closed: None,
        }
    }
}
