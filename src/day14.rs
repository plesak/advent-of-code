use ndarray::Array2;
use std::fmt::Display;
use std::str::FromStr;

fn parse_ln(ln: &str) -> Result<(isize, isize, isize, isize), Box<dyn std::error::Error>> {
    let (p, v) = ln.split_once(' ').ok_or("Invalid input format")?;
    let p = p.trim_start_matches("p=");
    let v = v.trim_start_matches("v=");

    let parse_pair = |s: &str| -> Result<(isize, isize), Box<dyn std::error::Error>> {
        let (a, b) = s.split_once(',').ok_or("Invalid pair format")?;
        Ok((isize::from_str(a)?, isize::from_str(b)?))
    };

    let (p1, p2) = parse_pair(p)?;
    let (v1, v2) = parse_pair(v)?;

    Ok((p1, p2, v1, v2))
}

pub fn part1(inp: &str) -> impl Display {
    const NROWS: isize = 103;
    const NCOLS: isize = 101;
    const SECONDS: isize = 100;

    // top left, top right, bottom left, bottom right
    let mut robots_in_qs: Vec<usize> = vec![0; 4];

    for ln in inp.lines() {
        let (p_y, p_x, v_y, v_x) = match parse_ln(ln) {
            Ok((p1, p2, p3, p4)) => {
                (p1, p2, p3, p4)
            }
            Err(_e) => panic!("Invalid input"),
        };

        let new_x = (p_x + v_x * SECONDS).rem_euclid(NROWS);
        let new_y = (p_y + v_y * SECONDS).rem_euclid(NCOLS);
        // println!("robot from {},{} speed {},{} found at {},{}", p_x, p_y, v_x, v_y, new_x, new_y);

        if new_x < NROWS / 2 {
            if new_y < NCOLS / 2 {
                robots_in_qs[0] += 1;
            } else if new_y > NCOLS / 2 {
                robots_in_qs[1] += 1;
            }
        } else if new_x > NROWS / 2 {
            if new_y < NCOLS / 2 {
                robots_in_qs[2] += 1;
            } else if new_y > NCOLS / 2 {
                robots_in_qs[3] += 1;
            }
        }
    }
    // println!("robots in qs: {:?}", robots_in_qs);
    robots_in_qs.iter().product::<usize>()
}

// fn calc_vert_symmetry(grid: &Array2<usize>) -> isize {
//     let ncols = grid.ncols();
//     let mut sym:isize = 0;
//     for row in grid.rows() {
//         let left = row.iter().take(ncols/2).sum::<usize>() as isize;
//         let right = row.iter().skip(ncols/2).sum::<usize>() as isize;
//         sym += abs(left - right);
//     }
//     sym
// }
// fn get_fullest_row(grid: &Array2<usize>) -> usize {
//     grid.rows().into_iter()
//         .map(|r| r.sum())
//         .max().unwrap()
// }
fn check_bots_in_row(grid: &Array2<usize>) -> bool {
    for r in grid.rows() {
        let mut streak = 0;
        for &n in r {
            if n != 0 {
                streak += 1;
                if streak > 10 { return true; }
            } else { streak = 0; }
        }
    }
    false
}

pub fn part2(inp: &str) -> impl Display {
    const NROWS: isize = 103;
    const NCOLS: isize = 101;
    // const SECONDS:isize = 100;

    let mut robots: Vec<(isize, isize, isize, isize)> = Vec::new();

    for ln in inp.lines() {
        let (p_y, p_x, v_y, v_x) = match parse_ln(ln) {
            Ok((p1, p2, p3, p4)) => {
                (p1, p2, p3, p4)
            }
            Err(_e) => panic!("Invalid input"),
        };
        robots.push((p_x, p_y, v_x, v_y));
    }

    // pattern will repeat after nrows*ncols
    for i in 1..=NROWS * NCOLS {
        // if i % 1000 == 0 {
        //     println!("working on iteration {}", i);
        // }
        let mut robots_at_i: Vec<(isize, isize)> = Vec::new();
        for (p_x, p_y, v_x, v_y) in robots.iter() {
            let new_x = (p_x + v_x * i).rem_euclid(NROWS);
            let new_y = (p_y + v_y * i).rem_euclid(NCOLS);
            robots_at_i.push((new_x, new_y));
        }
        let mut grid =
            Array2::from_elem((NROWS as usize, NCOLS as usize), 0);
        for (x, y) in robots_at_i {
            grid[[x as usize, y as usize]] += 1;
        }
        // println!("GRID AT i={} has qs {:?}", i, robots_in_qs);
        if check_bots_in_row(&grid) {
            // println!("GRID AT i={}", i);
            // for row in grid.rows() {
            //     for &cell in row.iter() {
            //         let char_to_print = match cell {
            //             0 => '.',
            //             1..=9 => char::from_digit(cell as u32, 10).unwrap(),
            //             _ => 'X',
            //         };
            //         print!("{}", char_to_print);
            //     }
            //     println!(); // New line after each row
            // }
            return i;
        }
    }
    -1
}