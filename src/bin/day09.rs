use advent_of_code_2020::{read_by_line, split_and_vectorize};

fn main() {
    let file_name = "inputs/day09.txt";
    let numbers: Vec<usize> = read_by_line(file_name).unwrap();
    let preamble = 25;
    // let numbers: Vec<usize> = vec![
    //     35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    // ];
    // let preamble = 5;
    for (i, n) in numbers.iter().enumerate() {
        if i < preamble {
            continue;
        }
        println!("n is {}; and current_position is {}", n, i);
        if is_this_n_valid(i, n, &numbers) == false {
            println!("First invalid I found: {}", n);
            break;
        }
    }
}

fn is_this_n_valid(current_position: usize, target_sum: &usize, numbers: &[usize]) -> bool {
    println!("checking validity...");
    for (i, n) in numbers[0..numbers.len()].iter().enumerate() {
        // println!("i is {}", i);
        for (j, m) in numbers[0..numbers.len()].iter().enumerate() {
            if i < current_position
                && i >= current_position - 25
                && j < current_position
                && j >= current_position - 25
            {
                if *target_sum == 127 {
                    println!("about to check if {} + {} == {}", n, m, target_sum);
                }
                if n + m == *target_sum {
                    return true;
                }
            }
        }
    }
    false
}
