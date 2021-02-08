use std::io;
use std::io::Write;
use std::time::Instant;
use text_engine::command::CommandHandler;
use text_engine::entity::Character;
use text_engine::entity::Inventory;
use text_engine::entity::Object;
use text_engine::parser::Parser;
use text_engine::player::Player;
use text_engine::room::{Door, Room, RoomID};

const GAME_LENGTH: u64 = 360;
const DOOR_CLOSED_LENGTH: u64 = 1;

fn get_living_room_items() -> Inventory {
    Inventory {
        objects: vec![
            Object {
                name: "watch",
                desc: "a watch",
                can_take: true,
                can_use: false,
                ..Default::default()
            },
            Object {
                name: "couch",
                desc: "a couch",
                ..Default::default()
            },
        ],
    }
}

fn get_kitchen_items() -> Inventory {
    Inventory {
        objects: vec![
            Object {
                name: "counter",
                desc: "a counter",
                inventory: vec!["kettle".into(), "tea tin".into()],
                ..Default::default()
            },
            Object {
                name: "water",
                desc: "water",
                can_take: true,
                ..Default::default()
            },
            Object {
                name: "kettle",
                desc: "a kettle",
                accepts: vec!["water".into()],
                ..Default::default()
            },
            Object {
                name: "tea tin",
                desc: "a tea tin",
                inventory: vec!["tea bag".into()],
                ..Default::default()
            },
            Object {
                name: "tea bag",
                desc: "a tea bag",
                ..Default::default()
            },
            Object {
                name: "cupboard",
                desc: "a cupboard",
                inventory: vec!["mug".into()],
                ..Default::default()
            },
            Object {
                name: "mug",
                desc: "a mug",
                accepts: vec!["tea bag".into(), "hot water".into(), "sugar cubes".into()],
                ..Default::default()
            },
        ],
    }
}

fn get_rooms() -> [Room; 4] {
    // living room 0, kitchen 1, hallway 2, cat room 3
    [
        Room {
            name: "living room",
            desc: "This is the living room.",
            doors: vec![
                Door {
                    target: RoomID(1),
                    direction: "east",
                    ..Default::default()
                },
                Door {
                    target: RoomID(2),
                    direction: "north",
                    ..Default::default()
                },
            ],
            inventory: get_living_room_items(),
            characters: vec![],
        },
        Room {
            name: "kitchen",
            desc: "This is the kitchen.",
            doors: vec![Door {
                target: RoomID(0),
                direction: "west",
                ..Default::default()
            }],
            inventory: get_kitchen_items(),
            characters: vec![],
        },
        Room {
            name: "hallway",
            desc: "This is the hallway.",
            doors: vec![
                Door {
                    target: RoomID(0),
                    direction: "south",
                    ..Default::default()
                },
                Door {
                    target: RoomID(3),
                    direction: "north",
                    is_open: false,
                    msg_on_closed: Some("A note is on the door. It reads `I'll be back at 10pm.`"),
                    ..Default::default()
                },
            ],
            inventory: Inventory::default(),
            characters: vec![],
        },
        Room {
            name: "cat room",
            desc: "This is the cat's room.",
            doors: vec![Door {
                target: RoomID(2),
                direction: "south",
                ..Default::default()
            }],
            inventory: Inventory::default(),
            characters: vec![Character {
                name: "cat",
                desc: "a cat",
                inventory: Inventory {
                    objects: vec![Object {
                        name: "sugar cubes",
                        desc: "these are sugar cubes",
                        inventory: vec![],
                        can_take: true,
                        ..Default::default()
                    }],
                },
                on_talk: "hey",
                has_interacted: false,
            }],
        },
    ]
}

fn get_trimmed_input(input: &mut String) -> &str {
    input.clear();
    io::stdin().read_line(input).unwrap();
    input.trim()
}

fn update(
    player: &mut Player,
    rooms: &mut [Room],
    player_won: &mut bool,
    player_lost: &mut bool,
    start_time: Instant,
) {
    // check whether to open the door yet
    let duration = start_time.elapsed().as_secs();
    if duration > DOOR_CLOSED_LENGTH {
        rooms[2]
            .doors
            .iter_mut()
            .find(|door| door.target == RoomID(3))
            .unwrap()
            .is_open = true;
    } else if duration >= GAME_LENGTH {
        *player_lost = true;
    }

    let mug = player.inventory.find_object("mug");
    if let Some(mug) = mug {
        // check win condition
        if mug.inventory.contains(&"brewed tea".to_owned())
            && mug.inventory.contains(&"sugar cubes".to_owned())
        {
            *player_won = true;
        }
    }
}

fn main() {
    let mut rooms = get_rooms();
    let mut input = String::new();

    let mut player_won = false;
    let mut player_lost = false;
    let start_time = Instant::now();

    let mut player = Player {
        name: "me",
        desc: "a person",
        at: RoomID(0),
        inventory: Inventory::default(),
    };

    println!("Tea Time");
    println!("============================");
    println!();

    loop {
        let here = &rooms[player.at.0];
        println!("{}\n{}", here.name, here.desc);
        loop {
            print!("\n> ");
            io::stdout().flush().unwrap();

            let trimmed_input = get_trimmed_input(&mut input);
            let parsed_input = Parser::parse(&trimmed_input);

            match parsed_input {
                Ok(command) => {
                    // println!("{:?}", command);
                    let result = CommandHandler::handle_command(
                        command,
                        &mut player,
                        &mut rooms,
                        start_time,
                    );
                    println!("{}", result.message)
                }
                Err(msg) => println!("{}", msg),
            }
            // println!("{:?}", player.get_curr_room(&rooms).inventory);
            update(
                &mut player,
                &mut rooms,
                &mut player_won,
                &mut player_lost,
                start_time,
            );
            println!("{}", player_lost);
        }
    }
}
