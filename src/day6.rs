#[derive(Debug, Clone, Copy)]
enum Operator {
    Addition,
    Multiplication,
}

pub fn solve() {
    let part_one = part_one_iterator::solve();
    let part_two = part_two::solve();

    dbg!(part_one);
    dbg!(part_two);

    assert_eq!(part_one, 5_171_061_464_548);
    assert_eq!(part_two, 10_189_959_087_258);
}

pub mod part_one_iterator {
    use crate::day6::Operator;

    pub fn solve() -> usize {
        let data = include_str!("../data/day6.txt");

        let (nums, ops) = parse(data);

        nums.iter()
            .enumerate()
            .map(|(idx, vals)| match ops[idx] {
                Operator::Multiplication => vals.iter().fold(1, |acc, num| acc * num),
                Operator::Addition => vals.iter().sum(),
            })
            .sum()
    }

    fn parse(data: &str) -> (Vec<Vec<usize>>, Vec<Operator>) {
        let mut nums: Vec<Vec<_>> = Vec::new();

        let mut lines = data.lines();

        let ops = lines
            .next_back()
            .unwrap()
            .split_whitespace()
            .map(|op| match op {
                "*" => Operator::Multiplication,
                "+" => Operator::Addition,
                _ => unreachable!(),
            })
            .collect();

        lines.for_each(|line| {
            line.split_whitespace()
                .enumerate()
                .for_each(|(i, num)| match nums.get_mut(i) {
                    Some(col) => col.push(num.parse().unwrap()),
                    None => nums.push(vec![num.parse().unwrap()]),
                })
        });

        (nums, ops)
    }
}

pub mod part_two {
    use crate::day6::Operator;

    pub fn solve() -> usize {
        let data = include_str!("../data/day6.txt");

        let (nums, ops) = parse(data);

        nums.iter()
            .enumerate()
            .map(|(idx, vals)| match ops[idx] {
                Operator::Multiplication => vals.iter().fold(1, |acc, num| acc * num),
                Operator::Addition => vals.iter().sum(),
            })
            .sum()
    }

    fn parse(data: &str) -> (Vec<Vec<usize>>, Vec<Operator>) {
        let mut lines = data.lines();

        let mut column_start_idx = Vec::new();
        let mut ops = Vec::new();

        for (idx, byte) in lines.next_back().unwrap().bytes().enumerate() {
            match byte {
                b'*' => {
                    ops.push(Operator::Multiplication);
                    column_start_idx.push(idx);
                }
                b'+' => {
                    ops.push(Operator::Addition);
                    column_start_idx.push(idx);
                }
                _ => {}
            }
        }

        let mut columns: Vec<Vec<Vec<usize>>> = vec![Vec::new(); ops.len()];

        for line in lines {
            let line = line.as_bytes();

            let mut column_idx = 0;
            let mut column_digit_idx = 0;
            let mut line_idx = 0;

            'line: while line_idx < line.len() {
                match line[line_idx] {
                    b' ' => {
                        column_digit_idx += 1;
                        line_idx += 1;
                    }
                    _ => {
                        loop {
                            while columns[column_idx].len() <= column_digit_idx {
                                columns[column_idx].push(Vec::new());
                            }

                            columns[column_idx][column_digit_idx]
                                .push((line[line_idx] - b'0') as usize);

                            column_digit_idx += 1;
                            line_idx += 1;

                            if let Some(b' ') = line.get(line_idx) {
                                if let Some(next_line_idx) = column_start_idx.get(column_idx + 1) {
                                    column_idx += 1;
                                    column_digit_idx = 0;
                                    line_idx = *next_line_idx; // Skipping to next number
                                    break;
                                } else {
                                    break 'line; // end of line reached
                                }
                            }

                            if line_idx >= line.len() {
                                break;
                            }
                        }
                    }
                }
            }
        }

        let nums = columns
            .iter()
            .map(|column| {
                column
                    .iter()
                    .map(|col_nums| col_nums.iter().fold(0, |acc, num| acc * 10 + *num))
                    .collect()
            })
            .collect();

        (nums, ops)
    }

    // 123  423
    //  12  56
    //   4  78540
    //
    // column 0
    //  a   b   c   d
    //  1   2   3   -
    //  -   1   2   -
    //  -   -   4   -
    //
    // column 1
    //  a   b   c   d   e
    //  4   2   3   -   -
    //  5   6   -   -   -
    //  7   8   5   4   0
    //
    // columns: Vec<Column>
    // Column:= Vec<Vec<Option<usize>>>
    //
    //  let column_idx = 0
    //
    //  for each line
    //      line_idx = 0
    //      column_digit_idx = 0
    //
    //
    //      while line_idx < line_len
    //          match c at line[line_idx]
    //              ' ' =>  column_digit_idx + 1
    //                      line_idx + 1
    //              digit => loop
    //                          append digit to columns[column_idx][column_digit_idx]
    //                          column_digit_idx + 1
    //                          line_idx + 1
    //
    //                          if line[line_idx] == ' '
    //                              column_digit_idx = 0
    //                              line_idx + 1
    //                              break
    //          column_idx + 1
    //
}
