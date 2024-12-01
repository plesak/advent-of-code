use itertools::Itertools;

pub fn y23q11q1(input:Vec<String>) -> String {

    let mut empty_rows:Vec<usize> = Vec::new();
    let mut empty_col_bools:Vec<bool> = vec![true; input[0].len()];

    let mut gx_rows:Vec<usize> = Vec::new();
    let mut gx_cols:Vec<usize> = Vec::new();

    for (i, ln) in input.iter().enumerate() {
        let mut empty = true;
        for (j, c) in ln.chars().enumerate() {
            if c == '#' {
                gx_rows.push(i);
                gx_cols.push(j);
                empty = false;
                empty_col_bools[j] = false;
            }
        }
        if empty { empty_rows.push(i); }
    }
    gx_cols.sort();
    let empty_cols:Vec<usize> = empty_col_bools
        .iter()
        // returns indices of where condition is true
        .positions(|&b| b)
        .collect();

    // expand rows
    let mut gx_row_exp_ptr = 0;
    let mut gx_rows_exp:Vec<isize> = Vec::with_capacity(gx_rows.len());
    for &gx in gx_rows.iter() {
        while gx_row_exp_ptr < empty_rows.len() && gx > empty_rows[gx_row_exp_ptr] {
            gx_row_exp_ptr += 1;
        }
        gx_rows_exp.push((gx + gx_row_exp_ptr) as isize);
    }
    // expand cols
    let mut gx_col_exp_ptr = 0;
    let mut gx_cols_exp:Vec<isize> = Vec::with_capacity(gx_cols.len());
    for &gx in gx_cols.iter() {
        while gx_col_exp_ptr < empty_cols.len() && gx > empty_cols[gx_col_exp_ptr] {
            gx_col_exp_ptr += 1;
        }
        gx_cols_exp.push((gx + gx_col_exp_ptr) as isize);
    }

    // math is fun
    // [x, y] --> -x +y
    // [x, y, z] --> -2x +0y +2z
    // [x, y, z, a] --> -3x -y +z +3a ... etc
    let mut total_sum:isize = 0;
    let num_gx:isize = gx_rows.len() as isize;
    // assert_eq!(num_gx, gx_cols_exp.len() as isize);
    // sum row distances
    for (i, x) in gx_rows_exp.iter().enumerate() {
        total_sum += (1 + 2*(i as isize) - num_gx) * x;
    }
    // sum col distances
    for (i, x) in gx_cols_exp.iter().enumerate() {
        total_sum += (1 + 2*(i as isize) - num_gx) * x;
    }

    total_sum.to_string()
}

pub fn y23q11q2(input:Vec<String>) -> String {
    const FACTOR:usize = 1000000;
    // example FACTOR=10 -> 1030 & FACTOR=100 -> 8410
    // task FACTOR=1000000

    let mut empty_rows:Vec<usize> = Vec::new();
    let mut empty_col_bools:Vec<bool> = vec![true; input[0].len()];

    let mut gx_rows:Vec<usize> = Vec::new();
    let mut gx_cols:Vec<usize> = Vec::new();

    for (i, ln) in input.iter().enumerate() {
        let mut empty = true;
        for (j, c) in ln.chars().enumerate() {
            if c == '#' {
                gx_rows.push(i);
                gx_cols.push(j);
                empty = false;
                empty_col_bools[j] = false;
            }
        }
        if empty { empty_rows.push(i); }
    }
    gx_cols.sort();
    let empty_cols:Vec<usize> = empty_col_bools
        .iter()
        // returns indices of where condition is true
        .positions(|&b| b)
        .collect();

    // expand rows
    let mut gx_row_exp_ptr = 0;
    let mut gx_rows_exp:Vec<isize> = Vec::with_capacity(gx_rows.len());
    for &gx in gx_rows.iter() {
        while gx_row_exp_ptr < empty_rows.len() && gx > empty_rows[gx_row_exp_ptr] {
            gx_row_exp_ptr += 1;
        }
        gx_rows_exp.push((gx + (FACTOR-1)*gx_row_exp_ptr) as isize);
    }
    // expand cols
    let mut gx_col_exp_ptr = 0;
    let mut gx_cols_exp:Vec<isize> = Vec::with_capacity(gx_cols.len());
    for &gx in gx_cols.iter() {
        while gx_col_exp_ptr < empty_cols.len() && gx > empty_cols[gx_col_exp_ptr] {
            gx_col_exp_ptr += 1;
        }
        gx_cols_exp.push((gx + (FACTOR-1)*gx_col_exp_ptr) as isize);
    }

    // math is fun
    // [x, y] --> -x +y
    // [x, y, z] --> -2x +0y +2z
    // [x, y, z, a] --> -3x -y +z +3a ... etc
    let mut total_sum:isize = 0;
    let num_gx:isize = gx_rows.len() as isize;
    // assert_eq!(num_gx, gx_cols_exp.len() as isize);
    // sum row distances
    for (i, x) in gx_rows_exp.iter().enumerate() {
        total_sum += (1 + 2*(i as isize) - num_gx) * x;
    }
    // sum col distances
    for (i, x) in gx_cols_exp.iter().enumerate() {
        total_sum += (1 + 2*(i as isize) - num_gx) * x;
    }

    total_sum.to_string()
}