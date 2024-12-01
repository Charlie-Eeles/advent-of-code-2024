use std::iter::zip;

fn main() {
    let input = include_str!("../input.txt");
    let mut left_col_vec = Vec::new();
    let mut right_col_vec = Vec::new();

    input
        .split_whitespace()
        .enumerate()
        .for_each(|(idx, str_num)| {
            let num: u32 = str_num.parse().unwrap();
            if idx % 2 == 0 {
                left_col_vec.push(num);
            } else {
                right_col_vec.push(num);
            }
        });

    left_col_vec.sort();
    right_col_vec.sort();

    let difference: u32 = zip(left_col_vec.iter(), right_col_vec.iter())
        .map(|(left_col, right_col)| (left_col.max(right_col) - left_col.min(right_col)))
        .sum();

    println!("Difference: {difference}");
}
