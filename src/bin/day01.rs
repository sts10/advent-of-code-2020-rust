use advent_of_code_2020::read_by_line;

fn main() {
    let file_name = "inputs/day01.txt";
    let expenses: Vec<usize> = read_by_line(file_name).unwrap();
    match solve_part_one(&expenses) {
        Some(ans) => println!("Answer to part one is: {}", ans),
        None => println!("No answer found for part one"),
    }
    match solve_part_two(&expenses) {
        Some(ans) => println!("Answer to part two is: {}", ans),
        None => println!("No answer found for part two"),
    }
}

fn solve_part_one(expenses: &[usize]) -> Option<usize> {
    for i in 0..expenses.len() {
        for j in 0..expenses.len() {
            if expenses[i] + expenses[j] == 2020 {
                return Some(expenses[i] * expenses[j]);
            }
        }
    }
    None
}

fn solve_part_two(expenses: &[usize]) -> Option<usize> {
    for i in 0..expenses.len() {
        for j in 0..expenses.len() {
            for k in 0..expenses.len() {
                if expenses[i] + expenses[j] + expenses[k] == 2020 {
                    return Some(expenses[i] * expenses[j] * expenses[k]);
                }
            }
        }
    }
    None
}

#[test]
fn can_solve_part_one() {
    let file_name = "inputs/day01.txt";
    let expenses: Vec<usize> = read_by_line(file_name).unwrap();
    assert_eq!(solve_part_one(&expenses), Some(1009899));
}

#[test]
fn can_solve_part_two() {
    let file_name = "inputs/day01.txt";
    let expenses: Vec<usize> = read_by_line(file_name).unwrap();
    assert_eq!(solve_part_two(&expenses), Some(44211152));
}
