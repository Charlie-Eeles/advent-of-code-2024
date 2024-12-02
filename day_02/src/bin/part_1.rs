fn main() {
    let input = include_str!("../input.txt");

    let num_of_safe_reports = input
        .lines()
        .filter(|line| {
            let report_vec: Vec<u32> = line
                .split_whitespace()
                .map(|str_num| str_num.parse::<u32>().unwrap())
                .collect();
            let is_ascending = report_vec[0] < report_vec[1];

            !report_vec.windows(2).any(|num| {
                let max = num[0].max(num[1]);
                let min = num[0].min(num[1]);
                let diff = max - min;

                !(1..=3).contains(&diff)
                    || (is_ascending && max == num[0])
                    || (!is_ascending && min == num[0])
            })
        })
        .count();
    println!("Num of safe reports: {num_of_safe_reports}");
}
