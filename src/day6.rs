use ndarray::Array2;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

fn inp_to_array2(input: Vec<&str>) -> Array2<char> {
    let nrows = input.len();
    let ncols = input[0].len();
    let flat_vec: Vec<char> = input.concat().chars().collect();
    Array2::from_shape_vec((nrows, ncols), flat_vec).unwrap()
}

pub fn part1(inp: &str) -> impl Display {

    // assume we never enter a loop (puzzle would be impossible)

    let mut current_coord: (isize, isize) = (0, 0);
    let mut current_dir: char = 'X';

    // eg row 8 has obstacles at cols 6 9 15 21 --> (8, [6, 9, 15, 21])
    let mut obstacles_by_row: HashMap<isize, Vec<isize>> = HashMap::new();
    let mut obstacles_by_col: HashMap<isize, Vec<isize>> = HashMap::new();

    // keep track of spaces already stepped on
    let mut stepped: HashSet<(isize, isize)> = HashSet::new();

    let nrows = inp.lines().count() as isize;
    let ncols = inp.lines().next().unwrap().len() as isize;

    for (i, ln) in inp.lines().enumerate() {
        for (j, c) in ln.chars().enumerate() {
            let starting_point = match c {
                '#' => {
                    obstacles_by_row.entry(i as isize).or_default().push(j as isize);
                    obstacles_by_col.entry(j as isize).or_default().push(i as isize);
                    false
                }
                '^' => {
                    current_dir = '^';
                    true
                }
                '>' => {
                    current_dir = '>';
                    true
                }
                'v' => {
                    current_dir = 'v';
                    true
                }
                '<' => {
                    current_dir = '<';
                    true
                }
                _ => { false }
            };
            if starting_point {
                current_coord = (i as isize, j as isize);
                stepped.insert(current_coord);
            }
        }
    }

    let mut in_grid = true;
    while in_grid {
        let x = current_coord.0;
        let y = current_coord.1;
        match &current_dir {
            '^' => {
                let boundary_top = obstacles_by_col.entry(y).or_default()
                    .iter().filter(|&&obs_x| obs_x < x).copied()
                    .max().unwrap_or(-1);
                for i in (boundary_top + 1..=x).rev() {
                    stepped.insert((i, y));
                    if i == 0 {
                        in_grid = false;
                    }
                }
                current_coord = (boundary_top + 1, y);
                current_dir = '>';
            }
            '>' => {
                let boundary_right = obstacles_by_row.entry(x).or_default()
                    .iter().filter(|&&obs_y| obs_y > y).copied()
                    .min().unwrap_or(ncols);
                for i in y..boundary_right {
                    stepped.insert((x, i));
                    if i == ncols - 1 {
                        in_grid = false;
                    }
                }
                current_coord = (x, boundary_right - 1);
                current_dir = 'v';
            }
            'v' => {
                let boundary_bottom = obstacles_by_col.entry(y).or_default()
                    .iter().filter(|&&obs_x| obs_x > x).copied()
                    .min().unwrap_or(nrows);
                for i in x..boundary_bottom {
                    stepped.insert((i, y));
                    if i == nrows - 1 {
                        in_grid = false;
                    }
                }
                current_coord = (boundary_bottom - 1, y);
                current_dir = '<';
            }
            '<' => {
                let boundary_left = obstacles_by_row.entry(x).or_default()
                    .iter().filter(|&&obs_y| obs_y < y).copied()
                    .max().unwrap_or(-1);
                for i in (boundary_left + 1..=y).rev() {
                    stepped.insert((x, i));
                    if i == 0 {
                        in_grid = false;
                    }
                }
                current_coord = (x, boundary_left + 1);
                current_dir = '^';
            }
            _ => panic!("unknown dir {}", current_dir),
        }
    }
    stepped.iter().count()
}

fn determine_if_grid_loops(
    mut stepped: HashMap<(isize, isize), HashSet<char>>,
    mut current_coord: (isize, isize),
    mut obstacles_by_row: HashMap<isize, Vec<isize>>,
    mut obstacles_by_col: HashMap<isize, Vec<isize>>,
    nrows: isize,
    ncols: isize,
    mut current_dir: char,
) -> bool {

    // println!("\nstarting a loop check!! inputs:");
    // println!("stepped: {:?}", stepped);
    // println!("current_coord: {:?}", current_coord);
    // println!("obstacles_by_row: {:?}", obstacles_by_row);
    // println!("obstacles_by_col: {:?}", obstacles_by_col);
    // println!("current_dir: {:?}", current_dir);

    let mut in_grid = true;
    while in_grid {
        let x = current_coord.0;
        let y = current_coord.1;
        match &current_dir {
            '^' => {
                let boundary_top = obstacles_by_col.entry(y).or_default()
                    .iter().filter(|&&obs_x| obs_x < x).copied()
                    .max().unwrap_or(-1);
                for i in (boundary_top + 1..=x).rev() {
                    if i == 0 {
                        return false;
                    }
                    match stepped.entry((i, y)) {
                        std::collections::hash_map::Entry::Occupied(
                            mut entry
                        ) => {
                            let set = entry.get_mut();
                            if !set.contains(&current_dir) {
                                set.insert(current_dir);
                            } else {
                                return true;
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(
                            entry
                        ) => {
                            let mut set = HashSet::new();
                            set.insert(current_dir);
                            entry.insert(set);
                        }
                    }
                }
                current_coord = (boundary_top + 1, y);
                current_dir = '>';
            }
            '>' => {
                let boundary_right = obstacles_by_row.entry(x).or_default()
                    .iter().filter(|&&obs_y| obs_y > y).copied()
                    .min().unwrap_or(ncols);
                for i in y..boundary_right {
                    if i == ncols - 1 {
                        return false;
                    }
                    match stepped.entry((x, i)) {
                        std::collections::hash_map::Entry::Occupied(
                            mut entry
                        ) => {
                            let set = entry.get_mut();
                            if !set.contains(&current_dir) {
                                set.insert(current_dir);
                            } else {
                                return true;
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(
                            entry
                        ) => {
                            let mut set = HashSet::new();
                            set.insert(current_dir);
                            entry.insert(set);
                        }
                    }
                }
                current_coord = (x, boundary_right - 1);
                current_dir = 'v';
            }
            'v' => {
                let boundary_bottom = obstacles_by_col.entry(y).or_default()
                    .iter().filter(|&&obs_x| obs_x > x).copied()
                    .min().unwrap_or(nrows);
                for i in x..boundary_bottom {
                    if i == nrows - 1 {
                        return false;
                    }
                    match stepped.entry((i, y)) {
                        std::collections::hash_map::Entry::Occupied(
                            mut entry
                        ) => {
                            let set = entry.get_mut();
                            if !set.contains(&current_dir) {
                                set.insert(current_dir);
                            } else {
                                return true;
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(
                            entry
                        ) => {
                            let mut set = HashSet::new();
                            set.insert(current_dir);
                            entry.insert(set);
                        }
                    }
                }
                current_coord = (boundary_bottom - 1, y);
                current_dir = '<';
            }
            '<' => {
                let boundary_left = obstacles_by_row.entry(x).or_default()
                    .iter().filter(|&&obs_y| obs_y < y).copied()
                    .max().unwrap_or(-1);
                for i in (boundary_left + 1..=y).rev() {
                    if i == 0 {
                        return false;
                    }
                    match stepped.entry((x, i)) {
                        std::collections::hash_map::Entry::Occupied(
                            mut entry
                        ) => {
                            let set = entry.get_mut();
                            if !set.contains(&current_dir) {
                                set.insert(current_dir);
                            } else {
                                return true;
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(
                            entry
                        ) => {
                            let mut set = HashSet::new();
                            set.insert(current_dir);
                            entry.insert(set);
                        }
                    }
                }
                current_coord = (x, boundary_left + 1);
                current_dir = '^';
            }
            _ => panic!("unknown dir {}", current_dir),
        }
    }
    false
}

pub fn part2(inp: &str) -> impl Display {
    // assume we never enter a loop (puzzle would be impossible)

    let mut current_coord: (isize, isize) = (0, 0);
    let mut current_dir: char = 'X';

    // eg row 8 has obstacles at cols 6 9 15 21 --> (8, [6, 9, 15, 21])
    let mut obstacles_by_row: HashMap<isize, Vec<isize>> = HashMap::new();
    let mut obstacles_by_col: HashMap<isize, Vec<isize>> = HashMap::new();

    // keep track of spaces already stepped on
    let mut stepped: HashMap<(isize, isize), HashSet<char>> = HashMap::new();

    let nrows = inp.lines().count() as isize;
    let ncols = inp.lines().next().unwrap().len() as isize;

    for (i, ln) in inp.lines().enumerate() {
        for (j, c) in ln.chars().enumerate() {
            let starting_point = match c {
                '#' => {
                    obstacles_by_row.entry(i as isize).or_default().push(j as isize);
                    obstacles_by_col.entry(j as isize).or_default().push(i as isize);
                    false
                }
                '^' => {
                    current_dir = '^';
                    true
                }
                '>' => {
                    current_dir = '>';
                    true
                }
                'v' => {
                    current_dir = 'v';
                    true
                }
                '<' => {
                    current_dir = '<';
                    true
                }
                _ => { false }
            };
            if starting_point {
                current_coord = (i as isize, j as isize);
                let mut new_set = HashSet::new();
                new_set.insert(current_dir);
                stepped.insert(current_coord, new_set);
            }
        }
    }

    let mut loops_tested: HashSet<(isize, isize)> = HashSet::new();
    let mut loops_count: isize = 0;

    let mut in_grid = true;
    while in_grid {
        let x = current_coord.0;
        let y = current_coord.1;
        match &current_dir {
            '^' => {
                let boundary_top = obstacles_by_col.entry(y).or_default()
                    .iter().filter(|&&obs_x| obs_x < x).copied()
                    .max().unwrap_or(-1);
                for i in (boundary_top + 1..x).rev() {
                    // put an obstacle here and try to see if things loop
                    if !loops_tested.contains(&(i, y)) {
                        loops_tested.insert((i, y));
                        let mut obstacles_by_row_clone = obstacles_by_row.clone();
                        obstacles_by_row_clone.entry(i).or_default().push(y);
                        let mut obstacles_by_col_clone = obstacles_by_col.clone();
                        obstacles_by_col_clone.entry(y).or_default().push(i);
                        if determine_if_grid_loops(
                            stepped.clone(),
                            current_coord.clone(),
                            obstacles_by_row_clone,
                            obstacles_by_col_clone,
                            nrows,
                            ncols,
                            '>',
                        ) {
                            // println!("grid loops! extra obstacle at {:?}", (i, y));
                            loops_count += 1;
                        }
                    }
                    current_coord = (i, y);

                    match stepped.entry((i, y)) {
                        std::collections::hash_map::Entry::Occupied(
                            mut entry
                        ) => {
                            let set = entry.get_mut();
                            if !set.contains(&current_dir) {
                                set.insert(current_dir);
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(
                            entry
                        ) => {
                            let mut set = HashSet::new();
                            set.insert(current_dir);
                            entry.insert(set);
                        }
                    }
                    if i == 0 {
                        in_grid = false;
                    }
                }
                current_coord = (boundary_top + 1, y);
                current_dir = '>';
            }
            '>' => {
                let boundary_right = obstacles_by_row.entry(x).or_default()
                    .iter().filter(|&&obs_y| obs_y > y).copied()
                    .min().unwrap_or(ncols);
                for i in y + 1..boundary_right {
                    // put an obstacle here and try to see if things loop
                    if !loops_tested.contains(&(x, i)) {
                        loops_tested.insert((x, i));
                        let mut obstacles_by_row_clone = obstacles_by_row.clone();
                        obstacles_by_row_clone.entry(x).or_default().push(i);
                        let mut obstacles_by_col_clone = obstacles_by_col.clone();
                        obstacles_by_col_clone.entry(i).or_default().push(x);
                        if determine_if_grid_loops(
                            stepped.clone(),
                            current_coord.clone(),
                            obstacles_by_row_clone,
                            obstacles_by_col_clone,
                            nrows,
                            ncols,
                            'v',
                        ) {
                            // println!("grid loops! extra obstacle at {:?}", (x, i));
                            loops_count += 1;
                        }
                    }
                    current_coord = (x, i);

                    match stepped.entry((x, i)) {
                        std::collections::hash_map::Entry::Occupied(
                            mut entry
                        ) => {
                            let set = entry.get_mut();
                            if !set.contains(&current_dir) {
                                set.insert(current_dir);
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(
                            entry
                        ) => {
                            let mut set = HashSet::new();
                            set.insert(current_dir);
                            entry.insert(set);
                        }
                    }
                    if i == ncols - 1 {
                        in_grid = false;
                    }
                }
                current_coord = (x, boundary_right - 1);
                current_dir = 'v';
            }
            'v' => {
                let boundary_bottom = obstacles_by_col.entry(y).or_default()
                    .iter().filter(|&&obs_x| obs_x > x).copied()
                    .min().unwrap_or(nrows);
                for i in x + 1..boundary_bottom {
                    // put an obstacle here and try to see if things loop
                    if !loops_tested.contains(&(i, y)) {
                        loops_tested.insert((i, y));
                        let mut obstacles_by_row_clone = obstacles_by_row.clone();
                        obstacles_by_row_clone.entry(i).or_default().push(y);
                        let mut obstacles_by_col_clone = obstacles_by_col.clone();
                        obstacles_by_col_clone.entry(y).or_default().push(i);
                        if determine_if_grid_loops(
                            stepped.clone(),
                            current_coord.clone(),
                            obstacles_by_row_clone,
                            obstacles_by_col_clone,
                            nrows,
                            ncols,
                            '<',
                        ) {
                            // println!("grid loops! extra obstacle at {:?}", (i, y));
                            loops_count += 1;
                        }
                    }
                    current_coord = (i, y);

                    match stepped.entry((i, y)) {
                        std::collections::hash_map::Entry::Occupied(
                            mut entry
                        ) => {
                            let set = entry.get_mut();
                            if !set.contains(&current_dir) {
                                set.insert(current_dir);
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(
                            entry
                        ) => {
                            let mut set = HashSet::new();
                            set.insert(current_dir);
                            entry.insert(set);
                        }
                    }
                    if i == nrows - 1 {
                        in_grid = false;
                    }
                }
                current_coord = (boundary_bottom - 1, y);
                current_dir = '<';
            }
            '<' => {
                let boundary_left = obstacles_by_row.entry(x).or_default()
                    .iter().filter(|&&obs_y| obs_y < y).copied()
                    .max().unwrap_or(-1);
                for i in (boundary_left + 1..y).rev() {
                    // put an obstacle here and try to see if things loop
                    if !loops_tested.contains(&(x, i)) {
                        loops_tested.insert((x, i));
                        let mut obstacles_by_row_clone = obstacles_by_row.clone();
                        obstacles_by_row_clone.entry(x).or_default().push(i);
                        let mut obstacles_by_col_clone = obstacles_by_col.clone();
                        obstacles_by_col_clone.entry(i).or_default().push(x);
                        if determine_if_grid_loops(
                            stepped.clone(),
                            current_coord.clone(),
                            obstacles_by_row_clone,
                            obstacles_by_col_clone,
                            nrows,
                            ncols,
                            '^',
                        ) {
                            // println!("grid loops! extra obstacle at {:?}", (x, i));
                            loops_count += 1;
                        }
                    }
                    current_coord = (x, i);

                    match stepped.entry((x, i)) {
                        std::collections::hash_map::Entry::Occupied(
                            mut entry
                        ) => {
                            let set = entry.get_mut();
                            if !set.contains(&current_dir) {
                                set.insert(current_dir);
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(
                            entry
                        ) => {
                            let mut set = HashSet::new();
                            set.insert(current_dir);
                            entry.insert(set);
                        }
                    }
                    if i == 0 {
                        in_grid = false;
                    }
                }
                current_coord = (x, boundary_left + 1);
                current_dir = '^';
            }
            _ => panic!("unknown dir {}", current_dir),
        }
    }
    loops_count
}