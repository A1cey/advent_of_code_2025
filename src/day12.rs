pub fn solve() {
    let part_one = part_one::solve();
    let part_two = part_two::solve();

    dbg!(part_one);
    dbg!(part_two);

    assert_eq!(part_one, 42);
    assert_eq!(part_two, 42);
}

pub mod part_one {
    pub fn solve() -> usize {
        let data = include_str!("../data/day12.txt");
        42
    }
}

pub mod part_two {
    pub fn solve() -> usize {
        let data = include_str!("../data/day12.txt");
        42
    }
}
