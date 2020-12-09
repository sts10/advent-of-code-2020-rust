use advent_of_code_2020::{read_by_line, split_and_vectorize};

fn main() {
    let file_name = "inputs/day09.txt";
    let numbers: Vec<usize> = read_by_line(file_name).unwrap();
    for n in numbers {
        println!("{}", n);
    }
}
