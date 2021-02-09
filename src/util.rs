use std::io;

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub fn get_trimmed_input(input: &mut String) -> &str {
    input.clear();
    io::stdin().read_line(input).unwrap();
    input.trim()
}

pub fn get_room_name_border() -> &'static str {
    "==============="
}
