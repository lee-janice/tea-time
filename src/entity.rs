pub struct Character {
    pub name: &'static str,
    pub desc: &'static str,
    pub inventory: Inventory,
    pub on_talk: &'static str,
    pub has_interacted: bool,
}

impl Character {
    pub fn give_object(&mut self, name: &str) -> Option<Object> {
        self.inventory.give_object(name)
    }

    pub fn find_object_mut(&mut self, name: &str) -> Option<&mut Object> {
        self.inventory.find_object_mut(name)
    }

    pub fn has(&self, object_name: &str) -> bool {
        self.inventory.contains(object_name)
    }
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub objects: Vec<Object>,
}

impl Inventory {
    pub fn find_object(&self, name: &str) -> Option<&Object> {
        self.objects.iter().find(|object| object.name == name)
    }

    pub fn find_object_mut(&mut self, name: &str) -> Option<&mut Object> {
        self.objects.iter_mut().find(|object| object.name == name)
    }

    pub fn find_object_pos(&self, name: &str) -> Option<usize> {
        self.objects.iter().position(|o| o.name == name)
    }

    pub fn give_object(&mut self, name: &str) -> Option<Object> {
        let index = self.find_object_pos(name);
        match index {
            Some(i) => Some(self.objects.remove(i)),
            _ => None,
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.objects.iter().any(|object| object.name == name)
    }

    // pub fn give_object(&mut self, index: Option<usize>) -> Option<Object> {
    //     match index {
    //         Some(i) if self.objects[i].can_take => Some(self.objects.remove(i)),
    //         _ => None,
    //     }
    // }
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory { objects: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub name: &'static str,
    pub desc: &'static str,
    pub inventory: Vec<String>,
    pub accepts: Vec<String>,
    pub can_take: bool,
    pub can_use: bool,
    pub msg_on_take: Option<String>,
    pub msg_on_use: Option<String>,
}

impl Default for Object {
    fn default() -> Self {
        Object {
            name: "",
            desc: "",
            inventory: vec![],
            accepts: vec![],
            can_take: false,
            can_use: false,
            msg_on_take: None,
            msg_on_use: None,
        }
    }
}

impl Object {
    pub fn contains(&self, name: String) -> bool {
        self.inventory.contains(&name)
    }
}
