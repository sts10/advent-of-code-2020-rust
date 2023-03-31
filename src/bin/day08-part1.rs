use advent_of_code_2020::{read_by_line, split_and_vectorize};

#[derive(Debug)]
struct Head {
    accumulator: isize,
    current_position: isize,
    trail_of_positions: Vec<usize>,
}

fn main() {
    let file_name = "inputs/day08.txt";
    let instructions: Vec<String> = read_by_line(file_name).unwrap();
    let mut my_head = Head {
        accumulator: 0,
        current_position: 0,
        trail_of_positions: vec![0],
    };

    loop {
        let operation = split_and_vectorize(&instructions[my_head.current_position as usize], " ")
            [0]
        .to_string();
        let argument: isize =
            split_and_vectorize(&instructions[my_head.current_position as usize], " ")[1]
                .parse()
                .unwrap();
        println!("About to execute {} {}", operation, argument);
        my_head = execute(operation, argument, my_head);
        if my_head
            .trail_of_positions
            .contains(&(my_head.current_position as usize))
        {
            println!(
                "The accumulator at this juncture, and thus answer to part 1 is {}",
                my_head.accumulator
            );
            break;
        }
    }
}

fn execute(operation: String, argument: isize, old_head: Head) -> Head {
    let mut new_accumulator = old_head.accumulator;
    let mut new_current_position = old_head.current_position;
    let mut new_trail_of_positions = old_head.trail_of_positions;
    new_trail_of_positions.push(old_head.current_position as usize);

    if operation == "nop" {
        new_current_position += 1;
    } else if operation == "acc" {
        new_accumulator += argument;
        new_current_position += 1;
    } else if operation == "jmp" {
        new_current_position = old_head.current_position + argument;
    }
    Head {
        accumulator: new_accumulator,
        current_position: new_current_position,
        trail_of_positions: new_trail_of_positions,
    }
}
