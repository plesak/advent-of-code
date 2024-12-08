use std::collections::{HashMap, HashSet};
use std::fmt::Display;


pub fn part1(inp: &str) -> impl Display {
    let mut interferences: HashSet<isize> = HashSet::new();
    let mut antennas: HashMap<char, Vec<isize>> = HashMap::new();
    let no_newline = inp.chars()
        .filter(|c| !['\n', '\r'].contains(&c)).collect::<String>();
    // assume square grid
    let n = inp.lines().count() as isize;
    // println!("len is {}, n is {}", total, n);

    fn coords_from_x(i: isize, n: isize) -> (isize, isize) {
        (i / n, i % n)
    }

    for (i, c) in no_newline.chars().enumerate() {
        let coords = coords_from_x(i as isize, n);
        if c != '.' {
            // println!("found antenna {} at {:?}", c, coords);
            if antennas.contains_key(&c) {
                let ls = antennas.get(&c).unwrap();
                for &y in ls {
                    let antenna_coords = coords_from_x(y, n);
                    let sub_x = 2 * antenna_coords.0 - coords.0;
                    let sub_y = 2 * antenna_coords.1 - coords.1;
                    if sub_x >= 0 && sub_y >= 0 && sub_y < n {
                        // println!("adding interference at {}, {}", sub_x, sub_y);
                        interferences.insert(2 * y - (i as isize));
                    }
                    let over_x = 2 * coords.0 - antenna_coords.0;
                    let over_y = 2 * coords.1 - antenna_coords.1;
                    if over_x < n && over_y < n && over_y >= 0 {
                        // println!("adding interference at {}, {}", over_x, over_y);
                        interferences.insert(2 * (i as isize) - y);
                    }
                }
            }
            antennas.entry(c).or_insert_with(Vec::new).push(i as isize);
            // println!("interferences found so far {:?}", interferences);
        }
    }
    interferences.len()
}

pub fn part2(inp: &str) -> impl Display {
    let mut interferences: HashSet<isize> = HashSet::new();
    let mut antennas: HashMap<char, Vec<isize>> = HashMap::new();
    let no_newline = inp.chars()
        .filter(|c| !['\n', '\r'].contains(&c)).collect::<String>();
    // assume square grid
    let n = inp.lines().count() as isize;
    // println!("len is {}, n is {}", total, n);

    fn coords_from_x(i: isize, n: isize) -> (isize, isize) {
        (i / n, i % n)
    }

    for (i, c) in no_newline.chars().enumerate() {
        let coords = coords_from_x(i as isize, n);
        if c != '.' {
            // println!("found antenna {} at {:?}", c, coords);
            if antennas.contains_key(&c) {
                let ls = antennas.get(&c).unwrap();
                for &y in ls {
                    let antenna_coords = coords_from_x(y, n);
                    if antenna_coords.0 == coords.0 {
                        // special case same row
                        for col in 0..n {
                            interferences.insert(antenna_coords.0 + col);
                        }
                    } else if antenna_coords.1 == coords.1 {
                        // special case same column
                        for row in 0..n {
                            interferences.insert(row + antenna_coords.1);
                        }
                    } else {
                        // get slope
                        let row_diff = coords.0 - antenna_coords.0;
                        let col_diff = coords.1 - antenna_coords.1;
                        let mut mutable_x = antenna_coords.0;
                        let mut mutable_y = antenna_coords.1;
                        // first go from earlier antenna backwards
                        for _ in 0..antenna_coords.0 {
                            mutable_x -= row_diff;
                            mutable_y -= col_diff;
                            if mutable_x >= 0 && mutable_y >= 0 && mutable_y < n {
                                // println!("  adding interference at {}, {}", mutable_x, mutable_y);
                                interferences.insert(n * mutable_x + mutable_y);
                            } else {
                                break;
                            }
                        }
                        // then consider earlier antenna point itself
                        mutable_x = antenna_coords.0;
                        mutable_y = antenna_coords.1;
                        // println!(" adding interference at {}, {}", mutable_x, mutable_y);
                        interferences.insert(n * mutable_x + mutable_y);
                        // then go from earlier antenna point onwards
                        for _ in antenna_coords.0..n {
                            mutable_x += row_diff;
                            mutable_y += col_diff;
                            if mutable_x < n && mutable_y >= 0 && mutable_y < n {
                                // println!("  adding interference at {}, {}", mutable_x, mutable_y);
                                interferences.insert(n * mutable_x + mutable_y);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            antennas.entry(c).or_insert_with(Vec::new).push(i as isize);
            // println!("interferences found so far {}", interferences.len());
        }
    }
    interferences.len()
}