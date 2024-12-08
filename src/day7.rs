use std::collections::VecDeque;
use std::fmt::Display;

fn parse_line(inp: &str) -> (isize, VecDeque<isize>) {
    // 3267: 81 40 27
    let (gl, rst) = inp.split_once(':').unwrap();
    let goal = gl.parse::<isize>().unwrap();
    let rest: VecDeque<isize> = rst.split_whitespace()
        .map(|s| s.parse::<isize>().unwrap()).collect();
    (goal, rest)
}

pub fn part1(inp: &str) -> impl Display {
    let mut total = 0;
    for ln in inp.lines() {
        let (goal, mut nums) = parse_line(ln);
        // if product of all is lower than goal we dont even have to try
        let mut totals: Vec<isize> = Vec::new();
        if let Some(n) = nums.pop_front() {
            totals.push(n);
        }
        let mut replacements: Vec<isize> = Vec::new();
        while let Some(n) = nums.pop_front() {
            for &x in totals.iter() {
                replacements.push(x * n);
                replacements.push(x + n);
            }
            totals = replacements;
            replacements = Vec::new();
        }
        for &n in totals.iter() {
            if n == goal {
                total += goal;
                break;
            }
        }
    }
    total
}

pub fn part2(inp: &str) -> impl Display {
    let mut total = 0;
    for ln in inp.lines() {
        let (goal, mut nums) = parse_line(ln);
        // if product of all is lower than goal we dont even have to try
        let mut totals: Vec<isize> = Vec::new();
        if let Some(n) = nums.pop_front() {
            totals.push(n);
        }
        let mut replacements: Vec<isize> = Vec::new();
        while let Some(n) = nums.pop_front() {
            for &x in totals.iter() {
                replacements.push(x * n);
                replacements.push(x + n);
                // new hidden operation concatenates numbers
                let mut concat = x.to_string();
                concat.push_str(&n.to_string());
                replacements.push(concat.parse::<isize>().unwrap());
            }
            totals = replacements;
            replacements = Vec::new();
        }
        for &n in totals.iter() {
            if n == goal {
                total += goal;
                break;
            }
        }
    }
    total
}