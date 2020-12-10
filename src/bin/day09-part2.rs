use advent_of_code_2020::read_by_line;

fn main() {
    let file_name = "inputs/day09.txt";
    let numbers: Vec<usize> = read_by_line(file_name).unwrap();
    // taking constants is probably frowned upon, but...
    // From part 1 with my input:
    let target_sum = 530627549;
    let target_sum_position = 610;
    'outer: for width in 2..target_sum_position {
        for slice in numbers.windows(width) {
            if slice.iter().sum::<usize>() == target_sum {
                println!("slice is {:?}", slice);
                let slice_max = slice.iter().max_by(|a, b| a.cmp(b)).unwrap();
                let slice_min = slice.iter().min_by(|a, b| a.cmp(b)).unwrap();
                println!("part 2 ans is {}", slice_min + slice_max);
                break 'outer;
            }
        }
    }
}
