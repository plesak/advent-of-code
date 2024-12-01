use std::cmp::{max, min};
use ndarray::Array2;
use num::ToPrimitive;
use crate::utils;

#[derive(Debug)]
struct Dig(char, i32);

pub fn y23q18q1(input:Vec<String>) -> String {
    let mut curr_lr = 0;
    let mut max_r = 0;
    let mut min_l = 0;
    let mut curr_du = 0;
    let mut max_d = 0;
    let mut min_u = 0;

    let mut digs:Vec<Dig> = Vec::with_capacity(input.len());

    for ln in input {
        let ln_content = ln.split_whitespace().collect::<Vec<&str>>();
        let dir = ln_content[0].chars().nth(0).unwrap();
        let val = ln_content[1].to_string().parse::<i32>().unwrap();
        match dir {
            'R' => { curr_lr += val; max_r = max(max_r, curr_lr); },
            'D' => { curr_du += val; max_d = max(max_d, curr_du); },
            'L' => { curr_lr -= val; min_l = min(min_l, curr_lr); },
            'U' => { curr_du -= val; min_u = min(min_u, curr_du); },
            _ => panic!("unknown direction! {}", dir)
        }
        digs.push(Dig(dir, val));
    }

    let mut grid:Array2<char> =
        Array2::from_elem((
                              (max_d - min_u +1).to_usize().unwrap(),
                              (max_r - min_l +1).to_usize().unwrap()
                          ),'.');
    let mut coords:(usize, usize) = (
        (-1*min_u).to_usize().unwrap(),
        (-1*min_l).to_usize().unwrap()
    );

    if digs.last().unwrap().0 == 'U' {
        grid[[coords.0, coords.1]] = 'D';
    } else if digs.last().unwrap().0 == 'D' {
        grid[[coords.0, coords.1]] = 'U';
    } else {
        panic!("unexpected setup! {}", digs.last().unwrap().0);
    }

    for dig in digs {
        match dig.0 {
            'R' => {
                for _ in 1..=dig.1 {
                    coords = (coords.0, coords.1+1);
                    grid[[coords.0, coords.1]] = 'P';
                }
            },
            'L' => {
                for _ in 1..=dig.1 {
                    coords = (coords.0, coords.1-1);
                    grid[[coords.0, coords.1]] = 'P';
                }
            },
            'U' => {
                grid[[coords.0, coords.1]] = 'U';
                for _ in 1..dig.1 {
                    coords = (coords.0-1, coords.1);
                    grid[[coords.0, coords.1]] = 'T';
                }
                coords = (coords.0-1, coords.1);
                grid[[coords.0, coords.1]] = 'D';
            },
            'D' => {
                grid[[coords.0, coords.1]] = 'D';
                for _ in 1..dig.1 {
                    coords = (coords.0+1, coords.1);
                    grid[[coords.0, coords.1]] = 'T';
                }
                coords = (coords.0+1, coords.1);
                grid[[coords.0, coords.1]] = 'U';
            },
            _ => panic!("unexpected direction! {}", dig.0)
        }
    }

    let mut flood_total = 0;
    for row in grid.rows() {
        let mut horizontal_start = 'X';
        let mut inside = false;
        for &c in row {
            match c {
                'P' => flood_total += 1,
                'T' => {
                    flood_total += 1;
                    inside = !inside;
                },
                'U' | 'D' => {
                    flood_total += 1;
                    if horizontal_start == 'X' {
                        horizontal_start = c;
                    } else if horizontal_start == c {
                        horizontal_start = 'X';
                    } else {
                        inside = !inside;
                        horizontal_start = 'X';
                    }
                },
                '.' => {
                        if inside { flood_total += 1; }
                },
                _ => panic!("unknown map char! {}", c)
            }
        }
    }

    flood_total.to_string()
}

fn hex_to_dig(inp:String) -> (char, i64) {
    // eg (#70c710)
    // 70c71 --> (hex) 461937
    // 0 - R, 1 - D, 2 - L, 3 - U
    let dir = match inp.chars().nth(7).unwrap() {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => panic!("unknown direction! {}", inp.chars().nth(7).unwrap())
    };

    let hex_val = inp.chars().skip(2).take(5).collect::<String>();
    let val = i64::from_str_radix(&hex_val, 16).unwrap();
    (dir, val)
}

pub fn y23q18q2(input:Vec<String>) -> String {
    let mut digs:Vec<(i64, char)> = Vec::with_capacity(input.len());
    for ln in input {
        let ln_content = ln.split_whitespace().collect::<Vec<&str>>();
        let (dir, val) = hex_to_dig(ln_content[2].to_string());
        digs.push((val, dir));
    }
    utils::flood_from_paths(digs, true).to_string()
}