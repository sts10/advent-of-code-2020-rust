use advent_of_code_2020::read_by_line;
// use std::fmt;

#[derive(Debug)]
struct Instruction {
    category: Category,
    direction: Direction,
    value: isize,
}

#[derive(Debug)]
struct Position {
    direction_facing: Direction,
    latitude: isize,  // north - south
    longitude: isize, // east - west
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Forward,
    Right,
    Left,
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, Debug)]
enum Category {
    Turn,         // if direction == Direction::Right or Left
    ForwardMove,  // if direction == Direction::Forward
    AbsoluteMove, // else
}

fn main() {
    let file_name = "inputs/day12.txt";
    let instructions: Vec<String> = read_by_line(file_name).unwrap();
    // let instructions: Vec<String> = vec!["F10", "N3", "F7", "L270", "F11"]
    // .iter()
    // .map(|s| s.to_string())
    // .collect();
    let mut current_position = Position {
        direction_facing: Direction::East, // The ship starts by facing east.
        latitude: 0,
        longitude: 0,
    };

    let mut instructions_as_structs: Vec<Instruction> = vec![];
    for instruction_as_string in instructions {
        instructions_as_structs.push(make_struct(instruction_as_string));
    }

    for instruction in instructions_as_structs {
        current_position = apply_instruction(instruction, current_position);
    }
    println!("CP at end is {:?}", current_position);
    println!(
        "So answer to part 1 is {}",
        current_position.latitude.abs() + current_position.longitude.abs()
    ); // 921 is too low! And 1937 is too high!
}

fn make_struct(instruction_as_string: String) -> Instruction {
    let direction_as_str = instruction_as_string.chars().nth(0).unwrap();
    let direction = match direction_as_str {
        'N' => Direction::North,
        'S' => Direction::South,
        'E' => Direction::East,
        'W' => Direction::West,
        'R' => Direction::Right,
        'L' => Direction::Left,
        'F' => Direction::Forward,
        _ => panic!("Misread direction character while attempting to create a Direction struct"),
    };

    // enum Category {
    //     Turn,           // if direction == Direction::Right or Left
    //     ForwardMove,    // if direction == Direction::Forward
    //     AbsoluteMove,   // else
    // }
    let category = if direction == Direction::Left || direction == Direction::Right {
        Category::Turn
    } else if direction == Direction::Forward {
        Category::ForwardMove
    } else {
        Category::AbsoluteMove
    };
    Instruction {
        direction,
        category,
        value: instruction_as_string[1..instruction_as_string.len()]
            .parse()
            .unwrap(),
    }
}

fn apply_instruction(instruction: Instruction, current_position: Position) -> Position {
    match instruction.category {
        Category::Turn => execute_turn(instruction, current_position),
        Category::ForwardMove => execute_forward_move(instruction, current_position),
        Category::AbsoluteMove => execute_absolute_move(instruction, current_position),
    }
}

fn execute_absolute_move(instruction: Instruction, current_position: Position) -> Position {
    if instruction.category != Category::AbsoluteMove {
        panic!(
            "Called execute_absolute_move on an instruction that is not of Category AbsoluteMove"
        );
    }
    match instruction.direction {
        Direction::East => Position {
            longitude: current_position.longitude + instruction.value,
            latitude: current_position.latitude,
            direction_facing: current_position.direction_facing,
        },
        Direction::South => Position {
            longitude: current_position.longitude,
            latitude: current_position.latitude - instruction.value,
            direction_facing: current_position.direction_facing,
        },
        Direction::West => Position {
            longitude: current_position.longitude - instruction.value,
            latitude: current_position.latitude,
            direction_facing: current_position.direction_facing,
        },

        Direction::North => Position {
            longitude: current_position.longitude,
            latitude: current_position.latitude + instruction.value,
            direction_facing: current_position.direction_facing,
        },
        _ => panic!("Don't know which direction I'm moving absolutely in"),
    }
}

fn execute_forward_move(instruction: Instruction, current_position: Position) -> Position {
    if instruction.category != Category::ForwardMove {
        panic!("Called execute_forward_move on an instruction that is not of Category ForwardMove");
    }
    match current_position.direction_facing {
        Direction::East => Position {
            longitude: current_position.longitude + instruction.value,
            latitude: current_position.latitude,
            direction_facing: current_position.direction_facing,
        },
        Direction::South => Position {
            longitude: current_position.longitude,
            latitude: current_position.latitude - instruction.value,
            direction_facing: current_position.direction_facing,
        },
        Direction::West => Position {
            longitude: current_position.longitude - instruction.value,
            latitude: current_position.latitude,
            direction_facing: current_position.direction_facing,
        },

        Direction::North => Position {
            longitude: current_position.longitude,
            latitude: current_position.latitude + instruction.value,
            direction_facing: current_position.direction_facing,
        },
        _ => panic!("Don't know which direction I'm moving forward in"),
    }
}

fn execute_turn(instruction: Instruction, current_position: Position) -> Position {
    if instruction.category != Category::Turn {
        panic!("Trying to turn a non-turn instruction");
    }
    if instruction.value % 90 != 0 {
        panic!("Trying to turn in a non-90-degree increment");
    }
    let currently_facing = match current_position.direction_facing {
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
        Direction::North => 3,
        _ => panic!("Don't know which way I'm facing before making a turn"),
    };

    // I started facing East, and was told to Instruction { category: Turn, direction: Left, value: 270 }. Now facing North
    // I started facing East, and was told to Instruction { category: Turn, direction: Left, value: 180 }. Now facing East
    //
    // OMG I literally could not work out the logic of Left turns here. I gave up and made a
    // seperate arm for the 180 turns
    let this_instruction_as_int = if instruction.value == 180 {
        2
    } else {
        match instruction.direction {
            Direction::Right => instruction.value / 90,
            Direction::Left => (instruction.value + 180) / 90,
            _ => panic!("Turning neither left nor right"),
        }
    };

    let new_direction_facing = match (currently_facing + this_instruction_as_int).abs() % 4 {
        0 => Direction::East,
        1 => Direction::South,
        2 => Direction::West,
        3 => Direction::North,
        _ => panic!("Don't know which way I'm facing before making a turn"),
    };
    println!(
        "I started facing {:?}, and was told to {:?}. Now facing {:?}",
        current_position.direction_facing, instruction, new_direction_facing
    );
    Position {
        direction_facing: new_direction_facing,
        latitude: current_position.latitude,
        longitude: current_position.longitude,
    }
}
