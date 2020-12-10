use advent_of_code_2020::read_by_line;

fn main() {
    let file_name = "inputs/day09.txt";
    let numbers: Vec<usize> = read_by_line(file_name).unwrap();
    let preamble = 25;
    println!(
        "Answer to part one is {}",
        find_first_invalid_number(numbers, preamble).unwrap()
    );
}

fn find_first_invalid_number(numbers: Vec<usize>, preamble: usize) -> Option<usize> {
    for (i, n) in numbers[preamble..numbers.len()].iter().enumerate() {
        let current_position = i + preamble;
        println!("n is {}; and current_position is {}", n, current_position);
        if is_this_n_valid(current_position, *n, &numbers, preamble) == false {
            println!("First invalid I found: {}", n);
            return Some(*n);
        }
    }
    None
}

fn is_this_n_valid(
    current_position: usize,
    target_sum: usize,
    numbers: &[usize],
    preamble: usize,
) -> bool {
    let start_from = current_position - preamble;
    let up_to = current_position;
    for n in numbers[start_from..up_to].iter() {
        for m in numbers[start_from..up_to].iter() {
            if n + m == target_sum {
                return true;
            }
        }
    }
    false
}

#[test]
fn can_solve_day_nine_part_one() {
    let file_name = "inputs/day09.txt";
    let numbers: Vec<usize> = read_by_line(file_name).unwrap();
    let preamble = 25;
    assert_eq!(
        find_first_invalid_number(numbers, preamble),
        Some(530627549)
    );
}

#[test]
fn can_solve_day_nine_part_one_example() {
    let numbers: Vec<usize> = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    let preamble = 5;
    assert_eq!(find_first_invalid_number(numbers, preamble), Some(127));
}
