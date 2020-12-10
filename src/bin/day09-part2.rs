use advent_of_code_2020::read_by_line;

fn main() {
    let file_name = "inputs/day09.txt";
    let numbers: Vec<usize> = read_by_line(file_name).unwrap();
    // let preamble = 25;
    // From part 1
    let target_sum = 530627549;
    let target_sum_position = 610;
    // let numbers: Vec<usize> = vec![
    //     35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    // ];
    // let preamble = 5;
    for width in 2..target_sum_position {
        for slice in numbers.windows(width) {
            if slice.iter().sum::<usize>() == target_sum {
                println!("slice is {:?}", slice);
                let slice_max = slice.iter().max_by(|a, b| a.cmp(b)).unwrap();
                let slice_min = slice.iter().min_by(|a, b| a.cmp(b)).unwrap();
                println!("part 2 ans is {}", slice_min + slice_max);
                break;
            }
        }
    }
}

fn _is_this_n_valid(current_position: usize, target_sum: &usize, numbers: &[usize]) -> bool {
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
