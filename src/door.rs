use crate::room::RoomID;

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
