use std::fmt::Display;

fn parse_col(inp: &str) -> Vec<isize> {
    // 7 6 4 2 1
    inp.split_whitespace().map(|s| s.parse::<isize>().unwrap()).collect()
}

pub fn part1(inp: &str) -> impl Display {
    let mut safe_reports = 0;
    let allowed_diffs: [isize; 3] = [1, 2, 3];
    for ln in inp.lines() {
        let report = parse_col(&ln);
        if report.windows(2)
            .all(|w| allowed_diffs.contains(&(w[1] - w[0]))) {
            safe_reports += 1;
        } else if report.windows(2)
            .all(|w| allowed_diffs.contains(&(w[0] - w[1]))) {
            safe_reports += 1;
        }
    }
    safe_reports
}

pub fn part2(inp: &str) -> impl Display {
    let mut safe_reports = 0;
    let allowed_diffs: [isize; 3] = [1, 2, 3];
    for ln in inp.lines() {
        let report = parse_col(ln);
        // 1  5  2  4  5
        for i in 0..report.len() {
            let mut rep_to_check = report.iter().map(|&n| n).collect::<Vec<isize>>();
            rep_to_check.remove(i);
            if rep_to_check.windows(2)
                .all(|w| allowed_diffs.contains(&(w[1] - w[0]))) {
                safe_reports += 1;
                break;
            } else if rep_to_check.windows(2)
                .all(|w| allowed_diffs.contains(&(w[0] - w[1]))) {
                safe_reports += 1;
                break;
            }
        }
    }
    safe_reports
}