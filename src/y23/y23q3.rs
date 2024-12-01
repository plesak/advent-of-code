use std::cmp::min;

pub fn y23q3q1(input: Vec<String>) -> String {

    let input2 = input.clone();
    let mut input:Vec<String> = Vec::new();
    for mut ln in input2 {
        ln.push('.');
        input.push(ln);
    }

    fn is_symbol(c:char) -> bool {
        c != '.' && !c.is_ascii_digit()
    }
    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 467 + 35 + 633 = 1135
    let mut sm = 0;

    let num_rows = input.len();
    let num_cols = input[0].len();
    // let mut grid:Vec<Vec<usize>> = vec![vec![0; num_cols+1]; num_rows+1];

    for (i, ln) in input.iter().enumerate() {

        let mut curr_num = "".to_string();
        let mut curr_is_num = false;
        let mut curr_num_valid = false;

        for (j, char) in ln.chars().enumerate() {
            if char.is_ascii_digit() {
                curr_num.push(char);
                curr_is_num = true;

            // handle last digit
            } else if curr_is_num {

                if is_symbol(char) {
                    // println!("validated by {} at {}, {}", char, i, j);
                    curr_num_valid = true;
                }

                // find all adjacent indices
                let num_digits = curr_num.len();
                let min_j = j.saturating_sub(num_digits+1);
                let max_j = min(j+1, num_cols);

                // check row above
                if !curr_num_valid && i > 0 {
                    for c in input[i-1].chars().skip(min_j).take(max_j - min_j) {
                        if is_symbol(c) {
                            // println!("validated by {} at {}, {}", c, i, j);
                            curr_num_valid = true;
                            break;
                        }
                    }
                }
                // check previous symbol
                if !curr_num_valid && j > num_digits {
                    if is_symbol(ln.chars().nth(j-num_digits-1).unwrap()) {
                        // println!("validated by ??? at {}, {}", i, j);
                        curr_num_valid = true;
                    }
                }
                // check row below
                if !curr_num_valid && i < num_rows-1 {
                    for c in input[i+1].chars().skip(min_j).take(max_j - min_j) {
                        if is_symbol(c) {
                            // println!("validated by {} at {}, {}", c, i, j);
                            curr_num_valid = true;
                            break;
                        }
                    }
                }

                // if number is valid, convert and sum it
                if curr_num_valid {
                    // println!("ADDING VALID NUM XXXXXXXXXX {}", curr_num.parse::<i32>().unwrap().to_string());
                    sm += curr_num.parse::<i32>().unwrap_or(0);
                    curr_num_valid = false;
                }

                curr_is_num = false;
                curr_num = "".to_string();
            }
        }
    }

    sm.to_string()
}

pub fn y23q3q2(input: Vec<String>) -> String {

    let input2 = input.clone();
    let mut input:Vec<String> = Vec::new();
    for mut ln in input2 {
        ln.push('.');
        input.push(ln);
    }

    fn is_symbol(c:char) -> bool {
        c != '.' && !c.is_ascii_digit()
    }
    const GEAR:char = '*';
    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 467 + 35 + 633 = 1135
    let mut sm = 0;

    let num_rows = input.len();
    let num_cols = input[0].len();
    let mut grid:Vec<Vec<Vec<usize>>> = vec![vec![vec![]; num_cols+1]; num_rows+1];

    for (i, ln) in input.iter().enumerate() {

        let mut curr_num = "".to_string();
        let mut curr_is_num = false;
        let mut curr_num_valid = false;

        for (j, char) in ln.chars().enumerate() {
            if char.is_ascii_digit() {
                curr_num.push(char);
                curr_is_num = true;

            // handle last digit
            } else if curr_is_num {

                let mut gear_coord = [0, 0];

                if is_symbol(char) {
                    curr_num_valid = true;
                    if char == GEAR {gear_coord = [i,j];}
                }

                // find all adjacent indices
                let num_digits = curr_num.len();
                let min_j = j.saturating_sub(num_digits+1);
                let max_j = min(j+1, num_cols);

                // check row above
                if !curr_num_valid && i > 0 {
                    for (k, c) in input[i-1].chars().skip(min_j).take(max_j - min_j).enumerate() {
                        if is_symbol(c) {
                            curr_num_valid = true;
                            if c == GEAR {gear_coord = [i-1,min_j+k];}
                            break;
                        }
                    }
                }
                // check previous symbol
                if !curr_num_valid && j > num_digits {
                    let c = ln.chars().nth(min_j).unwrap();
                    if is_symbol(c) {
                        curr_num_valid = true;
                        if c == GEAR {gear_coord = [i,min_j];}
                    }
                }
                // check row below
                if !curr_num_valid && i < num_rows-1 {
                    for (k, c) in input[i+1].chars().skip(min_j).take(max_j - min_j).enumerate() {
                        if is_symbol(c) {
                            curr_num_valid = true;
                            if c == GEAR {gear_coord = [i+1,min_j+k];}
                            break;
                        }
                    }
                }

                // if number is valid, convert and sum it
                if curr_num_valid {
                    let n = curr_num.parse::<usize>().unwrap_or(0);
                    // note gear
                    if gear_coord[0] != 0 {
                        grid[gear_coord[0]][gear_coord[1]].push(n);
                        // println!("pushed {} to gear {},{}", n, gear_coord[0], gear_coord[1]);
                    }
                    curr_num_valid = false;
                }

                curr_is_num = false;
                curr_num = "".to_string();
            }
        }
    }

    for i in 0..num_rows {
        for j in 0..num_cols {
            let val = grid[i][j].clone();
            if val.len() == 2 {
                // println!("found valid gear at {},{} with {:?}", i, j, val);
                sm += val[0]*val[1];
            }
        }
    }

    sm.to_string()
}