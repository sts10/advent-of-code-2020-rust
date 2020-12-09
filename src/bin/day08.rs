use advent_of_code_2020::{read_by_line, split_and_vectorize};

#[derive(Debug, Clone)]
struct Head {
    accumulator: isize,
    current_position: usize,
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
        let cp = my_head.current_position as usize;
        // if cp >= instructions.len() {
        //     println!("Here");
        //     finish_this_program(&instructions, &my_head);
        //     break;
        // }
        let operation = split_and_vectorize(&instructions[cp], " ")[0].to_string();
        let argument: isize =
            split_and_vectorize(&instructions[my_head.current_position as usize], " ")[1]
                .parse()
                .unwrap();
        println!("About to execute {} {}", operation, argument);
        my_head = execute(operation, argument, my_head);
        println!("Current position is {}", my_head.current_position);
        println!("Accumulator is {}", my_head.accumulator);
        if does_this_loop_infinitely_if_next_instruction_is_switched(&instructions, &my_head) {
            println!("why don't I make it here?");
            let ans2 = finish_this_program(&instructions, &my_head);
            println!("Answer to part 2 is {}", ans2);
            break;
        }
    }
}

fn does_this_loop_infinitely_if_next_instruction_is_switched(
    instructions: &[String],
    my_head: &Head,
) -> bool {
    let mut new_head = my_head.clone();
    let mut first_time = true;
    loop {
        let cp = &new_head.current_position;
        if cp >= &instructions.len() {
            // terminates correctly
            return false;
        }
        let operation = split_and_vectorize(&instructions[*cp], " ")[0].to_string();
        let operation = if first_time && operation == "nop" {
            first_time = false;
            "jmp".to_string()
        } else if first_time && operation == "jmp" {
            first_time = false;
            "nop".to_string()
        } else {
            // first_time = false;
            operation
        };

        let argument: isize =
            split_and_vectorize(&instructions[new_head.current_position as usize], " ")[1]
                .parse()
                .unwrap();
        println!("About to execute {} {}", operation, argument);
        new_head = execute(operation, argument, new_head);
        println!("Current position is {}", new_head.current_position);
        println!("Accumulator is {}", new_head.accumulator);
        if new_head.current_position > instructions.len() {
            // terminates correctly
            return false;
        }
        if new_head
            .trail_of_positions
            .contains(&(new_head.current_position as usize))
        {
            // we've looped around to this current positioan again,
            // thus we are in an infinite loop
            return true;
        }
    }
}

fn finish_this_program(instructions: &[String], my_head: &Head) -> isize {
    println!(
        "Made it to finish_this_program. Acc is {}",
        my_head.accumulator
    );
    let mut new_head = my_head.clone();
    loop {
        let cp = &new_head.current_position;
        if cp >= &instructions.len() {
            println!("Here");
            return new_head.accumulator;
        }
        let operation = split_and_vectorize(&instructions[*cp], " ")[0].to_string();
        let argument: isize =
            split_and_vectorize(&instructions[new_head.current_position as usize], " ")[1]
                .parse()
                .unwrap();
        println!("About to execute {} {}", operation, argument);
        new_head = execute(operation, argument, new_head);
        println!("Current position is {}", new_head.current_position);
        println!("Accumulator is {}", new_head.accumulator);
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
        let (abs_arg, err) = argument.overflowing_abs();
        if err {
            panic!("Overflow");
        }
        if argument > 0 {
            new_current_position = old_head.current_position + abs_arg as usize;
        } else {
            new_current_position = old_head.current_position - abs_arg as usize;
        }
    }
    Head {
        accumulator: new_accumulator,
        current_position: new_current_position,
        trail_of_positions: new_trail_of_positions.to_vec(),
    }
}
