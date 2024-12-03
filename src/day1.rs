use num::abs;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;

fn parse_col(inp: &str) -> (i32, i32) {
    let mut iter = inp.split_whitespace();
    (
        iter.next().unwrap().parse::<i32>().unwrap(),
        iter.next().unwrap().parse::<i32>().unwrap(),
    )
}

pub fn part1(input: &str) -> impl Display {

    let mut a_list: BinaryHeap<i32> = BinaryHeap::new();
    let mut b_list: BinaryHeap<i32> = BinaryHeap::new();

    for ln in input.lines() {
        let (a, b) = parse_col(ln);
        a_list.push(a);
        b_list.push(b);
    }

    let mut total_dist = 0;
    while let Some(a) = a_list.pop() {
        let b = b_list.pop().unwrap();
        total_dist += abs(b - a);
    }

    total_dist
}

pub fn part2(input: &str) -> impl Display {

    let mut a_list: HashMap<i32, i32> = HashMap::new();
    let mut b_list: HashMap<i32, i32> = HashMap::new();

    for ln in input.lines() {
        let (a, b) = parse_col(ln);
        *a_list.entry(a).or_insert(0) += 1;
        *b_list.entry(b).or_insert(0) += 1;
    }

    let mut total_cnt: i32 = 0;

    for a in a_list {
        if let Some(b) = b_list.get(&a.0) {
            total_cnt += a.0 * a.1 * b;
        }
    }

    total_cnt
}