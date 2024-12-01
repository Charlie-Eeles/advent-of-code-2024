use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let mut current_number = String::new();

    let mut in_left_column = true;
    let mut left_col_vec: Vec<u32> = Vec::new();
    let mut right_col_vec: Vec<u32> = Vec::new();

    for char in input.chars() {
        if char.is_ascii_digit() {
            current_number.push(char);
        } else if current_number.is_empty() {
            continue;
        } else {
            let complete_number = current_number.parse::<u32>().unwrap();
            if in_left_column {
                left_col_vec.push(complete_number);
            } else {
                right_col_vec.push(complete_number);
            }
            current_number = String::new();
            in_left_column = !in_left_column;
        }
    }

    let mut right_col_frequency = HashMap::new();
    for num in right_col_vec.iter() {
        *right_col_frequency.entry(num).or_insert(0) += 1;
    }

    let mut similarity: u32 = 0;
    for num in left_col_vec.iter() {
        let frequency = right_col_frequency.get(num).unwrap_or(&0);
        similarity += num * frequency;
    }

    println!("Similarity score: {}", similarity);
}
