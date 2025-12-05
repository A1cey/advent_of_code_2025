pub fn solve() {
    let part_one = part_one::solve();
    let part_two = part_two::solve();

    dbg!(part_one);
    dbg!(part_two);
}

pub mod part_one {
    pub fn solve() -> u32 {
        let data = include_str!("../data/day3.txt");

        data.lines().map(|bank| get_max_joltage(bank)).sum()
    }

    fn get_max_joltage(bank: &str) -> u32 {
        let (mut left, mut right) = (0, 0);
        let num_of_digits = bank.len() - 1;

        for (idx, batterie) in bank.chars().enumerate() {
            let digit = batterie.to_digit(10).unwrap();

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

        for batterie in left
            .chars()
            .map(|batterie| batterie.to_digit(10).unwrap().into())
        {
            for (joltage_idx, num) in max_joltage.iter_mut().enumerate() {
                if *num < batterie {
                    *num = batterie;
                    // set all numbers after the new one back to zero
                    for i in joltage_idx + 1..12 {
                        max_joltage[i] = 0;
                    }
                    break;
                }
            }
        }

        for (idx, batterie) in right
            .chars()
            .map(|batterie| batterie.to_digit(10).unwrap().into())
            .enumerate()
        {
            for (joltage_idx, joltage) in max_joltage.iter_mut().enumerate().skip(idx) {
                if *joltage < batterie {
                    *joltage = batterie;
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
