use advent_of_code_2020::read_by_line;

fn main() {
    let file_name = "inputs/day10.txt";
    let mut joltage_ratings: Vec<usize> = read_by_line(file_name).unwrap();
    // example joltage_ratings
    // let mut joltage_ratings: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    joltage_ratings.sort();
    println!("Answer to part 1 is {}", solve_part_1(joltage_ratings));
    // println!(
    //     "Answer to part 2 is {}",
    //     count_higher_roads_not_taken(joltage_ratings)
    // );
}

// For Part 2? Though it's probably wrong. I don't know how to appraoch Part 2 tbh
fn _count_higher_roads_not_taken(joltage_ratings: Vec<usize>) -> usize {
    // (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
    // (0), 1, 4, 5, 6, 7, 10,     12, 15, 16, 19, (22)
    // (0), 1, 4, 5,    7, 10, 11, 12, 15, 16, 19, (22)
    // (0), 1, 4, 5,    7, 10,     12, 15, 16, 19, (22)
    // (0), 1, 4,    6, 7, 10, 11, 12, 15, 16, 19, (22)
    // (0), 1, 4,    6, 7, 10,     12, 15, 16, 19, (22)
    // (0), 1, 4,       7, 10, 11, 12, 15, 16, 19, (22)
    // (0), 1, 4,       7, 10,     12, 15, 16, 19, (22)
    let mut vector_of_number_of_higher_forks_not_taken: Vec<usize> = vec![];
    for (i, _joltage) in joltage_ratings.iter().enumerate() {
        // let _next_three_joltages = vec![
        //     joltage_ratings[i + 1],
        //     joltage_ratings[i + 2],
        //     joltage_ratings[i + 3],
        // ];
        if i + 4 > joltage_ratings.len() {
            break;
        }
        let mut higher_forks_not_taken_this_time = 0;
        // we're gonna take the lowest option [i+1] every time,
        // but let's see if i+2 or i+3 qualify as valid jumps
        if joltage_ratings[i + 2] - joltage_ratings[i] <= 3 {
            higher_forks_not_taken_this_time += 1;
        }
        if joltage_ratings[i + 3] - joltage_ratings[i] <= 3 {
            higher_forks_not_taken_this_time += 1;
        }
        vector_of_number_of_higher_forks_not_taken.push(higher_forks_not_taken_this_time + 1);
    }
    println!(
        "completed vector_of_number_of_higher_forks_not_taken is {:?}",
        vector_of_number_of_higher_forks_not_taken
    );
    // gonna screw with this part...
    let mut product = 1;
    for fork_count in vector_of_number_of_higher_forks_not_taken {
        // product = product * fork_count;
        if fork_count > 1 {
            product = product * 2
        }
    }
    product
}

fn solve_part_1(joltage_ratings: Vec<usize>) -> usize {
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
    number_of_1_jumps * number_of_3_jumps
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
