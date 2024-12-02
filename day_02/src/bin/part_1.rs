fn main() {
    let input = include_str!("../input.txt");

    //TODO: hah this is messy, needs cleaning up
    let mut num_of_safe_reports: u32 = 0;
    for line in input.lines() {
        let mut prev_num: u32 = 0;
        let mut is_unsafe = false;
        let mut is_increasing = false;

        line.split_whitespace()
            .enumerate()
            .for_each(|(idx, str_num)| {
                let num: u32 = str_num.parse().unwrap();
                if idx > 0 && !is_unsafe {
                    let max = num.max(prev_num);
                    let min = num.min(prev_num);
                    let mut out_of_order = false;

                    if idx == 1 {
                        is_increasing = max == num;
                    } else if is_increasing && max != num || !is_increasing && min != num {
                        out_of_order = true;
                    }
                    let diff = max - min;
                    is_unsafe = !(1..=3).contains(&diff) || out_of_order;
                }
                prev_num = num;
            });
        if !is_unsafe {
            num_of_safe_reports += 1;
        }
    }
    println!("Num of safe reports: {num_of_safe_reports}");
}
