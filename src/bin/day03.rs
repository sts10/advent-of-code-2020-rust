use advent_of_code_2020::read_by_line;

fn main() {
    let file_name = "inputs/day03.txt";
    let terrain: Vec<String> = read_by_line(file_name).unwrap();

    println!(
        "Trees hit in part one: {}",
        count_tree_hits_for_given_path(3, 1, &terrain)
    );

    // Part two
    let trees_hit_slope_1 = count_tree_hits_for_given_path(1, 1, &terrain);
    let trees_hit_slope_2 = count_tree_hits_for_given_path(3, 1, &terrain);
    let trees_hit_slope_3 = count_tree_hits_for_given_path(5, 1, &terrain);
    let trees_hit_slope_4 = count_tree_hits_for_given_path(7, 1, &terrain);
    let trees_hit_slope_5 = count_tree_hits_for_given_path(1, 2, &terrain);

    let product = trees_hit_slope_1
        * trees_hit_slope_2
        * trees_hit_slope_3
        * trees_hit_slope_4
        * trees_hit_slope_5;
    println!("Part two answer is: {}", product);
}

fn count_tree_hits_for_given_path(
    column_increment: usize,
    row_increment: usize,
    terrain: &[String],
) -> usize {
    let mut number_of_trees_hit = 0;
    let mut current_col = 0;
    for (current_row, line) in terrain.iter().enumerate() {
        if current_row % row_increment == 0 {
            if is_tree(current_col, line) {
                number_of_trees_hit += 1;
            }
            current_col += column_increment;
        }
    }
    number_of_trees_hit
}

fn is_tree(col_number: usize, line_of_terrain: &str) -> bool {
    let row_width = line_of_terrain.len();
    let this_space = line_of_terrain.chars().nth(col_number % row_width).unwrap();
    this_space == '#'
}
