use crate::utils;
use std::fmt::Display;

pub fn part1(inp: &str) -> impl Display {
    let (nrows, ncols, grid) = utils::inp_to_array2_char(inp);


    let mut xmas_count = 0;
    for i in 0..ncols {
        for j in 0..nrows {
            // if letter is X search all 8 directions
            if grid[[i, j]] == 'X' {
                let east_valid = j + 3 < ncols;
                let south_valid = i + 3 < nrows;
                let west_valid = j >= 3;
                let north_valid = i >= 3;
                // east
                if east_valid {
                    if grid[[i, j + 1]] == 'M' && grid[[i, j + 2]] == 'A' && grid[[i, j + 3]] == 'S' {
                        xmas_count += 1;
                    }
                }
                // south-east
                if east_valid && south_valid {
                    if grid[[i + 1, j + 1]] == 'M' && grid[[i + 2, j + 2]] == 'A' && grid[[i + 3, j + 3]] == 'S' {
                        xmas_count += 1;
                    }
                }
                // south
                if south_valid {
                    if grid[[i + 1, j]] == 'M' && grid[[i + 2, j]] == 'A' && grid[[i + 3, j]] == 'S' {
                        xmas_count += 1;
                    }
                }
                // south-west
                if south_valid && west_valid {
                    if grid[[i + 1, j - 1]] == 'M' && grid[[i + 2, j - 2]] == 'A' && grid[[i + 3, j - 3]] == 'S' {
                        xmas_count += 1;
                    }
                }
                // west
                if west_valid {
                    if grid[[i, j - 1]] == 'M' && grid[[i, j - 2]] == 'A' && grid[[i, j - 3]] == 'S' {
                        xmas_count += 1;
                    }
                }
                // north-west
                if west_valid && north_valid {
                    if grid[[i - 1, j - 1]] == 'M' && grid[[i - 2, j - 2]] == 'A' && grid[[i - 3, j - 3]] == 'S' {
                        xmas_count += 1;
                    }
                }
                // north
                if north_valid {
                    if grid[[i - 1, j]] == 'M' && grid[[i - 2, j]] == 'A' && grid[[i - 3, j]] == 'S' {
                        xmas_count += 1;
                    }
                }
                // north-east
                if north_valid && east_valid {
                    if grid[[i - 1, j + 1]] == 'M' && grid[[i - 2, j + 2]] == 'A' && grid[[i - 3, j + 3]] == 'S' {
                        xmas_count += 1;
                    }
                }
            }
        }
    }
    xmas_count
}

pub fn part2(inp: &str) -> impl Display {
    let (nrows, ncols, grid) = utils::inp_to_array2_char(inp);

    let mut xmas_count = 0;
    for i in 1..ncols - 1 {
        for j in 1..nrows - 1 {
            if grid[[i, j]] == 'A' {
                // negative diag
                let (a, b) = (grid[[i - 1, j - 1]], grid[[i + 1, j + 1]]);
                if (a, b) == ('M', 'S') || (a, b) == ('S', 'M') {
                    // positive diag
                    let (a, b) = (grid[[i - 1, j + 1]], grid[[i + 1, j - 1]]);
                    if (a, b) == ('M', 'S') || (a, b) == ('S', 'M') {
                        xmas_count += 1;
                    }
                }
            }
        }
    }
    xmas_count
}