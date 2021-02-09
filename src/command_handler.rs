use crate::command::Command;
use crate::command::CommandResult;
use crate::entity::Object;
use crate::game_state::GameState;
use crate::player::Player;
use crate::room::Room;
use std::time::Instant;

pub struct CommandHandler;

impl CommandHandler {
    pub fn handle_command(
        command: Command,
        player: &mut Player,
        rooms: &mut [Room],
        state: &mut GameState,
    ) -> CommandResult {
        let cmd = command.clone();
        let verb = command.verb.unwrap_or_default();
        match verb.as_str() {
            "north" | "n" | "south" | "s" | "east" | "e" | "west" | "w" => {
                CommandHandler::handle_go(&verb, player, rooms)
            }
            "examine" | "x" | "look" => CommandHandler::handle_examine(cmd, player, rooms),
            "take" | "pickup" | "get" => CommandHandler::handle_take(cmd, player, rooms),
            "inventory" | "i" | "items" => CommandHandler::handle_inventory(player),
            "put" | "place" => CommandHandler::handle_put(cmd, player, rooms, &mut state.tea_time),
            "use" => CommandHandler::handle_use(cmd, player, rooms, state.start_time),
            "talk" => CommandHandler::handle_talk(cmd, player, rooms),
            "ask" => CommandHandler::handle_ask(cmd, player, rooms),
            _ => CommandResult::didnt_understand(verb),
        }
    }

    fn handle_go(direction: &str, player: &mut Player, rooms: &mut [Room]) -> CommandResult {
        let door = player.get_curr_room(rooms).get_door(direction);
        match door {
            Some(door) => {
                if door.is_open {
                    player.go(door.target);
                    let curr_room = player.get_curr_room(rooms);
                    let room_name = curr_room.get_display_name();
                    let door_msg = door.msg_on_open.unwrap_or_default().to_owned();
                    let room_msg = curr_room.desc.to_owned();
                    let msg = if door_msg.is_empty() {
                        format!("{}\n{}", room_name, room_msg)
                    } else {
                        format!("{}\n{}\n{}", room_name, door_msg, room_msg)
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

    fn handle_examine(command: Command, player: &mut Player, rooms: &mut [Room]) -> CommandResult {
        let curr_room = player.get_curr_room_mut(rooms);
        match command.obj {
            Some(object_name) => {
                let name = object_name.as_str();
                if name == "me" || name == "myself" {
                    CommandResult {
                        message: player.desc.to_owned(),
                    }
                } else if let Some(object) = player.find_object(name) {
                    CommandResult {
                        message: object.desc.to_owned(),
                    }
                } else if let Some(object) = curr_room.find_object(name).cloned() {
                    let mut msg = object.desc.to_owned();
                    if !object.inventory.is_empty() {
                        for name in &object.inventory {
                            curr_room.find_object_mut(name).unwrap().can_take = true;
                            msg = format!(
                                "{}\nYou can now take: {}.",
                                object.desc.to_owned(),
                                object.inventory.join(", ")
                            );
                        }
                    }
                    CommandResult { message: msg }
                } else if let Some(character) = curr_room.find_character(name) {
                    CommandResult {
                        message: character.desc.to_owned(),
                    }
                } else {
                    CommandResult::cant_do_that("do".to_string())
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
                if curr_room.has(&object_name) && curr_room.find_object(&object_name).is_some() {
                    match curr_room.remove(&object_name) {
                        Some(object) => {
                            player.take_object(object);
                            for obj in &mut curr_room.inventory.objects {
                                let index = obj.inventory.iter().position(|o| *o == object_name);
                                if let Some(i) = index {
                                    obj.inventory.remove(i);
                                }
                            }
                            CommandResult {
                                message: format!("You take the {}.", object_name),
                            }
                        }
                        None => CommandResult::cant_do_that("do".to_owned()),
                    }
                } else {
                    CommandResult::cant_do_that("do".to_owned())
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

    fn handle_put(
        command: Command,
        player: &mut Player,
        rooms: &mut [Room],
        tea_time: &mut Option<Instant>,
    ) -> CommandResult {
        let cmd = command.clone();
        let curr_room = player.get_curr_room_mut(rooms);
        match command.obj {
            Some(object_name) => {
                // if the player has the object
                if player.has(object_name.as_str()) {
                    let prep = cmd.prep.unwrap_or_default();
                    match prep.as_str() {
                        // and the preposition is valid
                        "in" | "into" | "inside" => {
                            let obj_prep_name = command.obj_prep.unwrap_or_default();
                            // and the player or room has the object_prep
                            if player.has(obj_prep_name.as_str())
                                || curr_room.has(obj_prep_name.as_str())
                            {
                                let object = player.remove(&object_name).unwrap();
                                let object_prep = if player.has(obj_prep_name.as_str()) {
                                    player.find_object_mut(&obj_prep_name).unwrap()
                                } else {
                                    curr_room.find_object_mut(&obj_prep_name).unwrap()
                                };
                                if object_prep.accepts.contains(&object_name) {
                                    // put the object in the object_prep
                                    object_prep.inventory.push(object.name.to_string());
                                    // hardcoded :(
                                    if obj_prep_name == "mug"
                                        && object_prep.contains("hot water".to_string())
                                        && object_prep.contains("tea bag".to_string())
                                    {
                                        *tea_time = Some(Instant::now())
                                    }
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
                    if let Some(kettle) = curr_room.find_object_mut("kettle") {
                        if kettle.contains("water".to_owned()) {
                            let hot_water = Object {
                                name: "hot water",
                                desc: "hot water",
                                can_take: true,
                                ..Default::default()
                            };
                            kettle.inventory = vec!["hot water".into()];
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
                "watch" if player.has("watch") => {
                    let hour = 7 + (start_time.elapsed().as_secs() / 60);
                    let minute = start_time.elapsed().as_secs() % 60;
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
                    if !character.has_interacted {
                        character.has_interacted = true;
                        CommandResult {
                            message: character.on_talk.to_owned(),
                        }
                    } else {
                        CommandResult {
                            message: character.on_talk_again.to_owned(),
                        }
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
                                                "{}. The {} gives you {}.",
                                                character.on_ask, character_name, obj_prep_name
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
