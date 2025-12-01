
pub fn solve() {
    let (part_one, part_two) = solution1::solve();

    dbg!(part_one);
    dbg!(part_two);
}

pub mod solution1 {
    pub fn solve() -> (u32, u32) {
        let data = include_str!("../data/day1.txt");

        (42, 42)
    }
}
