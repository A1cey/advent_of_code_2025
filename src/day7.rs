#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

pub fn solve() {
    let part_one = part_one::solve();
    let part_two = part_two_mem_swap_buffer::solve();

    dbg!(part_one);
    dbg!(part_two);

    assert_eq!(part_one, 1662);
    assert_eq!(part_two, 40_941_112_789_504);
}

pub mod part_one {
    use std::collections::{HashSet, VecDeque};

    use crate::day7::Coord;

    pub fn solve() -> usize {
        let data = include_str!("../data/day7.txt");

        let mut start = Default::default();
        let mut row_len = 0;

        let mut manifold = data.as_bytes().iter();
        let mut col = 0;

        // get start_point and row length
        while let Some(point) = manifold.next() {
            match point {
                b'S' => {
                    start = Coord(0, col);
                }
                b'\n' => {
                    row_len = col + 1;
                    break;
                }
                _ => {}
            }
            col += 1;
        }

        // get splitters
        let mut splitters = vec![vec![false; row_len]];
        let mut row = 0;
        col = 0;

        for point in manifold {
            match point {
                b'^' => splitters[row][col] = true,
                b'\n' => {
                    row += 1;
                    col = 0;
                    splitters.push(vec![false; row_len]);
                    continue;
                }
                _ => {}
            }

            col += 1;
        }

        // run through manifold
        row_len = row;
        row = 0;

        let mut current_pos = VecDeque::new();
        current_pos.push_back(start);

        let mut new_pos = HashSet::new();
        let mut num_of_splits = 0;

        while row < row_len {
            while let Some(pos) = current_pos.pop_front() {
                if splitters[pos.0 + 1][pos.1] {
                    num_of_splits += 1;
                    new_pos.insert(Coord(pos.0 + 1, pos.1 - 1)); // left of splitter
                    new_pos.insert(Coord(pos.0 + 1, pos.1 + 1)); // right of splitter
                } else {
                    new_pos.insert(Coord(pos.0 + 1, pos.1)); // otherwise add same pos but one row below
                }
            }

            new_pos.drain().for_each(|pos| current_pos.push_back(pos));
            row += 1
        }

        num_of_splits
    }
}

// too much allocations, very slow
pub mod part_two_not_working {
    use std::collections::VecDeque;

    use crate::day7::Coord;

    pub fn _solve() -> usize {
        let data = include_str!("../data/day7.txt");

        let mut start = Default::default();
        let mut row_len = 0;

        let mut manifold = data.as_bytes().iter();
        let mut col = 0;

        // get start_point and row length
        while let Some(point) = manifold.next() {
            match point {
                b'S' => {
                    start = Coord(0, col);
                }
                b'\n' => {
                    row_len = col + 1;
                    break;
                }
                _ => {}
            }
            col += 1;
        }

        // get splitters
        let mut splitters = vec![vec![false; row_len]];
        let mut row = 0;
        col = 0;

        for point in manifold {
            match point {
                b'^' => splitters[row][col] = true,
                b'\n' => {
                    row += 1;
                    col = 0;
                    splitters.push(vec![false; row_len]);
                    continue;
                }
                _ => {}
            }

            col += 1;
        }

        // run through manifold
        row_len = row;
        row = 0;

        let mut current_pos = VecDeque::new();
        current_pos.push_back(start);

        let mut new_pos = VecDeque::new();
        let mut num_of_timelines = 1;

        while row < row_len {
            while let Some(pos) = current_pos.pop_front() {
                if splitters[pos.0 + 1][pos.1] {
                    num_of_timelines += 1;
                    new_pos.push_back(Coord(pos.0 + 1, pos.1 - 1)); // left of splitter
                    new_pos.push_back(Coord(pos.0 + 1, pos.1 + 1)); // right of splitter
                } else {
                    new_pos.push_back(Coord(pos.0 + 1, pos.1)); // otherwise add same pos but one row below
                }
            }

            current_pos.append(&mut new_pos);
            row += 1
        }

        num_of_timelines
    }
}

pub mod part_two_clone_buffer {
    // list of active cols
    // because huge amount of double values
    // -> store at index of col the number of paths instead
    // calculate once per index * number of paths

    use std::ops::AddAssign;

    use crate::day7::Coord;

    pub fn solve() -> usize {
        let data = include_str!("../data/day7.txt");

        let mut start = Default::default();
        let mut row_len = 0;

        let mut manifold = data.as_bytes().iter();
        let mut col = 0;

        // get start_point and row length
        while let Some(point) = manifold.next() {
            match point {
                b'S' => {
                    start = Coord(0, col);
                }
                b'\n' => {
                    row_len = col + 1;
                    break;
                }
                _ => {}
            }
            col += 1;
        }

        // get splitters
        let mut splitters = vec![vec![false; row_len]];
        let mut row = 0;
        col = 0;

        for point in manifold {
            match point {
                b'^' => splitters[row][col] = true,
                b'\n' => {
                    row += 1;
                    col = 0;
                    splitters.push(vec![false; row_len]);
                    continue;
                }
                _ => {}
            }

            col += 1;
        }

        // run through manifold
        let col_len = row;
        row = 0;

        let mut beam_cols = vec![None; row_len];
        beam_cols[start.1] = Some(1); // one timeline at start_col

        let mut next_beam_cols: Vec<Option<usize>> = vec![None; row_len];
        let mut num_of_timelines = 1;

        while row < col_len {
            for (col, num_of_beams) in
                beam_cols
                    .iter()
                    .enumerate()
                    .filter_map(|(col, num_of_beams)| {
                        num_of_beams.map(|num_of_beams| (col, num_of_beams))
                    })
            {
                //  Iterate over all
                if splitters[row + 1][col] {
                    num_of_timelines += num_of_beams;
                    next_beam_cols[col - 1]
                        .get_or_insert_default()
                        .add_assign(num_of_beams); // left of splitter
                    next_beam_cols[col + 1]
                        .get_or_insert_default()
                        .add_assign(num_of_beams); // right of splitter
                } else {
                    next_beam_cols[col]
                        .get_or_insert_default()
                        .add_assign(num_of_beams); // otherwise add same pos but one row below
                }
            }

            beam_cols.clone_from_slice(&next_beam_cols);
            next_beam_cols.fill(None);
            row += 1
        }

        num_of_timelines
    }
}

pub mod part_two_mem_swap_buffer {
    // list of active cols
    // because huge amount of double values
    // -> store at index of col the number of paths instead
    // calculate once per index * number of paths

    use std::ops::AddAssign;

    use crate::day7::Coord;

    pub fn solve() -> usize {
        let data = include_str!("../data/day7.txt");

        let mut start = Default::default();
        let mut row_len = 0;

        let mut manifold = data.as_bytes().iter();
        let mut col = 0;

        // get start_point and row length
        while let Some(point) = manifold.next() {
            match point {
                b'S' => {
                    start = Coord(0, col);
                }
                b'\n' => {
                    row_len = col + 1;
                    break;
                }
                _ => {}
            }
            col += 1;
        }

        // get splitters
        let mut splitters = vec![vec![false; row_len]];
        let mut row = 0;
        col = 0;

        for point in manifold {
            match point {
                b'^' => splitters[row][col] = true,
                b'\n' => {
                    row += 1;
                    col = 0;
                    splitters.push(vec![false; row_len]);
                    continue;
                }
                _ => {}
            }

            col += 1;
        }

        // run through manifold
        let col_len = row;
        row = 0;

        let mut beam_cols = vec![None; row_len];
        beam_cols[start.1] = Some(1); // one timeline at start_col

        let mut next_beam_cols: Vec<Option<usize>> = vec![None; row_len];
        let mut num_of_timelines = 1;

        while row < col_len {
            for (col, num_of_beams) in
                beam_cols
                    .iter()
                    .enumerate()
                    .filter_map(|(col, num_of_beams)| {
                        num_of_beams.map(|num_of_beams| (col, num_of_beams))
                    })
            {
                //  Iterate over all
                if splitters[row + 1][col] {
                    num_of_timelines += num_of_beams;
                    next_beam_cols[col - 1]
                        .get_or_insert_default()
                        .add_assign(num_of_beams); // left of splitter
                    next_beam_cols[col + 1]
                        .get_or_insert_default()
                        .add_assign(num_of_beams); // right of splitter
                } else {
                    next_beam_cols[col]
                        .get_or_insert_default()
                        .add_assign(num_of_beams); // otherwise add same pos but one row below
                }
            }

            beam_cols.swap_with_slice(&mut next_beam_cols);
            next_beam_cols.fill(None);
            row += 1
        }

        num_of_timelines
    }
}

pub mod part_two_int_not_option_buffer {
    // list of active cols
    // because huge amount of double values
    // -> store at index of col the number of paths instead
    // calculate once per index * number of paths

    use crate::day7::Coord;

    pub fn solve() -> usize {
        let data = include_str!("../data/day7.txt");

        let mut start = Default::default();
        let mut row_len = 0;

        let mut manifold = data.as_bytes().iter();
        let mut col = 0;

        // get start_point and row length
        while let Some(point) = manifold.next() {
            match point {
                b'S' => {
                    start = Coord(0, col);
                }
                b'\n' => {
                    row_len = col + 1;
                    break;
                }
                _ => {}
            }
            col += 1;
        }

        // get splitters
        let mut splitters = vec![vec![false; row_len]];
        let mut row = 0;
        col = 0;

        for point in manifold {
            match point {
                b'^' => splitters[row][col] = true,
                b'\n' => {
                    row += 1;
                    col = 0;
                    splitters.push(vec![false; row_len]);
                    continue;
                }
                _ => {}
            }

            col += 1;
        }

        // run through manifold
        let col_len = row;
        row = 0;

        let mut beam_cols = vec![0usize; row_len];
        beam_cols[start.1] = 1; // one timeline at start_col

        let mut next_beam_cols = vec![0usize; row_len];
        let mut num_of_timelines = 1;

        while row < col_len {
            for (col, num_of_beams) in beam_cols.iter().enumerate() {
                if *num_of_beams == 0 {
                    continue;
                }

                if splitters[row + 1][col] {
                    num_of_timelines += num_of_beams;
                    next_beam_cols[col - 1] += num_of_beams; // left of splitter
                    next_beam_cols[col + 1] += num_of_beams; // right of splitter
                } else {
                    next_beam_cols[col] += num_of_beams; // otherwise add same pos but one row below
                }
            }

            beam_cols.clone_from_slice(&next_beam_cols);
            next_beam_cols.fill(0);
            row += 1
        }

        num_of_timelines
    }
}
