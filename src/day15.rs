use crate::utils;
use ndarray::{s, Array2, ArrayViewMut1};
use std::collections::HashSet;
use std::fmt::Display;

fn move_forward(line: &mut ArrayViewMut1<char>) -> usize {
    // println!("pushing line {:?}", line);
    let mut end = 'X';
    let pos = line.iter().position(|&c| c == '@').unwrap();
    let mut cntr: usize = 0;

    while end == 'X' {
        match line[pos + cntr + 1] {
            '.' => {
                end = '.';
                cntr += 1;
            }
            '#' => end = '#',
            'O' => cntr += 1,
            _ => panic!("unknown cell content! {}", line[pos + cntr + 1])
        }
    }

    if cntr > 0 && end == '.' {
        line[pos] = '.';
        line[pos + 1] = '@';
        if cntr > 1 {
            line[pos + cntr] = 'O';
        }
        return 1;
    }
    0
}

pub fn part1(inp: &str) -> impl Display {
    let (_nrows, _ncols, mut grid) = utils::inp_to_array2_char(
        &*inp.lines().take_while(|l| !l.is_empty()).collect::<Vec<&str>>().join("\n")
    );

    let dirs = inp.chars()
        .filter(|c| ['>', 'v', '<', '^'].contains(c)).collect::<String>();

    let mut coords: (usize, usize) = (9999, 9999);
    if let Some(((row, col), _)) = grid.indexed_iter()
        .find(|(_, &value)| value == '@') {
        coords = (row, col);
    }

    for c in dirs.chars() {
        match c {
            '>' => coords.1 += move_forward(
                &mut grid.row_mut(coords.0)),
            'v' => coords.0 += move_forward(
                &mut grid.column_mut(coords.1)),
            '<' => coords.1 -= move_forward(
                &mut grid.row_mut(coords.0).slice_mut(s![..;-1])),
            '^' => coords.0 -= move_forward(
                &mut grid.column_mut(coords.1).slice_mut(s![..;-1])),
            _ => panic!("unknown direction! {}", c)
        }
    }
    grid.indexed_iter().filter(|(_, &v)| v == 'O')
        .map(|((r, c), _)| 100 * r + c).sum::<usize>()
}

fn move_horz(line: &mut ArrayViewMut1<char>) -> usize {
    let mut end = 'X';
    let pos = line.iter().position(|&c| c == '@').unwrap();
    let mut cntr: usize = 0;

    while end == 'X' {
        match line[pos + cntr + 1] {
            '.' => {
                end = '.';
                cntr += 1;
            }
            '#' => end = '#',
            '[' | ']' => cntr += 2,
            _ => panic!("unknown cell content! {}", line[pos + cntr + 1])
        }
    }

    if cntr > 0 && end == '.' {
        for ix in (pos + 2..pos + cntr + 1).rev() {
            match line[ix - 1] {
                '[' => line[ix] = '[',
                ']' => line[ix] = ']',
                _ => {}
            }
        }
        line[pos] = '.';
        line[pos + 1] = '@';
        return 1;
    }
    0
}

fn move_down(grid: &mut Array2<char>, coords: (usize, usize)) -> usize {
    let mut end = 'X';

    let mut front: HashSet<(usize, usize)> = HashSet::from([coords]);
    let mut to_move: HashSet<(usize, usize)> = HashSet::from([coords]);
    while end == 'X' {
        let mut new_front: HashSet<(usize, usize)> = HashSet::new();
        for &(row, col) in &front {
            match grid[[row + 1, col]] {
                '#' => {
                    new_front.insert((row + 1, col));
                    end = '#';
                    break;
                }
                '.' => {}
                '[' => {
                    new_front.insert((row + 1, col));
                    to_move.insert((row + 1, col));
                    new_front.insert((row + 1, col + 1));
                    to_move.insert((row + 1, col + 1));
                }
                ']' => {
                    new_front.insert((row + 1, col));
                    to_move.insert((row + 1, col));
                    new_front.insert((row + 1, col - 1));
                    to_move.insert((row + 1, col - 1));
                }
                _ => panic!("unknown cell content! {}", grid[[row + 1, col]])
            }
        }
        if new_front.iter().all(|&(r, c)| grid[[r, c]] == '.') {
            end = '.';
        } else {
            front = new_front;
        }
    }

    if end == '.' {
        let mut to_move_vec = to_move.iter().collect::<Vec<&(usize, usize)>>();
        to_move_vec.sort_unstable();
        to_move_vec.reverse();
        for &(r, c) in to_move_vec {
            let chr = grid[[r, c]];
            grid[[r, c]] = '.';
            grid[[r + 1, c]] = chr;
        }
        return 1;
    }
    0
}

fn move_up(grid: &mut Array2<char>, coords: (usize, usize)) -> usize {
    let mut end = 'X';

    let mut front: HashSet<(usize, usize)> = HashSet::from([coords]);
    let mut to_move: HashSet<(usize, usize)> = HashSet::from([coords]);
    while end == 'X' {
        let mut new_front: HashSet<(usize, usize)> = HashSet::new();
        for &(row, col) in &front {
            match grid[[row - 1, col]] {
                '#' => {
                    new_front.insert((row - 1, col));
                    end = '#';
                    break;
                }
                '.' => {}
                '[' => {
                    new_front.insert((row - 1, col));
                    to_move.insert((row - 1, col));
                    new_front.insert((row - 1, col + 1));
                    to_move.insert((row - 1, col + 1));
                }
                ']' => {
                    new_front.insert((row - 1, col));
                    to_move.insert((row - 1, col));
                    new_front.insert((row - 1, col - 1));
                    to_move.insert((row - 1, col - 1));
                }
                _ => panic!("unknown cell content! {}", grid[[row - 1, col]])
            }
        }
        if new_front.iter().all(|&(r, c)| grid[[r, c]] == '.') {
            end = '.';
        } else {
            front = new_front;
        }
    }

    if end == '.' {
        let mut to_move_vec = to_move.iter().collect::<Vec<&(usize, usize)>>();
        to_move_vec.sort_unstable();
        for &(r, c) in to_move_vec {
            let chr = grid[[r, c]];
            grid[[r, c]] = '.';
            grid[[r - 1, c]] = chr;
        }
        return 1;
    }
    0
}


pub fn part2(inp: &str) -> impl Display {
    let inp_wide = inp.chars().map(
        |c| match c {
            '#' => "##".to_string(),
            'O' => "[]".to_string(),
            '.' => "..".to_string(),
            '@' => "@.".to_string(),
            _ => c.to_string()
        }
    ).collect::<Vec<String>>().join("");

    let (_nrows, _ncols, mut grid) = utils::inp_to_array2_char(
        &*inp_wide.lines().take_while(|l| !l.is_empty()).collect::<Vec<&str>>().join("\n")
    );

    let dirs = inp.chars()
        .filter(|c| ['>', 'v', '<', '^'].contains(c)).collect::<String>();

    let mut coords: (usize, usize) = (9999, 9999);
    if let Some(((row, col), _)) = grid.indexed_iter()
        .find(|(_, &value)| value == '@') {
        coords = (row, col);
    }

    for c in dirs.chars() {
        match c {
            '>' => coords.1 += move_horz(
                &mut grid.row_mut(coords.0)),
            'v' => { coords.0 += move_down(&mut grid, coords) }
            '<' => coords.1 -= move_horz(
                &mut grid.row_mut(coords.0).slice_mut(s![..;-1])),
            '^' => { coords.0 -= move_up(&mut grid, coords) }
            _ => panic!("unknown direction! {}", c)
        }
    }
    grid.indexed_iter().filter(|(_, &v)| v == '[')
        .map(|((r, c), _)| 100 * r + c).sum::<usize>()
}