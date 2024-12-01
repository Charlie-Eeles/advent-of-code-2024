use std::collections::HashMap;

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

    let mut right_col_frequency = HashMap::new();
    right_col_vec.iter().for_each(|num| {
        *right_col_frequency.entry(num).or_insert(0) += 1;
    });

    let similarity: u32 = left_col_vec
        .iter()
        .map(|num| num * right_col_frequency.get(num).unwrap_or(&0))
        .sum();

    println!("Similarity score: {}", similarity);
}
