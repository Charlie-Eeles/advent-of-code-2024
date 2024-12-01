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
    left_col_vec.sort();
    right_col_vec.sort();

    let mut difference: u32 = 0;
    for idx in 0..left_col_vec.len() {
        let left = left_col_vec.get(idx);
        let right = right_col_vec.get(idx);
        let max = left.max(right).unwrap();
        let min = left.min(right).unwrap();
        difference += max - min;
    }

    println!("Difference: {difference}")
}
