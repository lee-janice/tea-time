use crate::entity::Object;
use crate::inventory::Inventory;
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
    /// -------------------------
    /// methods for room/location
    /// -------------------------
    pub fn get_curr_room<'a, 'b>(&'a self, rooms: &'b [Room]) -> &'b Room {
        &rooms[self.at.0]
    }

    pub fn get_curr_room_mut<'a, 'b>(&'a self, rooms: &'b mut [Room]) -> &'b mut Room {
        &mut rooms[self.at.0]
    }

    pub fn go(&mut self, room_id: RoomID) {
        self.at = room_id;
    }

    pub fn list_objects(&self) -> String {
        self.inventory
            .objects
            .iter()
            .map(|o| o.name)
            .collect::<Vec<&'static str>>()
            .join("\n\t- ")
    }

    /// -----------------------------
    /// methods for inventory/objects
    /// -----------------------------
    pub fn has(&self, object_name: &str) -> bool {
        self.inventory.contains(object_name)
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

    pub fn remove(&mut self, object_name: &str) -> Option<Object> {
        self.inventory.remove(object_name)
    }
}
