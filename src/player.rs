use crate::command::CommandResult;
use crate::entity::Inventory;
use crate::entity::Object;
use crate::room::Room;
use crate::room::RoomID;

#[derive(Debug)]
pub struct Player {
    pub name: &'static str,
    pub desc: &'static str,
    pub at: RoomID,
    pub inventory: Inventory,
}

impl Player {
    pub fn get_curr_room<'a, 'b>(&'a self, rooms: &'b [Room]) -> &'b Room {
        &rooms[self.at.0]
    }

    pub fn get_curr_room_mut<'a, 'b>(&'a self, rooms: &'b mut [Room]) -> &'b mut Room {
        &mut rooms[self.at.0]
    }

    pub fn go(&mut self, direction: &str, rooms: &[Room]) -> CommandResult {
        let door = self.get_curr_room(rooms).get_door(direction);
        match door {
            Some(door) => {
                if door.is_open {
                    self.at = door.target;
                    let door_msg = door.msg_on_open.unwrap_or_default().to_owned();
                    let room_msg = self.get_curr_room(rooms).desc.to_owned();
                    let msg = if door_msg.is_empty() {
                        room_msg
                    } else {
                        [door_msg, room_msg].join("\n")
                    };
                    CommandResult { message: msg }
                } else {
                    CommandResult {
                        message: door.msg_on_closed.unwrap_or_default().to_owned(),
                    }
                }
            }
            None => CommandResult {
                message: "You can't go that way.".to_owned(),
            },
        }
    }

    pub fn list_objects(&self) -> String {
        self.inventory
            .objects
            .iter()
            .map(|o| o.name)
            .collect::<Vec<&'static str>>()
            .join("\n\t- ")
    }

    pub fn find_object(&self, object_name: &str) -> Option<&Object> {
        self.inventory.find_object(object_name)
    }

    pub fn find_object_mut(&mut self, object_name: &str) -> Option<&mut Object> {
        self.inventory.find_object_mut(object_name)
    }

    pub fn take_object(&mut self, object: Object) {
        self.inventory.add(object);
    }

    pub fn has(&self, object_name: &str) -> bool {
        self.inventory.contains(object_name)
    }
}
