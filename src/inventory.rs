use crate::entity::Object;

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

    pub fn contains(&self, name: &str) -> bool {
        self.objects.iter().any(|object| object.name == name)
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn remove(&mut self, name: &str) -> Option<Object> {
        let index = self.find_object_pos(name);
        match index {
            Some(i) if self.objects[i].can_take => Some(self.objects.remove(i)),
            _ => None,
        }
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory { objects: vec![] }
    }
}
