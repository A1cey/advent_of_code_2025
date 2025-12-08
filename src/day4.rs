use std::ops::Add;

#[derive(Clone, Copy)]
struct Position {
    row: isize,
    col: isize,
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Add<&Position> for &Position {
    type Output = Position;

    fn add(self, rhs: &Position) -> Self::Output {
        Position {
            row: (*self).row + (*rhs).row,
            col: (*self).col + (*rhs).col,
        }
    }
}

struct Grid(Vec<Vec<u8>>);

impl Grid {
    pub fn get(&self, pos: Position) -> Option<&u8> {
        self.0
            .get(pos.row as usize)
            .and_then(|row| row.get(pos.col as usize))
    }

    pub fn set(&mut self, pos: Position, val: u8) {
        self.0[pos.row as usize][pos.col as usize] = val;
    }
}

pub fn solve() {
    let part_one = part_one::solve();
    let part_two = part_two::solve();

    dbg!(part_one);
    dbg!(part_two);

    assert_eq!(part_one, 1533);
    assert_eq!(part_two, 9206);
}

pub mod part_one {
    use crate::day4::{Grid, Position};

    pub fn solve() -> usize {
        let data = include_str!("../data/day4.txt");

        let grid: Grid = Grid(data.lines().map(|line| line.bytes().collect()).collect());

        grid.0
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(col_idx, col)| {
                        **col == b'@'
                            && has_fewer_than_four_neighbours(
                                Position {
                                    row: row_idx as isize,
                                    col: *col_idx as isize,
                                },
                                &grid,
                            )
                    })
                    .count()
            })
            .sum()
    }

    fn has_fewer_than_four_neighbours(pos: Position, grid: &Grid) -> bool {
        const NEIGHBOURS: [Position; 8] = [
            Position { row: -1, col: -1 },
            Position { row: 0, col: -1 },
            Position { row: 1, col: -1 },
            Position { row: -1, col: 0 },
            Position { row: 1, col: 0 },
            Position { row: -1, col: 1 },
            Position { row: 0, col: 1 },
            Position { row: 1, col: 1 },
        ];

        NEIGHBOURS
            .iter()
            .filter(|&position| match grid.get(&pos + position) {
                Some(b'@') => true,
                _ => false,
            })
            .count()
            < 4
    }
}

pub mod part_two {
    use crate::day4::{Grid, Position};

    pub fn solve() -> usize {
        let data = include_str!("../data/day4.txt");

        let mut grid: Grid = Grid(data.lines().map(|line| line.bytes().collect()).collect());

        let height = grid.0.len();
        let width = grid.0[0].len();

        let mut count_removed = 0;

        loop {
            let take: Vec<Position> = (0..height)
                .flat_map(|row| {
                    (0..width)
                        .map(move |col| Position {
                            row: row as isize,
                            col: col as isize,
                        })
                        .filter(|pos| {
                            *grid.get(*pos).unwrap() == b'@'
                                && has_fewer_than_four_neighbours(*pos, &grid)
                        })
                })
                .collect();

            if take.len() == 0 {
                break;
            }

            count_removed += take.len();

            take.into_iter().for_each(|pos| grid.set(pos, b'.'));
        }

        count_removed
    }

    fn has_fewer_than_four_neighbours(pos: Position, grid: &Grid) -> bool {
        const NEIGHBOURS: [Position; 8] = [
            Position { row: -1, col: -1 },
            Position { row: 0, col: -1 },
            Position { row: 1, col: -1 },
            Position { row: -1, col: 0 },
            Position { row: 1, col: 0 },
            Position { row: -1, col: 1 },
            Position { row: 0, col: 1 },
            Position { row: 1, col: 1 },
        ];

        NEIGHBOURS
            .iter()
            .filter(|&position| match grid.get(&pos + position) {
                Some(b'@') => true,
                _ => false,
            })
            .count()
            < 4
    }
}
