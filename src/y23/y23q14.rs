use std::collections::HashMap;
use ndarray::{Array2, Axis};

pub fn y23q14q1(input:Vec<String>) -> String {

    let nrows = input.len();
    let ncols = input[0].len();
    let flat_vec:Vec<char> = input.concat().chars().collect();
    let mut a =
        Array2::from_shape_vec((nrows, ncols), flat_vec).unwrap();

    a = a.t().to_owned();

    let mut running_total = 0;
    for row in a.rows() {
        // println!("processing row {:?}", row);
        let mut pull_dest_idx = nrows;
        for (i, c) in row.iter().enumerate() {
            match c {
                '#' => pull_dest_idx = nrows-i-1,
                'O' => {
                    // println!("adding destination index {}", pull_dest_idx);
                    running_total += pull_dest_idx;
                    pull_dest_idx -= 1;
                },
                _ => {}
            }
        }
    }

    running_total.to_string()
}

pub fn y23q14q2(input:Vec<String>) -> String {

    let nrows = input.len();
    let ncols = input[0].len();
    let flat_vec:Vec<char> = input.concat().chars().collect();
    let mut a =
        Array2::from_shape_vec((nrows, ncols), flat_vec).unwrap();

    fn tilt(mut a:Array2<char>) -> Array2<char> {
        // always tilts "left" ie all elems are pulled toward start of their row
        for mut row in a.rows_mut() {
            let mut pull_dest_idx = 0;
            let mut changes:Vec<(usize, char)> = Vec::new();
            for (i, c) in row.iter().enumerate() {
                match c {
                    '#' => pull_dest_idx = i + 1,
                    'O' => {
                        if pull_dest_idx != i {
                            changes.push((pull_dest_idx, 'O'));
                            changes.push((i, '.'));
                        }
                        pull_dest_idx += 1;
                    },
                    _ => {}
                }
            }
            for (idx, val) in changes {
                row[idx] = val;
            }
        }
        a
    }

    // orient "north to the left" before cycle starts
    a = a.t().to_owned();
    a.invert_axis(Axis(0));

    let mut saved_boards:HashMap<Array2<char>, i32> = HashMap::new();
    const ROUNDS:i32 = 1000000000;
    let mut r = 0;
    let mut cycle_found = false;
    while r < ROUNDS {
        if !cycle_found {
            if let Some(&saved) = saved_boards.get(&a) {
                println!("found previous board!! {} / {}", saved, r);
                let cycle_len = r-saved;
                r += cycle_len*((ROUNDS-r)/(cycle_len));
                println!("skipped to cycle {}", r);
                cycle_found = true;
            }
            saved_boards.insert(a.clone(), r);
        }
        // Cycle start: north is already to the left
        a = tilt(a);
        // Rotate for west (90 degrees clockwise)
        a = a.t().to_owned();
        a.invert_axis(Axis(1));
        // Tilt west (now to the left)
        a = tilt(a);
        // Rotate for south (90 degrees clockwise)
        a = a.t().to_owned();
        a.invert_axis(Axis(1));
        // Tilt south (now to the left)
        a = tilt(a);
        // Rotate for east (90 degrees clockwise)
        a = a.t().to_owned();
        a.invert_axis(Axis(1));
        // Tilt east (now to the left)
        a = tilt(a);
        // Rotate back to cycle-start position (90 degrees clockwise)
        a = a.t().to_owned();
        a.invert_axis(Axis(1));
        r += 1;
    }

    // finally, calculate value of north supports (leverage still facing north)
    let mut running_total = 0;
    let nsym = a.ncols();
    for r in a.rows() {
        for (i, &c) in r.iter().enumerate() {
            if c == 'O' {
                running_total += nsym-i;
            }
        }
    }

    // rotate counterclockwise to return to initial state for printing
    // a = a.t().to_owned();
    // a.invert_axis(Axis(1));
    // println!();
    // for r in a.rows() {
    //     println!("{}", r.iter().join(""));
    // }


    running_total.to_string()
}