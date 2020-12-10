use advent_of_code_2020::read_by_line;

fn main() {
    let file_name = "inputs/day10.txt";
    let mut joltage_ratings: Vec<usize> = read_by_line(file_name).unwrap();
    // let mut joltage_ratings: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    joltage_ratings.sort();

    let differences = make_vector_of_differences(joltage_ratings);

    let mut number_of_1_jumps = 0;
    let mut number_of_3_jumps = 0;
    for diff in differences {
        if diff == 1 {
            number_of_1_jumps += 1;
        } else if diff == 3 {
            number_of_3_jumps += 1;
        }
    }
    println!(
        "So the answer to part 1 is {}",
        number_of_1_jumps * number_of_3_jumps
    );
}

fn make_vector_of_differences(joltage_ratings: Vec<usize>) -> Vec<usize> {
    // we want to pre-populate the Vector with the lowest joltage, since we start at 0
    // and since the inputted joltage_ratings Vector is already sorted,
    // we know this is just the first element. (first_element - 0 == first_element)
    let mut differences: Vec<usize> = vec![joltage_ratings[0]];
    for pos in 1..joltage_ratings.len() {
        let this_difference = joltage_ratings[pos] - joltage_ratings[pos - 1];
        differences.push(this_difference);
    }
    differences.push(3); // last difference is for the device and is always 3
    differences
}
