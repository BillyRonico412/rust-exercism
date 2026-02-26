fn check_coord(input: &[Vec<u64>], row_to_checked: usize, col_to_checked: usize) -> bool {
    let tree_to_checked = input[row_to_checked][col_to_checked];
    for (c, &current_tree) in input[row_to_checked].iter().enumerate() {
        if c == col_to_checked {
            continue;
        }
        if current_tree > tree_to_checked {
            return false;
        }
    }
    for (r, row) in input.iter().enumerate() {
        if r == row_to_checked {
            continue;
        }
        let current_tree = row[col_to_checked];
        if current_tree < tree_to_checked {
            return false;
        }
    }
    true
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for row_to_checked in 0..input.len() {
        for col_to_checked in 0..input[row_to_checked].len() {
            if check_coord(input, row_to_checked, col_to_checked) {
                result.push((row_to_checked, col_to_checked));
            }
        }
    }
    result
}
