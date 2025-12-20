#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Segment {
    start: Coordinate,
    end: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

pub fn solve() {
    let part_one = part_one::solve();
    let part_two = part_two::solve();

    dbg!(part_one);
    dbg!(part_two);

    assert_eq!(part_one, 4_777_967_538);
    assert_eq!(part_two, 42);
}

pub mod part_one {
    use std::usize;

    pub fn solve() -> usize {
        let data = include_str!("../data/day9.txt");

        let coords: Vec<(usize, usize)> = data
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        let mut max_area = usize::MIN;

        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let a = coords[i];
                let b = coords[j];

                let width = if a.0 > b.0 {
                    a.0 - b.0 + 1
                } else {
                    b.0 - a.0 + 1
                };
                let height = if a.1 > b.1 {
                    a.1 - b.1 + 1
                } else {
                    b.1 - a.1 + 1
                };

                let area = width * height;

                max_area = max_area.max(area);
            }
        }

        max_area
    }
}

// TODO: New solution: check whether any segment (thats not the border) intersects with rectangle -> not complete
// OR: Map complete space into 2D Array with lookup by using coordinate compression
pub mod part_two {
    use std::collections::HashSet;

    use crate::day9::{Coordinate, Segment};

    pub fn solve() -> usize {
        let data = include_str!("../data/day9.txt");
        let mut red_tiles: Vec<Coordinate> = data
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                Coordinate {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect();

        let mut segments: HashSet<Segment> = HashSet::new();

        for tiles in red_tiles.windows(2) {
            segments.insert(Segment {
                start: tiles[0],
                end: tiles[1],
            });
        }

        // Add the segment wrapping around
        {
            let start = *red_tiles.last().unwrap();
            let end = red_tiles[0];

            segments.insert(Segment { start, end });
        }

        red_tiles.sort();

        // get all possible areas in order
        let mut areas = Vec::new();

        for i in 0..red_tiles.len() {
            for j in i + 1..red_tiles.len() {
                let a = red_tiles[i];
                let b = red_tiles[j];

                // Ordered therefore always a.x <= b.x
                let width = b.x - a.x + 1;

                // Ordering of .y is dependend on .x so this check is still needed
                let height = if a.y > b.y {
                    a.y - b.y + 1
                } else {
                    b.y - a.y + 1
                };

                areas.push((width * height, (a, b)));
            }
        }

        // sort areas by size
        areas.sort_by(|(area_a, _), (area_b, _)| area_a.cmp(area_b));

        'area: for (area, (a, b)) in areas.iter().rev() {
            // check if area only contains green and red tiles
            // yes -> return this area

            // for all points in area
            // ray-cast-segment intersection checks with all segments
            // if num of intersections odd -> inside, even -> outside
            // if any outside continue with next area

            // dbg!(area, a, b);

            let j_range = if a.y < b.y {
                (a.y)..=(b.y)
            } else {
                (b.y)..=(a.y)
            };

            // areas is ordered therefore always a.x <= b.y
            for i in (a.x)..=(b.x) {
                for j in j_range.clone() {
                    if outside(&segments, Coordinate { x: i, y: j }) {
                        // println!("Outside: {i}|{j}");
                        continue 'area;
                    }
                }
            }

            return *area;
        }

        unreachable!()
    }

    fn outside(segments: &HashSet<Segment>, coord: Coordinate) -> bool {
        // let mut total_intersections = 0usize;

        // for segment in segments {
        //     // y range check
        //     if coord.y >= segment.start.y.min(segment.end.y)
        //         && coord.y <= segment.start.y.max(segment.end.y)
        //     {
        //         // on segment check
        //         if coord.x >= segment.start.x.min(segment.end.x)
        //             && coord.x <= segment.start.x.max(segment.end.x)
        //         {
        //             // println!("on segment: {coord:?}");
        //             return false;
        //         }

        //         // ignore horizontal segments
        //         if segment.start.y != segment.end.y {
        //             // raycast
        //             let intersection_x = segment.start.x
        //                 + (segment.end.x - segment.start.x) * (coord.y - segment.start.y)
        //                     / (segment.end.y - segment.start.y);

        //             // dbg!(intersection_x);

        //             // coord lies on the left of the segment
        //             if intersection_x >= coord.x {
        //                 total_intersections += 1;
        //             }
        //         }
        //     }

        //     // dbg!(total_intersections);
        // }

        // // even number of intersections means that the point is outside the polygon
        // // therefore the area is not complete
        // return total_intersections.is_multiple_of(2);

        // let mut inside = false;
        // for seg in segments {
        //     // Only consider segments that span the Y-level of our coordinate
        //     let (y1, y2) = (seg.start.y as f64, seg.end.y as f64);
        //     let (x1, x2) = (seg.start.x as f64, seg.end.x as f64);
        //     let py = coord.y as f64;
        //     let px = coord.x as f64;

        //     if ((y1 > py) != (y2 > py)) && (px < (x2 - x1) * (py - y1) / (y2 - y1) + x1) {
        //         inside = !inside;
        //     }
        // }
        // !inside
    }
}
