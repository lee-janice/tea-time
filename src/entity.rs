#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub name: &'static str,
    // pub desc: &'static str,
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
