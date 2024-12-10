use ndarray::Array2;
use std::collections::HashSet;
use std::fmt::Display;

fn inp_to_array2(input: Vec<&str>) -> Array2<usize> {
    let nrows = input.len();
    let ncols = input[0].len();
    let flat_vec: Vec<usize> = input.concat().chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
    Array2::from_shape_vec((nrows, ncols), flat_vec).unwrap()
}

pub fn part1(inp: &str) -> impl Display {
    let grid = inp_to_array2(inp.lines().collect::<Vec<&str>>());

    let nrows = grid.nrows();
    let ncols = grid.ncols();

    let mut hikes_sum = 0;

    for i in 0..ncols {
        for j in 0..nrows {
            if grid[[i, j]] == 0 {
                let mut nines: HashSet<(usize, usize)> = HashSet::new();
                let mut currs: Vec<(usize, usize)> = Vec::new();
                currs.push((i, j));

                for step_height in 1..=9 {
                    let mut news: Vec<(usize, usize)> = Vec::new();
                    for &(x, y) in currs.iter() {
                        if x > 0 {
                            if grid[[x - 1, y]] == step_height {
                                news.push((x - 1, y));
                            }
                        }
                        if y > 0 {
                            if grid[[x, y - 1]] == step_height {
                                news.push((x, y - 1));
                            }
                        }
                        if x < nrows - 1 {
                            if grid[[x + 1, y]] == step_height {
                                news.push((x + 1, y));
                            }
                        }
                        if y < ncols - 1 {
                            if grid[[x, y + 1]] == step_height {
                                news.push((x, y + 1));
                            }
                        }
                    }
                    if step_height == 9 {
                        for &new in news.iter() {
                            // println!("adding coords {:?}", new);
                            nines.insert(new);
                        }
                        hikes_sum += nines.len()
                    } else {
                        currs = news;
                    }
                }
            }
        }
    }
    hikes_sum
}

pub fn part2(inp: &str) -> impl Display {
    let grid = inp_to_array2(inp.lines().collect::<Vec<&str>>());

    let nrows = grid.nrows();
    let ncols = grid.ncols();

    let mut hikes_sum = 0;

    for i in 0..ncols {
        for j in 0..nrows {
            if grid[[i, j]] == 0 {
                let mut nines: HashSet<Vec<(usize, usize)>> = HashSet::new();
                let mut currs: Vec<Vec<(usize, usize)>> = Vec::new();
                currs.push(vec![(i, j)]);

                for step_height in 1..=9 {
                    let mut news: Vec<Vec<(usize, usize)>> = Vec::new();
                    for path in currs.iter() {
                        let (x, y) = path[path.len() - 1];
                        if x > 0 {
                            if grid[[x - 1, y]] == step_height {
                                let mut new_path = path.clone();
                                new_path.push((x - 1, y));
                                news.push(new_path);
                            }
                        }
                        if y > 0 {
                            if grid[[x, y - 1]] == step_height {
                                let mut new_path = path.clone();
                                new_path.push((x, y - 1));
                                news.push(new_path);
                            }
                        }
                        if x < nrows - 1 {
                            if grid[[x + 1, y]] == step_height {
                                let mut new_path = path.clone();
                                new_path.push((x + 1, y));
                                news.push(new_path);
                            }
                        }
                        if y < ncols - 1 {
                            if grid[[x, y + 1]] == step_height {
                                let mut new_path = path.clone();
                                new_path.push((x, y + 1));
                                news.push(new_path);
                            }
                        }
                    }
                    if step_height == 9 {
                        for new in news.iter() {
                            // println!("adding coords {:?}", new);
                            nines.insert(new.clone());
                        }
                        hikes_sum += nines.len()
                    } else {
                        currs = news;
                    }
                }
            }
        }
    }
    hikes_sum
}