use std::cmp::min;
use ndarray::Array2;

pub fn y23q13q1(input:Vec<String>) -> String {
    let mut running_total = 0;

    fn evaluate_sqr(inp:Array2<char>, by_rows:bool) -> i32 {
        let mut res:i32 = -1;
        let n_rows = inp.nrows();
        assert!(n_rows > 0);
        for i in 0..n_rows-1 {
            if inp.row(i) == inp.row(i+1) {
                let mut validated = true;
                let rows_to_check = min(i, n_rows-i-2)+1;
                for j in 1..rows_to_check {
                    if !(inp.row(i-j) == inp.row(i+j+1)) {
                        validated = false;
                        break;
                    }
                }
                if validated {
                    if by_rows {
                        // println!("found mirror between rows {} and {}, adding {}", i+1, i+2, 100*(i+1));
                        res = (100*(i+1)) as i32;
                    } else {
                        // println!("found mirror between cols {} and {}, adding {}", i+1, i+2, i+1);
                        res = (i+1) as i32;
                    }
                    break;
                }
            }
        }
        res
    }

    let mut curr_sqr:Vec<String> = Vec::new();
    let mut iter = input.iter().peekable();
    while let Some(ln) = iter.next() {
        if ln.is_empty() || iter.peek().is_none() {
            let rows = curr_sqr.len();
            let cols = curr_sqr[0].len();
            let a =
                Array2::from_shape_fn((rows, cols),
                                      |(i, j)| curr_sqr[i].chars().nth(j).unwrap());
            // println!("evaluating grid by rows\n{}", a);
            let mut val = evaluate_sqr(a.clone(), true);
            if val == -1 {
                // that means nothing was found, we transpose and find in cols
                // println!("evaluating grid by cols\n{}", a.t());
                val = evaluate_sqr(a.t().to_owned(), false);
            }
            running_total += val;
            curr_sqr.clear();
        } else {
            curr_sqr.push(ln.to_owned());
        }
    }
    running_total.to_string()
}

pub fn y23q13q2(input:Vec<String>) -> String {
    let mut running_total = 0;

    fn evaluate_sqr(inp:Array2<char>, by_rows:bool) -> i32 {
        let mut res:i32 = -1;
        let n_rows = inp.nrows();
        assert!(n_rows > 0);
        for i in 0..n_rows-1 {
            let mut smudges = 0;
            let rows_to_check = min(i+1, n_rows-i-1);
            for j in 0..rows_to_check {
                smudges += inp.row(i-j).iter()
                    .zip(inp.row(i+j+1).iter())
                    .filter(|(c1, c2)| c1 != c2)
                    .count();
                if smudges > 1 { break; }
            }
            if smudges == 1 {
                if by_rows {
                    println!("found mirror between rows {} and {}, adding {}", i+1, i+2, 100*(i+1));
                    res = (100*(i+1)) as i32;
                } else {
                    println!("found mirror between cols {} and {}, adding {}", i+1, i+2, i+1);
                    res = (i+1) as i32;
                }
                break;
            }
        }
        res
    }

    let mut curr_sqr:Vec<String> = Vec::new();
    let mut iter = input.iter().peekable();
    while let Some(ln) = iter.next() {
        if ln.is_empty() || iter.peek().is_none() {
            let rows = curr_sqr.len();
            let cols = curr_sqr[0].len();
            let a =
                Array2::from_shape_fn((rows, cols),
                                      |(i, j)| curr_sqr[i].chars().nth(j).unwrap());
            // println!("evaluating grid by rows\n{}", a);
            let mut val = evaluate_sqr(a.clone(), true);
            if val == -1 {
                // that means nothing was found, we transpose and find in cols
                // println!("evaluating grid by cols\n{}", a.t());
                val = evaluate_sqr(a.t().to_owned(), false);
            }
            running_total += val;
            curr_sqr.clear();
        } else {
            curr_sqr.push(ln.to_owned());
        }
    }
    running_total.to_string()
}