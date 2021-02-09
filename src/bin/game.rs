use rodio::Source;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Write;
use std::time::Instant;
use std::{thread, time};
use text_engine::character::Character;
use text_engine::command_handler::CommandHandler;
use text_engine::door::Door;
use text_engine::entity::Object;
use text_engine::game_state::GameState;
use text_engine::inventory::Inventory;
use text_engine::parser::Parser;
use text_engine::player::Player;
use text_engine::room::{Room, RoomID};
use text_engine::util::{get_room_name_border, get_trimmed_input};

const GAME_LENGTH: u64 = 300;
const DOOR_CLOSED_LENGTH: u64 = 180;
const TEA_BREW_LENGTH: u64 = 60;

fn get_living_room_items() -> Inventory {
    Inventory {
        objects: vec![
            Object {
                name: "couch",
                desc: "A fluffy light grey couch. It's so comfortable that you sometimes unknowingly doze off on its cushions.",
                ..Default::default()
            },
            Object {
                name: "coffee table",
                desc: "The amber surface of the table is stained with faint traces of old coffee and tea mugs.",
                inventory: vec!["watch".into()],
                ..Default::default()
            },
            Object {
                name: "watch",
                desc: "A simple analog watch with a thin gold band. You take comfort in the fact that if you ever need to know the time, you can USE the watch.",
                can_take: false,
                can_use: true,
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
                desc: "Your friendly philodendron sits idly on the countertop, keeping the remnants of this morning's half-eaten breakfast company. A kettle and a tea tin rest on the other side of the sink.",
                inventory: vec!["water".into()],
                ..Default::default()
            },
            Object {
                name: "kettle",
                desc: "Your trusty electric kettle. Sees USE almost every morning, as well as some unfortunate nights.",
                accepts: vec!["water".into()],
                ..Default::default()
            },
            Object {
                name: "tea tin",
                desc: "A delicate purple-hued tin box with a vaguely English air. The label reads `Harney & Son's Earl Grey Tea Sachets`. It also tells you that this is special tea and must be brewed for 60 minutes.",
                inventory: vec!["tea bag".into()],
                can_take: false,
                ..Default::default()
            },
            Object {
                name: "tea bag",
                desc: "A silk tea bag with dark leaves inside. The blueberry maple aroma of the tea comforts you.",
                ..Default::default()
            },
            Object {
                name: "water",
                desc: "Water, the source of life! Straight from the Brita.",
                ..Default::default()
            },
            Object {
                name: "cupboard",
                desc: "A white-framed cupboard. You can see your growing mug collection through the glass panes.",
                inventory: vec!["mug".into()],
                ..Default::default()
            },
            Object {
                name: "mug",
                desc: "Your favorite mug. It fits snugly into your hand. A small outline of a rabbit is painted on the side.",
                accepts: vec!["tea bag".into(), "hot water".into(), "sugar".into()],
                can_take: false,
                ..Default::default()
            },
        ],
    }
}

fn get_rooms() -> [Room; 4] {
    // living room 0, kitchen 1, hallway 2, cat room 3
    [
        Room {
            name: "Living Room",
            desc: "Your small but cozy living room. A light grey couch is nestled into the far corner, and a coffee table sits comfortably at its feet. The front door of your apartment lies to the north, and the kitchen door lies to the east.",
            doors: vec![
                Door {
                    target: RoomID(1),
                    direction: "east",
                    ..Default::default()
                },
                Door {
                    target: RoomID(2),
                    direction: "north",
                    msg_on_open: Some("You step into the hallway."),
                    ..Default::default()
                },
            ],
            inventory: get_living_room_items(),
            characters: vec![],
        },
        Room {
            name: "Kitchen",
            desc:
                "A full moon glow illuminates the room from the window above the sink. Beside the window is a cupboard full of kitchenware. Various items lay on the kitchen counter. The door to the living room lies to the west.",
            doors: vec![Door {
                target: RoomID(0),
                direction: "west",
                ..Default::default()
            }],
            inventory: get_kitchen_items(),
            characters: vec![],
        },
        Room {
            name: "Hallway",
            desc: "The forest green walls of the corridor are decorated with black and white photos, eclectic paintings, and old 70s movie posters. It smells a bit musty. Your front door is to the south, and across the hall is the door to Unit 11.",
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
                    msg_on_open: Some("You knock hesitantly. As soon as your hand makes contact with the door, it slowly creaks open."),
                    msg_on_closed: Some("A note is on the door. It reads `I'll be back at 11:00pm.`"),
                },
            ],
            inventory: Inventory::default(),
            characters: vec![],
        },
        Room {
            name: "Unit 11",
            desc: "The living room in Unit 11 is dimly lit, the only source of light being the glow of a few candles. A small cat is curled on the leather couch, and raises its head to look at you. Its owner is nowhere to be seen.",
            doors: vec![Door {
                target: RoomID(2),
                direction: "south",
                msg_on_open: Some("You step back into the hallway, and the door shuts softly behind you."),
                ..Default::default()
            }],
            inventory: Inventory::default(),
            characters: vec![Character {
                name: "cat",
                desc: "A medium-haired calico cat. It blinks slowly in your direction. You feel a bit silly, but you have the urge to talk to it.",
                inventory: Inventory {
                    objects: vec![Object {
                        name: "sugar",
                        desc: "Small delicate sugar cubes. Each individual granule seems to shimmer and strangely reflect the light.",
                        inventory: vec![],
                        can_take: true,
                        ..Default::default()
                    }],
                },
                on_talk: "You ask the cat if it can talk. It stares at you for a while, and just as you were about to give up, you hear it speak. `Hi, I suppose you're here for some sugar?`",
                on_talk_again: "The cat seems to be preoccupied with trying to catch its own tail. You think it's best not to bother it.",
                on_ask: "The cat thinks for a moment. `I was saving this sugar for a special moment, but I guess this is as good as any.` The cat takes out some sugar cubes. `Good luck with your tea!`",
                has_interacted: false,
            }],
        },
    ]
}

fn update(player: &mut Player, rooms: &mut [Room], state: &mut GameState) {
    // check time-based events
    let duration = state.start_time.elapsed().as_secs();
    if duration > DOOR_CLOSED_LENGTH {
        rooms[2]
            .doors
            .iter_mut()
            .find(|door| door.target == RoomID(3))
            .unwrap()
            .is_open = true;
    }
    if duration >= GAME_LENGTH {
        state.player_lost = true;
    }

    let mug = player.find_object_mut("mug");
    if let Some(mug) = mug {
        // check brew time on tea
        if let Some(time) = state.tea_time {
            if time.elapsed().as_secs() > TEA_BREW_LENGTH
                && mug.inventory.contains(&"hot water".to_string())
                && mug.inventory.contains(&"tea bag".to_string())
            {
                for object_name in &["hot water", "tea bag"] {
                    let index = mug.inventory.iter().position(|o| o == object_name);
                    if let Some(i) = index {
                        mug.inventory.remove(i);
                    }
                    mug.inventory.push("brewed tea".to_string());
                    mug.name = "brewed tea";
                }
                println!("Your tea is brewed, but you would really like some sugar. Maybe your neighbor in Unit 11 has some...")
            }
        }
    }

    let brewed_tea = player.find_object_mut("brewed tea");
    if let Some(tea) = brewed_tea {
        // check win condition
        if tea.inventory.contains(&"sugar".to_owned()) {
            state.player_won = true;
        }
    }
}

fn main() {
    // play audio with rodio
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = File::open("music.mp3").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let result = stream_handle.play_raw(source.convert_samples());
    if result.is_err() {
        println!("Music can't be played right now, sorry!")
    }

    // get world
    let mut rooms = get_rooms();
    let mut player = Player {
        name: "me",
        desc: "a person",
        at: RoomID(0),
        inventory: Inventory::default(),
    };

    // set game state
    let mut state = GameState {
        player_won: false,
        player_lost: false,
        start_time: Instant::now(),
        tea_time: None,
    };

    let mut input = String::new();

    // start game
    println!("============================");
    println!("Tea Time");
    println!("============================");
    thread::sleep(time::Duration::from_secs(3));
    println!();
    println!("You slowly open your eyes.");
    println!();
    thread::sleep(time::Duration::from_secs(5));
    println!();
    println!("You've fallen asleep on your couch. You blink groggily as your eyes adjust to the darkness. Wait... it's dark outside? What time is it...");
    println!();
    thread::sleep(time::Duration::from_secs(5));
    println!();
    println!("You drag yourself up and turn on the lights. You're still really sleepy. A warm cup of tea sounds like the best thing in the world right now. You only have until 12am...");
    println!();
    thread::sleep(time::Duration::from_secs(5));

    'game: loop {
        let here = &rooms[player.at.0];
        let border = get_room_name_border();
        println!("{}\n{}\n{}\n{}", border, here.name, border, here.desc);
        loop {
            print!("\n> ");
            io::stdout().flush().unwrap();

            let trimmed_input = get_trimmed_input(&mut input);
            let parsed_input = Parser::parse(&trimmed_input);

            match parsed_input {
                Ok(command) => {
                    let result = CommandHandler::handle_command(
                        command,
                        &mut player,
                        &mut rooms,
                        &mut state,
                    );
                    println!("{}", result.message)
                }
                Err(msg) => println!("{}", msg),
            }

            // update game state and events
            update(&mut player, &mut rooms, &mut state);

            // handle win/lose conditions
            if state.player_won {
                thread::sleep(time::Duration::from_secs(3));
                println!();
                println!("You look down at your tea and watch the sugar slowly disappear. You walk to your couch and sit down, waiting for it to cool down a bit.");
                println!();
                thread::sleep(time::Duration::from_secs(5));
                println!();
                println!("You take a sip of your tea -- it's perfectly bittersweet and fills you with warmth. You glance at your watch. It reads 12:00am.");
                println!();
                thread::sleep(time::Duration::from_secs(5));
                println!();
                println!("You close your eyes and listen to the gentle rainfall. Not long after, you drift off into sleep.");
                println!();
                thread::sleep(time::Duration::from_secs(5));
                println!("============================");
                println!("THE END");
                println!("============================");
                break 'game;
            } else if state.player_lost {
                thread::sleep(time::Duration::from_secs(3));
                println!();
                println!("You get the sudden urge to stop in your tracks. Far away, a gong starts to ring.");
                println!();
                thread::sleep(time::Duration::from_secs(5));
                println!();
                println!("You glance at your watch. It reads 11:59pm. With each tick of the second hand, the gong sounds closer and closer.");
                println!();
                thread::sleep(time::Duration::from_secs(5));
                println!();
                println!("Finally, all hands of the watch meet. The ringing in your ears is unbearably loud. After what seems like an eternity, everything fades to black.");
                println!();
                thread::sleep(time::Duration::from_secs(5));
                println!("============================");
                println!("THE END");
                println!("============================");
                break 'game;
            }
        }
    }
}
