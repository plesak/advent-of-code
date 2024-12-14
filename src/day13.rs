use std::fmt::Display;

fn parse_ln(ln: &str) -> (isize, isize) {
    ln.split_once(',').map(
        |(x, y)| (
            x.chars().filter(|c| c.is_digit(10))
                .collect::<String>().parse::<isize>().unwrap(),
            y.chars().filter(|c| c.is_digit(10))
                .collect::<String>().parse::<isize>().unwrap()
        )
    ).unwrap()
}

fn find_combo(a: (isize, isize), b: (isize, isize), p: (isize, isize)) -> Option<isize> {
    // P1 = A1*a + B1*b
    // P2 = A2*a + B2*b
    // --> a = (P1 - B1*b) / A1
    // --> P2 = A2*P1/A1 - A2*B1*b/A1 + B2*b
    // --> A1*P2 = A2*P1 - A2*B1*b + A1*B2*b
    // --> b*(A1*B2 - A2*B1) = A1*P2 - A2*P1
    // --> b = (A1*P2 - A2*P1) / (A1*B2 - A2*B1)
    // --> a = (B1*P2 - B2*P1) / (B1*A2 - B2*A1)
    let den_a = b.0 * a.1 - b.1 * a.0;
    let den_b = a.0 * b.1 - a.1 * b.0;

    // Check for division by zero
    if den_a == 0 || den_b == 0 {
        return None;
    }

    let num_a = b.0 * p.1 - b.1 * p.0;
    let num_b = a.0 * p.1 - a.1 * p.0;

    let (res_a, rem_a) = (num_a / den_a, num_a % den_a);
    let (res_b, rem_b) = (num_b / den_b, num_b % den_b);

    if rem_a == 0 && rem_b == 0 {
        // verify or panic
        if a.0 * res_a + b.0 * res_b == p.0 && a.1 * res_a + b.1 * res_b == p.1 {
            return Some(3 * res_a + res_b);
        } else {
            panic!("uh-oh")
        }
    }

    None
}

pub fn part1(inp: &str) -> impl Display {
    let mut a: (isize, isize) = (0, 0);
    let mut b: (isize, isize) = (0, 0);
    let mut p: (isize, isize);

    let mut total_presses: isize = 0;

    for (i, ln) in inp.lines().enumerate() {
        if i % 4 == 0 {
            a = parse_ln(ln);
        } else if i % 4 == 1 {
            b = parse_ln(ln);
        } else if i % 4 == 2 {
            p = parse_ln(ln);
            if let Some(res) = find_combo(a, b, p) {
                total_presses += res;
            }
        }
    }
    total_presses
}

pub fn part2(inp: &str) -> impl Display {
    let mut a: (isize, isize) = (0, 0);
    let mut b: (isize, isize) = (0, 0);
    let mut p: (isize, isize);

    let mut total_presses: isize = 0;

    for (i, ln) in inp.lines().enumerate() {
        if i % 4 == 0 {
            a = parse_ln(ln);
        } else if i % 4 == 1 {
            b = parse_ln(ln);
        } else if i % 4 == 2 {
            let (p1, p2) = parse_ln(ln);
            p = (10000000000000 + p1, 10000000000000 + p2);
            if let Some(res) = find_combo(a, b, p) {
                total_presses += res;
            }
        }
    }
    total_presses
}