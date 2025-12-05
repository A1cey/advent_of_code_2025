// divided by 11, 101, 1001, ...
// 99 -> 9*11, 123123 -> 123*1001
// as long as k is even length and divisable by a number n 11, 101, 1001, ...
// and the length of n is greater then length of x
// it is a repeated sequece
// n = 10^a + 1, x < 10^a
//
// k/n = x -> n=10^((1/2 length of k)+1) + 1, x< 10^((1/2 length of k)+1)
// if length of k is even and x is whole this is a repeating number
//
// TODO: Part two, only one path where all pattern lengths are checked until halth of num length
pub fn solve() {
    let part_one = part_one_iterator::solve();
    let part_two = part_two_division::solve();

    dbg!(part_one);
    dbg!(part_two);

    assert_eq!(part_one, 13_919_717_792);
    assert_eq!(part_two, 14_582_313_461);
}

pub mod part_one_conversion_to_string {
    pub fn solve() -> u64 {
        let data = include_str!("../data/day2.txt");

        let mut result = 0;

        for range in data.split(',').map(|range| {
            let (low, high) = range.split_once('-').unwrap();
            low.parse().unwrap()..high.parse::<u64>().unwrap() + 1
        }) {
            for num in range {
                let num_str = num.to_string();
                let (left, right) = num_str.split_at(num_str.len() / 2);
                if left == right {
                    result += num;
                }
            }
        }

        result
    }
}

pub mod part_one_all_numbers_with_log {
    pub fn solve() -> u64 {
        let data = include_str!("../data/day2.txt");

        let mut result = 0;

        for range in data.split(',').map(|range| {
            let (low, high) = range.split_once('-').unwrap();
            low.parse().unwrap()..high.parse::<u64>().unwrap() + 1
        }) {
            for num in range {
                let num_len = num.ilog10() + 1;

                if !num_len.is_multiple_of(2) {
                    continue;
                }

                let divisor = 10u64.pow(num_len / 2) + 1;

                if num % divisor == 0 {
                    result += num;
                }
            }
        }

        result
    }
}

pub mod part_one_all_numbers_with_loop {
    pub fn solve() -> u64 {
        let data = include_str!("../data/day2.txt");

        let mut result = 0;

        for range in data.split(',').map(|range| {
            let (low, high) = range.split_once('-').unwrap();
            low.parse().unwrap()..high.parse::<u64>().unwrap() + 1
        }) {
            for num in range {
                let num_len = get_len(num);

                if !num_len.is_multiple_of(2) {
                    continue;
                }

                let divisor = 10u64.pow(num_len / 2) + 1;

                if num % divisor == 0 {
                    result += num;
                }
            }
        }

        result
    }

    fn get_len(mut num: u64) -> u32 {
        let mut len = 0;
        while num != 0 {
            num /= 10;
            len += 1;
        }

        len
    }
}

pub mod part_one_iterator {
    pub fn solve() -> u64 {
        let data = include_str!("../data/day2.txt");

        data.split(',')
            .map(|range| {
                let (low, high) = range.split_once('-').unwrap();
                low.parse().unwrap()..high.parse::<u64>().unwrap() + 1
            })
            .flat_map(|range| {
                range.map(|num| {
                    let num_len = num.ilog10() + 1;

                    if !num_len.is_multiple_of(2) {
                        return 0;
                    }

                    let divisor = 10u64.pow(num_len / 2) + 1;

                    if num % divisor == 0 { num } else { 0 }
                })
            })
            .sum()
    }
}

pub mod part_two_division {
    pub fn solve() -> u64 {
        let data = include_str!("../data/day2.txt");

        let mut result = 0;

        for range in data.split(',').map(|range| {
            let (low, high) = range.split_once('-').unwrap();
            low.parse().unwrap()..high.parse::<u64>().unwrap() + 1
        }) {
            for num in range {
                let num_len = num.ilog10() + 1;

                if !num_len.is_multiple_of(2) {
                    // slow path same as below but only for odd lengths -> check for pattern length n = 1,3,5,...,1/2*length
                    if slow_path(
                        (1..num_len / 2 + 1).into_iter().filter(|n| n % 2 != 0),
                        num,
                        num_len,
                    ) {
                        result += num;
                    }

                    continue;
                }

                let divisor = 10u64.pow(num_len / 2) + 1;

                // fast path or slow path for all lengths -> check for pattern length n = 1,2,3,...,1/2*length
                if num % divisor == 0 || slow_path(1..num_len / 2 + 1, num, num_len) {
                    result += num;
                }
            }
        }

        result
    }

    // length/n must be whole
    // to check n must be repeated length/n times
    // for every num x with length x_len
    // for every pattern with length n < 1/2*x_len
    //
    // p_len = x_len/n
    // if p_len is whole                           x = 121212
    // take high/low p_len digits of x as pattern p -> --or--  for p_len == 2
    // x/10^(x_len-p_len)
    //
    // repeat p n-times
    // p_num = 0
    // n*(p_num=(p_num * 10^p_len + p))
    //
    // if p_num == x then x is repeated pattern
    //
    // OR
    //
    // divide x by 10^(x_len-p_len) n times with each rem being the same then x is a repeated pattern
    fn slow_path(pattern_lens: impl IntoIterator<Item = u32>, num: u64, num_len: u32) -> bool {
        for pattern_len in pattern_lens {
            // num_len must be multiple of pattern_len
            if num_len % pattern_len != 0 {
                return false;
            }
            let num_of_repeats = num_len / pattern_len;

            let mut aggregated_num = 0;

            let pattern = num / 10u64.pow(num_len - pattern_len);

            for _ in 0..num_of_repeats {
                aggregated_num = aggregated_num * 10u64.pow(pattern_len) + pattern;
            }

            if aggregated_num == num {
                return true;
            }
        }
        false
    }
}

pub mod part_two_compare_remainder {
    pub fn solve() -> u64 {
        let data = include_str!("../data/day2.txt");

        let mut result = 0;

        for range in data.split(',').map(|range| {
            let (low, high) = range.split_once('-').unwrap();
            low.parse().unwrap()..high.parse::<u64>().unwrap() + 1
        }) {
            for num in range {
                let num_len = num.ilog10() + 1;

                if !num_len.is_multiple_of(2) {
                    // slow path same as below but only for odd lengths -> check for pattern length n = 1,3,5,...,1/2*length
                    if slow_path(
                        (1..num_len / 2 + 1).into_iter().filter(|n| n % 2 != 0),
                        num,
                        num_len,
                    ) {
                        result += num;
                    }

                    continue;
                }

                let divisor = 10u64.pow(num_len / 2) + 1;

                // fast path or slow path for all lengths -> check for pattern length n = 1,2,3,...,1/2*length
                if num % divisor == 0 || slow_path((1..num_len / 2 + 1).into_iter(), num, num_len) {
                    result += num;
                }
            }
        }

        result
    }

    // length/n must be whole
    // to check n must be repeated length/n times
    // for every num x with length x_len
    // for every pattern with length n < 1/2*x_len
    //
    // p_len = x_len/n
    // if p_len is whole                           x = 121212
    // take high/low p_len digits of x as pattern p -> --or--  for p_len == 2
    // x/10^(x_len-p_len)
    //
    // repeat p n-times
    // p_num = 0
    // n*(p_num=(p_num * 10^p_len + p))
    //
    // if p_num == x then x is repeated pattern
    //
    // OR
    //
    // divide x by 10^(p_len) n times with each rem being the same then x is a repeated pattern
    fn slow_path(pattern_lens: impl IntoIterator<Item = u32>, num: u64, num_len: u32) -> bool {
        'outer: for pattern_len in pattern_lens {
            // num_len must be multiple of pattern_len
            if num_len % pattern_len != 0 {
                continue;
            }
            let num_of_repeats = num_len / pattern_len;

            let divisor = 10u64.pow(pattern_len);
            let expected_rem = num % divisor;
            let mut num = num;
            num /= divisor;

            for _ in 0..num_of_repeats - 1 {
                let rem = num % divisor;
                num /= divisor;

                if rem != expected_rem {
                    continue 'outer;
                }
            }

            return true;
        }
        false
    }
}
