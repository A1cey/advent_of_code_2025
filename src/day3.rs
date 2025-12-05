pub fn solve() {
    let part_one = part_one::solve();
    let part_two = part_two_position_iterator::solve();

    dbg!(part_one);
    dbg!(part_two);

    assert_eq!(part_one, 17324);
    assert_eq!(part_two, 171846613143331);
}

pub mod part_one {
    pub fn solve() -> u32 {
        let data = include_str!("../data/day3.txt");

        data.lines().map(|bank| get_max_joltage(bank)).sum()
    }

    fn get_max_joltage(bank: &str) -> u32 {
        let (mut left, mut right) = (0, 0);
        let num_of_digits = bank.len() - 1;

        for (idx, battery) in bank.chars().enumerate() {
            let digit = battery.to_digit(10).unwrap();

            if idx < num_of_digits && left < digit {
                left = digit;
                right = 0;
            } else if right < digit {
                right = digit;
            }
        }

        left * 10 + right
    }
}

pub mod part_two {
    pub fn solve() -> u64 {
        let data = include_str!("../data/day3.txt");

        data.lines().map(|bank| get_max_joltage(bank)).sum()
    }

    fn get_max_joltage(bank: &str) -> u64 {
        let mut max_joltage = [0u64; 12];

        let (left, right) = bank.split_at(bank.len() - 12);

        for battery in left
            .chars()
            .map(|battery| battery.to_digit(10).unwrap().into())
        {
            for (joltage_idx, num) in max_joltage.iter_mut().enumerate() {
                if *num < battery {
                    *num = battery;
                    // set all numbers after the new one back to zero
                    for i in joltage_idx + 1..12 {
                        max_joltage[i] = 0;
                    }
                    break;
                }
            }
        }

        for (idx, battery) in right
            .chars()
            .map(|battery| battery.to_digit(10).unwrap().into())
            .enumerate()
        {
            for (joltage_idx, joltage) in max_joltage.iter_mut().enumerate().skip(idx) {
                if *joltage < battery {
                    *joltage = battery;
                    for i in joltage_idx + 1..12 {
                        max_joltage[i] = 0;
                    }
                    break;
                }
            }
        }

        max_joltage
            .into_iter()
            .reduce(|acc, num| acc * 10 + num)
            .unwrap()
    }
}

pub mod part_two_using_bytes_not_chars {
    pub fn solve() -> u64 {
        let data = include_str!("../data/day3.txt");

        data.lines().map(|bank| get_max_joltage(bank)).sum()
    }

    fn get_max_joltage(bank: &str) -> u64 {
        const SEQUENCE_LEN: usize = 12;
        let mut max_joltage = [0u64; SEQUENCE_LEN];

        let (left, right) = bank.split_at(bank.len() - SEQUENCE_LEN);

        for battery in left.bytes().map(|battery| u64::from(battery - b'0')) {
            for (joltage_idx, num) in max_joltage.iter_mut().enumerate() {
                if *num < battery {
                    *num = battery;
                    // set all numbers after the new one back to zero
                    for i in joltage_idx + 1..SEQUENCE_LEN {
                        max_joltage[i] = 0;
                    }
                    break;
                }
            }
        }

        for (idx, battery) in right
            .bytes()
            .map(|battery| u64::from(battery - b'0'))
            .enumerate()
        {
            for (joltage_idx, joltage) in max_joltage.iter_mut().enumerate().skip(idx) {
                if *joltage < battery {
                    *joltage = battery;
                    for i in joltage_idx + 1..SEQUENCE_LEN {
                        max_joltage[i] = 0;
                    }
                    break;
                }
            }
        }

        max_joltage
            .into_iter()
            .reduce(|acc, num| acc * 10 + num)
            .unwrap()
    }
}

pub mod part_two_position_iterator {
    pub fn solve() -> u64 {
        let data = include_str!("../data/day3.txt");

        data.lines().map(|bank| get_max_joltage(bank)).sum()
    }

    fn get_max_joltage(bank: &str) -> u64 {
        const SEQUENCE_LEN: usize = 12;
        let mut max_joltage = [0u64; SEQUENCE_LEN];

        let (left, right) = bank.split_at(bank.len() - SEQUENCE_LEN);

        for battery in left.bytes().map(|battery| u64::from(battery - b'0')) {
            if let Some(pos) = max_joltage.iter().position(|&val| val < battery) {
                max_joltage[pos] = battery;
                max_joltage[pos + 1..].fill(0);
            }
        }

        for (idx, battery) in right
            .bytes()
            .map(|battery| u64::from(battery - b'0'))
            .enumerate()
        {
            if let Some(pos) = max_joltage[idx..].iter().position(|&val| val < battery) {
                max_joltage[idx + pos] = battery;
                max_joltage[idx + pos + 1..].fill(0);
            }
        }

        max_joltage
            .into_iter()
            .reduce(|acc, num| acc * 10 + num)
            .unwrap()
    }
}
