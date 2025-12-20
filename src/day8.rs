use std::fmt::Debug;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct JunctionBox(usize, usize, usize);

impl Debug for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}|{}|{})", self.0, self.1, self.2)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Distance {
    junction_box_a: JunctionBox,
    junction_box_b: JunctionBox,
    distance: usize,
}

impl Debug for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:\t{:?}\t<->\t{:?})",
            self.distance, self.junction_box_a, self.junction_box_b,
        )
    }
}

pub fn solve() {
    let part_one = part_one::solve();
    let part_two = part_two::solve();

    dbg!(part_one);
    dbg!(part_two);

    assert_eq!(part_one, 66_640);
    assert_eq!(part_two, 78_894_156);
}

pub mod part_one {
    use crate::day8::{Distance, JunctionBox};

    pub fn solve() -> usize {
        let data = include_str!("../data/day8.txt");

        let junction_boxes = parse_coords(data);

        let mut all_distances = get_all_distances(&junction_boxes);
        all_distances.sort_by_key(|distance| distance.distance);

        let mut circuits: Vec<Vec<JunctionBox>> = Vec::new();

        for distance in all_distances.first_chunk::<1000>().unwrap() {
            let circuit_a = circuits
                .iter()
                .position(|circuit| circuit.contains(&distance.junction_box_a));
            let circuit_b = circuits
                .iter()
                .position(|circuit| circuit.contains(&distance.junction_box_b));

            match (circuit_a, circuit_b) {
                // both in the same circuit
                (Some(circuit_a_idx), Some(circuit_b_idx)) if circuit_a_idx == circuit_b_idx => {}
                // a and b are in different circuits
                (Some(circuit_a_idx), Some(circuit_b_idx)) => {
                    let mut circuit_b = circuits.swap_remove(circuit_b_idx);
                    circuits[circuit_a_idx].append(&mut circuit_b);
                }
                // a is in a circuit but not b
                (Some(circuit_a_idx), None) => {
                    circuits[circuit_a_idx].push(distance.junction_box_b)
                }
                // b is in a circuit but not a
                (None, Some(circuit_b_idx)) => {
                    circuits[circuit_b_idx].push(distance.junction_box_a)
                }
                // both a and b are not in a circuit
                (None, None) => {
                    circuits.push(vec![distance.junction_box_a, distance.junction_box_b])
                }
            }
        }

        // we dont need to add the single junction box circuits as they have len 1 and would not change the multiplication

        let mut circuit_lens: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
        circuit_lens.sort();

        circuit_lens[circuit_lens.len() - 1]
            * circuit_lens[circuit_lens.len() - 2]
            * circuit_lens[circuit_lens.len() - 3]
    }

    fn parse_coords(data: &str) -> Vec<JunctionBox> {
        data.lines()
            .map(|line| {
                let (x, yz) = line.split_once(',').unwrap();
                let (y, z) = yz.split_once(',').unwrap();
                // println!("{x}|{y}|{z}");
                JunctionBox(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
            })
            .collect()
    }

    fn get_all_distances(junction_boxes: &[JunctionBox]) -> Vec<Distance> {
        let mut distances = Vec::new();

        for i in 0..junction_boxes.len() {
            for j in i + 1..junction_boxes.len() {
                let junction_box_a = junction_boxes[i];
                let junction_box_b = junction_boxes[j];

                let distance = calculate_distance(junction_box_a, junction_box_b);
                distances.push(Distance {
                    junction_box_a,
                    junction_box_b,
                    distance,
                });
            }
        }

        distances
    }

    fn calculate_distance(coord_a: JunctionBox, coord_b: JunctionBox) -> usize {
        ((coord_a.0 - coord_b.0).pow(2)
            + (coord_a.1 - coord_b.1).pow(2)
            + (coord_a.2 - coord_b.2).pow(2))
        .isqrt()
    }
}

pub mod part_two {
    use std::cmp::Ordering;

    use crate::day8::{Distance, JunctionBox};

    pub fn solve() -> usize {
        let data = include_str!("../data/day8.txt");

        let junction_boxes = parse_coords(data);

        let mut all_distances = get_all_distances(&junction_boxes);
        all_distances.sort_by_key(|distance| distance.distance);

        let mut circuits: Vec<Vec<JunctionBox>> = junction_boxes
            .into_iter()
            .map(|junction_box| vec![junction_box])
            .collect();

        let mut distance_to_wall = 0;

        for distance in all_distances {
            let circuit_a = circuits
                .iter()
                .position(|circuit| circuit.contains(&distance.junction_box_a));
            let circuit_b = circuits
                .iter()
                .position(|circuit| circuit.contains(&distance.junction_box_b));

            match (circuit_a, circuit_b) {
                (Some(circuit_a_idx), Some(circuit_b_idx)) => {
                    match circuit_a_idx.cmp(&circuit_b_idx) {
                        // both in the same circuit
                        Ordering::Equal => {}
                        // a and b are in different circuits
                        Ordering::Less => {
                            let (circuit_a_part, circuit_b_part) =
                                circuits.split_at_mut(circuit_a_idx + 1);
                            circuit_a_part[circuit_a_idx].extend_from_slice(
                                &circuit_b_part[circuit_b_idx - circuit_a_idx - 1],
                            );
                            circuits.swap_remove(circuit_b_idx);
                        }
                        Ordering::Greater => {
                            let (circuit_b_part, circuit_a_part) =
                                circuits.split_at_mut(circuit_b_idx + 1);
                            circuit_a_part[circuit_a_idx - circuit_b_idx - 1]
                                .extend(&circuit_b_part[circuit_b_idx]);
                            circuits.swap_remove(circuit_b_idx);
                        }
                    }
                }
                // a is in a circuit but not b
                (Some(circuit_a_idx), None) => {
                    circuits[circuit_a_idx].push(distance.junction_box_b)
                }
                // b is in a circuit but not a
                (None, Some(circuit_b_idx)) => {
                    circuits[circuit_b_idx].push(distance.junction_box_a)
                }
                // both a and b are not in a circuit
                (None, None) => unreachable!("every junction box is in its own circuit to start"),
            }

            // we have reached a complete circuit
            if circuits.len() == 1 {
                distance_to_wall = distance.junction_box_a.0 * distance.junction_box_b.0;
                break;
            }
        }

        // we dont need to add the single junction box circuits as they have len 1 and would not change the multiplication

        // dbg!(&circuits);

        distance_to_wall
    }

    fn parse_coords(data: &str) -> Vec<JunctionBox> {
        data.lines()
            .map(|line| {
                let (x, yz) = line.split_once(',').unwrap();
                let (y, z) = yz.split_once(',').unwrap();
                // println!("{x}|{y}|{z}");
                JunctionBox(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
            })
            .collect()
    }

    fn get_all_distances(junction_boxes: &[JunctionBox]) -> Vec<Distance> {
        let mut distances = Vec::new();

        for i in 0..junction_boxes.len() {
            for j in i + 1..junction_boxes.len() {
                let junction_box_a = junction_boxes[i];
                let junction_box_b = junction_boxes[j];

                let distance = calculate_distance(junction_box_a, junction_box_b);
                distances.push(Distance {
                    junction_box_a,
                    junction_box_b,
                    distance,
                });
            }
        }

        distances
    }

    fn calculate_distance(coord_a: JunctionBox, coord_b: JunctionBox) -> usize {
        ((coord_a.0 - coord_b.0).pow(2)
            + (coord_a.1 - coord_b.1).pow(2)
            + (coord_a.2 - coord_b.2).pow(2))
        .isqrt()
    }
}
