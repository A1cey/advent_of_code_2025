pub fn solve() {
    let (part_one, part_two) = solution1::solve();

    dbg!(part_one);
    dbg!(part_two);
}

pub mod solution1 {
    pub fn solve() -> (u32, u32) {
        let data = include_str!("../data/day1.txt");

        let mut curr_num = 50;
        let mut old_curr_num = 50;
        let mut zeros_part_one = 0;
        let mut zeros_part_two = 0;

        data.lines().for_each(|line| {
            let (sign_str, num_str) = line.split_at(1);

            let sign = match sign_str {
                "L" => -1,
                "R" => 1,
                _ => unreachable!(),
            };

            old_curr_num = curr_num;
            curr_num += num_str.parse::<i32>().unwrap() * sign;
            
            while curr_num < 0 || curr_num > 99 {
                if curr_num > 99 {
                    // if we are at one hundred then we are actually at zero and to not double count we do not increase the value
                    if curr_num != 100 {
                        zeros_part_two += 1;
                    }
                    curr_num -= 100;
                } else {
                    // if started at zero then a negative number does not mean, that we went past zero
                    if old_curr_num != 0 {
                        zeros_part_two += 1;
                    }
                    
                    curr_num = 100 + curr_num;
                }
                old_curr_num = curr_num;
            }

            if curr_num == 0 {
                zeros_part_one += 1;
            }
        });

        (zeros_part_one, zeros_part_two + zeros_part_one)
    }
}
