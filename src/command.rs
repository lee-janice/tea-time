use crate::entity::Object;
use crate::player::Player;
use crate::room::Room;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Command {
    pub verb: Option<String>,
    pub obj: Option<String>,
    pub prep: Option<String>,
    pub obj_prep: Option<String>,
}

pub struct CommandResult {
    pub message: String,
}

impl CommandResult {
    pub fn didnt_understand(word: String) -> Self {
        CommandResult {
            message: format!("I don't know what {} means.", word),
        }
    }

    pub fn no_object(name: String) -> Self {
        CommandResult {
            message: format!("There is no {} here.", name),
        }
    }

    pub fn cant_do_that(verb: String) -> Self {
        CommandResult {
            message: format!("You can't {} that.", verb),
        }
    }
    pub fn doesnt_have_that(character_name: String) -> Self {
        CommandResult {
            message: format!("The {} doesn't have that.", character_name),
        }
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct CommandHandler;

impl CommandHandler {
    pub fn handle_command(
        command: Command,
        player: &mut Player,
        rooms: &mut [Room],
        start_time: Instant,
    ) -> CommandResult {
        let cmd = command.clone();
        let verb = command.verb.unwrap_or_default();
        match verb.as_str() {
            "north" | "n" | "south" | "s" | "east" | "e" | "west" | "w" => {
                player.go(verb.as_str(), rooms)
            }
            "examine" | "x" | "inspect" => CommandHandler::handle_examine(cmd, player, rooms),
            "take" | "pickup" | "get" => CommandHandler::handle_take(cmd, player, rooms),
            "inventory" | "i" | "items" => CommandHandler::handle_inventory(player),
            "put" | "place" => CommandHandler::handle_put(cmd, player, rooms),
            "use" => CommandHandler::handle_use(cmd, player, rooms, start_time),
            "talk" => CommandHandler::handle_talk(cmd, player, rooms),
            "ask" => CommandHandler::handle_ask(cmd, player, rooms),
            _ => CommandResult::didnt_understand(verb),
        }
    }

    fn handle_examine(command: Command, player: &mut Player, rooms: &mut [Room]) -> CommandResult {
        let curr_room = player.get_curr_room_mut(rooms);
        match command.obj {
            Some(object_name) => {
                let name = object_name.as_str();
                if let Some(object) = player.find_object(name) {
                    CommandResult {
                        message: object.desc.to_owned(),
                    }
                } else if let Some(object) = curr_room.find_object(name).cloned() {
                    if !object.inventory.is_empty() {
                        for name in &object.inventory {
                            print!("{}", name);
                            curr_room.find_object_mut(name).unwrap().can_take = true;
                        }
                    }
                    CommandResult {
                        message: object.desc.to_owned(),
                    }
                } else {
                    CommandResult::no_object(name.to_owned())
                }
            }
            None => CommandResult {
                message: player.get_curr_room(rooms).desc.to_owned(),
            },
        }
    }

    fn handle_take(command: Command, player: &mut Player, rooms: &mut [Room]) -> CommandResult {
        let curr_room = player.get_curr_room_mut(rooms);
        match command.obj {
            Some(object_name) => {
                let object = curr_room.give_object(&object_name);
                match object {
                    Some(object) if object.can_take => {
                        player.take_object(object);
                        // for obj in
                        CommandResult {
                            message: format!("You take the {}.", object_name),
                        }
                    }
                    Some(_) => CommandResult::cant_do_that("take".to_owned()),
                    None => CommandResult::no_object(object_name),
                }
            }
            None => CommandResult {
                message: "You can't take nothing!".to_string(),
            },
        }
    }

    fn handle_inventory(player: &mut Player) -> CommandResult {
        let objects_str = player.list_objects();
        if objects_str.is_empty() {
            CommandResult {
                message: "Your pockets are empty.".to_string(),
            }
        } else {
            CommandResult {
                message: format!(
                    "Here are the contents of your pockets:\n\t- {}",
                    objects_str
                ),
            }
        }
    }

    fn handle_put(command: Command, player: &mut Player, rooms: &mut [Room]) -> CommandResult {
        let cmd = command.clone();
        let curr_room = player.get_curr_room_mut(rooms);
        match command.obj {
            Some(object_name) => {
                let object_name = object_name.as_str();
                // if the player has the object
                if player.find_object(object_name).is_some() {
                    let prep = cmd.prep.unwrap_or_default();
                    match prep.as_str() {
                        // and the preposition is valid
                        "in" | "into" => {
                            let obj_prep_name = command.obj_prep.unwrap_or_default();
                            // and the player has the object_prep
                            if let Some(object_prep) =
                                player.find_object_mut(obj_prep_name.as_str())
                            {
                                // and the object_prep can accept the object
                                if object_prep.accepts.contains(&object_name.to_owned()) {
                                    // put the object in the object_prep
                                    object_prep.inventory.push(object_name.to_owned());
                                    CommandResult {
                                        message: format!(
                                            "You put {} into {}.",
                                            object_name, obj_prep_name
                                        ),
                                    }
                                } else {
                                    CommandResult::cant_do_that("do".to_owned())
                                }
                            // and the object_prep is in the room, put object in object prep
                            } else if let Some(obj_prep) = curr_room.find_object_mut(&obj_prep_name)
                            {
                                // and the object_prep can accept the object
                                if obj_prep.accepts.contains(&object_name.to_owned()) {
                                    // put the object in the object_prep
                                    obj_prep.inventory.push(object_name.to_owned());
                                    CommandResult {
                                        message: format!(
                                            "You put {} into {}.",
                                            object_name, obj_prep_name
                                        ),
                                    }
                                } else {
                                    CommandResult::cant_do_that("do".to_owned())
                                }
                            } else {
                                CommandResult::cant_do_that("do".to_owned())
                            }
                        }
                        _ => CommandResult::didnt_understand(prep.to_owned()),
                    }
                } else {
                    CommandResult {
                        message: format!("You don't have {}.", object_name),
                    }
                }
            }
            None => CommandResult {
                message: "You can't put nothing!".to_string(),
            },
        }
    }

    /// Hardcoded :(
    fn handle_use(
        command: Command,
        player: &mut Player,
        rooms: &mut [Room],
        start_time: Instant,
    ) -> CommandResult {
        let curr_room = player.get_curr_room_mut(rooms);
        match command.obj {
            Some(object_name) => match object_name.as_str() {
                "kettle" => {
                    if let Some(object) = curr_room.find_object_mut("kettle") {
                        if object.contains("water".to_owned()) {
                            let hot_water = Object {
                                name: "hot water",
                                desc: "hot water",
                                can_take: true,
                                ..Default::default()
                            };
                            object.inventory = vec!["hot water".into()];
                            curr_room.inventory.add(hot_water);
                            CommandResult {
                                message: "You turn on the kettle. There is now hot water inside the kettle.".to_owned()
                            }
                        } else {
                            CommandResult {
                                message: "You might need to put water into the kettle first."
                                    .to_owned(),
                            }
                        }
                    } else {
                        CommandResult::no_object("kettle".to_owned())
                    }
                }
                "watch" => {
                    let hour = 7 + (start_time.elapsed().as_secs() / 60);
                    let minute = start_time.elapsed().as_secs() / 60;
                    CommandResult {
                        message: format!(
                            "You glance at your watch. It reads {}:{:02}pm.",
                            hour, minute
                        ),
                    }
                }
                _ => CommandResult::cant_do_that("use".to_owned()),
            },
            _ => CommandResult {
                message: "Use what?".to_string(),
            },
        }
    }

    fn handle_talk(command: Command, player: &mut Player, rooms: &mut [Room]) -> CommandResult {
        match command.obj {
            Some(object_name) => {
                if let Some(character) = player
                    .get_curr_room_mut(rooms)
                    .find_character_mut(&object_name)
                {
                    CommandResult {
                        message: character.on_talk.to_owned(),
                    }
                } else {
                    CommandResult::no_object(object_name)
                }
            }
            None => CommandResult {
                message: "Talk to what?".to_string(),
            },
        }
    }

    fn handle_ask(command: Command, player: &mut Player, rooms: &mut [Room]) -> CommandResult {
        let cmd = command.clone();
        match command.obj {
            Some(character_name) => {
                if let Some(character) = player
                    .get_curr_room_mut(rooms)
                    .find_character_mut(&character_name)
                {
                    let prep = cmd.prep.unwrap_or_default();
                    match prep.as_str() {
                        "for" => {
                            let obj_prep_name = command.obj_prep.unwrap_or_default();
                            // and the character has the object_prep
                            if character.has(&obj_prep_name) {
                                match character.give_object(obj_prep_name.as_str()) {
                                    Some(object) if object.can_take => {
                                        player.take_object(object);
                                        CommandResult {
                                            message: format!(
                                                "The {} gives you {}.",
                                                character_name, obj_prep_name
                                            ),
                                        }
                                    }
                                    Some(_) => CommandResult {
                                        message: format!(
                                            "The {} can't give you that.",
                                            character_name
                                        ),
                                    },
                                    None => CommandResult::doesnt_have_that(character_name),
                                }
                            } else {
                                CommandResult::doesnt_have_that(character_name)
                            }
                        }
                        _ => CommandResult::didnt_understand(prep.to_owned()),
                    }
                } else {
                    CommandResult::no_object(character_name)
                }
            }
            None => CommandResult {
                message: "Ask who?".to_string(),
            },
        }
    }
}
