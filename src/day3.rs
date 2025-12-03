pub fn solve() {
    let part_one = part_one::solve();
    let part_two = 42;

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
        
    }
}