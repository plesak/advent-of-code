use ndarray::Array2;
use std::collections::HashSet;
use std::fmt::Display;

fn inp_to_array2(input: &str) -> (usize, usize, Array2<char>) {
    let inp = input.lines().collect::<Vec<&str>>();
    let nrows = inp.len();
    let ncols = inp[0].len();
    let flat_vec: Vec<char> = inp.concat().chars().collect();
    (nrows, ncols, Array2::from_shape_vec((nrows, ncols), flat_vec).unwrap())
}

pub fn part1(inp: &str) -> impl Display {
    let (nrows, ncols, grid) = inp_to_array2(inp);

    let mut checked: HashSet<(usize, usize)> = HashSet::new();
    let mut total_fence_needed = 0;

    for i in 0..nrows {
        for j in 0..ncols {
            if !checked.contains(&(i, j)) {
                // start a new region calculation
                let c = grid[[i, j]];

                let mut curr_region_unchecked: Vec<(usize, usize)> = vec![(i, j)];

                let mut region_area = 0;
                let mut region_boundary = 0;

                while let Some((x, y)) = curr_region_unchecked.pop() {
                    // this check is needed bcs the same location could be "discovered" twice
                    // before being checked itself
                    if !checked.contains(&(x, y)) {
                        checked.insert((x, y));
                        let mut same_region_neighbors = 0;

                        // check east
                        if y < ncols - 1 {
                            if grid[[x, y + 1]] == c {
                                same_region_neighbors += 1;
                                if !checked.contains(&(x, y + 1)) {
                                    curr_region_unchecked.push((x, y + 1));
                                }
                            }
                        }
                        // check south
                        if x < nrows - 1 {
                            if grid[[x + 1, y]] == c {
                                same_region_neighbors += 1;
                                if !checked.contains(&(x + 1, y)) {
                                    curr_region_unchecked.push((x + 1, y));
                                }
                            }
                        }
                        // check west
                        if y > 0 {
                            if grid[[x, y - 1]] == c {
                                same_region_neighbors += 1;
                                if !checked.contains(&(x, y - 1)) {
                                    curr_region_unchecked.push((x, y - 1));
                                }
                            }
                        }
                        // check north
                        if x > 0 {
                            if grid[[x - 1, y]] == c {
                                same_region_neighbors += 1;
                                if !checked.contains(&(x - 1, y)) {
                                    curr_region_unchecked.push((x - 1, y));
                                }
                            }
                        }

                        region_area += 1;
                        region_boundary += 4 - same_region_neighbors;
                    }
                }
                // for each region add area * perimeter
                // println!("fence for region {} at {:?} needed for area {}, perimeter {}",
                // c, (i, j), region_area, region_boundary);
                total_fence_needed += region_area * region_boundary;
            }
        }
    }
    total_fence_needed
}

pub fn part2(inp: &str) -> impl Display {
    let (nrows, ncols, grid) = inp_to_array2(inp);

    let mut checked: HashSet<(usize, usize)> = HashSet::new();
    let mut total_fence_needed = 0;

    for i in 0..nrows {
        for j in 0..ncols {
            if !checked.contains(&(i, j)) {
                // start a new region calculation
                let c = grid[[i, j]];

                let mut curr_region_unchecked: Vec<(usize, usize)> = vec![(i, j)];

                let mut region_area = 0;
                let mut same_region_boundaries: HashSet<(char, i32, i32)> = HashSet::new();

                while let Some((x, y)) = curr_region_unchecked.pop() {
                    // this check is needed bcs the same location could be "discovered" twice
                    // before being checked itself
                    if !checked.contains(&(x, y)) {
                        checked.insert((x, y));
                        // boundaries are either

                        // check east
                        let mut east_elem = false;
                        if y < ncols - 1 {
                            if grid[[x, y + 1]] == c {
                                east_elem = true;
                                if !checked.contains(&(x, y + 1)) {
                                    curr_region_unchecked.push((x, y + 1));
                                }
                            }
                        }
                        if !east_elem {
                            let x32 = x as i32;
                            let y32 = y as i32;
                            same_region_boundaries.insert(('E', x32, y32));
                        }
                        // check south
                        let mut south_elem = false;
                        if x < nrows - 1 {
                            if grid[[x + 1, y]] == c {
                                south_elem = true;
                                if !checked.contains(&(x + 1, y)) {
                                    curr_region_unchecked.push((x + 1, y));
                                }
                            }
                        }
                        if !south_elem {
                            let x32 = x as i32;
                            let y32 = y as i32;
                            same_region_boundaries.insert(('S', x32, y32));
                        }
                        // check west
                        let mut west_elem = false;
                        if y > 0 {
                            if grid[[x, y - 1]] == c {
                                west_elem = true;
                                if !checked.contains(&(x, y - 1)) {
                                    curr_region_unchecked.push((x, y - 1));
                                }
                            }
                        }
                        if !west_elem {
                            let x32 = x as i32;
                            let y32 = y as i32;
                            same_region_boundaries.insert(('W', x32, y32));
                        }
                        // check north
                        let mut north_elem = false;
                        if x > 0 {
                            if grid[[x - 1, y]] == c {
                                north_elem = true;
                                if !checked.contains(&(x - 1, y)) {
                                    curr_region_unchecked.push((x - 1, y));
                                }
                            }
                        }
                        if !north_elem {
                            let x32 = x as i32;
                            let y32 = y as i32;
                            same_region_boundaries.insert(('N', x32, y32));
                        }

                        region_area += 1;
                    }
                }

                let mut num_sides = 0;
                for dir in ['E', 'W'] {
                    let mut dir_walls = same_region_boundaries.iter()
                        .filter_map(|&(d, x, y)| if d == dir { Some((y, x)) } else { None })
                        .collect::<Vec<_>>();
                    dir_walls.sort_unstable();
                    // println!("walls {} are {:?}", dir, dir_walls);
                    let mut prev_col = -1;
                    let mut prev_row = -1;
                    for (y, x) in dir_walls {
                        if !(y == prev_col && x == prev_row + 1) {
                            num_sides += 1;
                        }
                        prev_col = y;
                        prev_row = x;
                    }
                }
                for dir in ['N', 'S'] {
                    let mut dir_walls = same_region_boundaries.iter()
                        .filter_map(|&(d, x, y)| if d == dir { Some((x, y)) } else { None })
                        .collect::<Vec<_>>();
                    dir_walls.sort_unstable();
                    // println!("walls {} are {:?}", dir, dir_walls);
                    let mut prev_col = -1;
                    let mut prev_row = -1;
                    for (x, y) in dir_walls {
                        if !(x == prev_row && y == prev_col + 1) {
                            num_sides += 1;
                        }
                        prev_row = x;
                        prev_col = y;
                    }
                }

                // for each region add area * perimeter
                // println!("fence for region {} at {:?} needed for area {}, perimeter {}",
                //          c, (i, j), region_area, num_sides);
                total_fence_needed += region_area * num_sides;
            }
        }
    }
    total_fence_needed
}