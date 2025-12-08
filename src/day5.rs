pub fn solve() {
    let part_one = part_one::solve();
    let part_two = part_two_sorted_ranges_comparison::solve();

    dbg!(part_one);
    dbg!(part_two);

    assert_eq!(part_one, 739);
    assert_eq!(part_two, 344_486_348_901_788);
}

pub mod part_one {
    use std::ops::RangeInclusive;

    pub fn solve() -> usize {
        let data = include_str!("../data/day5.txt");

        let (ranges, ids) = parse_input(data);

        ids.iter()
            .filter(|id| ranges.iter().any(|range| range.contains(id)))
            .count()
    }

    fn parse_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
        let (ranges, ids) = input.split_once("\r\n\r\n").unwrap();

        (parse_ranges(ranges), parse_ids(ids))
    }

    fn parse_ranges(ranges: &str) -> Vec<RangeInclusive<usize>> {
        ranges
            .lines()
            .map(|range| range.split_once('-').unwrap())
            .map(|(low, high)| low.parse().unwrap()..=high.parse().unwrap())
            .collect()
    }

    fn parse_ids(ids: &str) -> Vec<usize> {
        ids.lines().map(|id| id.parse().unwrap()).collect()
    }
}

pub mod part_two_very_slow {
    use std::{collections::HashSet, ops::RangeInclusive};

    pub fn solve() -> usize {
        let data = include_str!("../data/day5.txt");

        let ranges = parse_input(data);

        let set: HashSet<_> = ranges.into_iter().flat_map(|range| range).collect();

        set.len()
    }

    fn parse_input(input: &str) -> Vec<RangeInclusive<usize>> {
        let (ranges, _) = input.split_once("\r\n\r\n").unwrap();

        parse_ranges(ranges)
    }

    fn parse_ranges(ranges: &str) -> Vec<RangeInclusive<usize>> {
        ranges
            .lines()
            .map(|range| range.split_once('-').unwrap())
            .map(|(low, high)| low.parse().unwrap()..=high.parse().unwrap())
            .collect()
    }
}

pub mod part_two_sorted_ranges_comparison {
    use std::ops::RangeInclusive;

    pub fn solve() -> usize {
        let data = include_str!("../data/day5.txt");

        let mut ranges = parse_input(data);

        // Sort ranges for lower bound
        // start with lowest range
        // take upper bound and search in ranges (sorted for lower bound)
        // while next range has lowerorequal start bound than high bound of current range
        //      compare upper bounds and use higher of both
        // -> range is complete
        // add sum of values in range to result OR save range in separate structure

        let mut sum = 0;

        ranges.sort_by(|a, b| a.start().cmp(b.start()));

        let mut current_range_idx = 0;

        while current_range_idx < ranges.len() {
            let range = &ranges[current_range_idx];
            let mut current_end = range.end();

            for next_range_idx in current_range_idx + 1..ranges.len() {
                let next_range = &ranges[next_range_idx];

                if current_end >= next_range.start() {
                    current_end = current_end.max(next_range.end());
                    current_range_idx += 1; // skip the next range
                }
            }

            sum += current_end - range.start() + 1; // +1 for inclusive range
            current_range_idx += 1;
        }

        sum
    }

    fn parse_input(input: &str) -> Vec<RangeInclusive<usize>> {
        let (ranges, _) = input.split_once("\r\n\r\n").unwrap();

        parse_ranges(ranges)
    }

    fn parse_ranges(ranges: &str) -> Vec<RangeInclusive<usize>> {
        ranges
            .lines()
            .map(|range| range.split_once('-').unwrap())
            .map(|(low, high)| low.parse().unwrap()..=high.parse().unwrap())
            .collect()
    }
}
