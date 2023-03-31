use advent_of_code_2020::read_by_line;
use advent_of_code_2020::split_and_vectorize;

fn main() {
    let file_name = "inputs/day02.txt";
    let policies_and_passwords: Vec<String> = read_by_line(file_name).unwrap();
    let mut number_of_valid_passwords = 0;
    for policy_and_password in &policies_and_passwords {
        if is_valid_for_part_one(policy_and_password) {
            number_of_valid_passwords += 1
        }
    }
    println!(
        "Number of valid passwords in part one: {}",
        number_of_valid_passwords
    );

    number_of_valid_passwords = 0;
    for policy_and_password in &policies_and_passwords {
        if is_valid_for_part_two(policy_and_password) {
            number_of_valid_passwords += 1
        }
    }
    println!(
        "Number of valid passwords in part two: {}",
        number_of_valid_passwords
    );
}

fn is_valid_for_part_one(line: &str) -> bool {
    let password = split_and_vectorize(&line, ":")[1];
    let policy = split_and_vectorize(&line, ":")[0];
    let target_character: char = split_and_vectorize(policy, " ")[1].parse().unwrap();

    let lower_bound: usize = split_and_vectorize(split_and_vectorize(policy, " ")[0], "-")[0]
        .parse()
        .unwrap();
    let upper_bound: usize = split_and_vectorize(split_and_vectorize(policy, " ")[0], "-")[1]
        .parse()
        .unwrap();

    let mut character_count = 0;
    for character in password.chars() {
        if character == target_character {
            character_count += 1;
        }
    }
    lower_bound <= character_count && character_count <= upper_bound
}

fn is_valid_for_part_two(line: &str) -> bool {
    let password = split_and_vectorize(&line, ":")[1];
    let policy = split_and_vectorize(&line, ":")[0];
    let target_character: char = split_and_vectorize(policy, " ")[1].parse().unwrap();

    let slot_one: usize = split_and_vectorize(split_and_vectorize(policy, " ")[0], "-")[0]
        .parse()
        .unwrap();
    let slot_two: usize = split_and_vectorize(split_and_vectorize(policy, " ")[0], "-")[1]
        .parse()
        .unwrap();

    let character_in_slot_one = password.chars().nth(slot_one).unwrap();
    let character_in_slot_two = password.chars().nth(slot_two).unwrap();

    (character_in_slot_one == target_character || character_in_slot_two == target_character)
        && !(character_in_slot_one == target_character && character_in_slot_two == target_character)
}
