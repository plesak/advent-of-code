use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

fn parse_rule(inp: &str) -> (usize, usize) {
    inp.split_once('|')
        .map(|(a, b)| (
            a.parse::<usize>().unwrap(),
            b.parse::<usize>().unwrap()
        )).unwrap()
}
fn parse_report(inp: &str) -> Vec<usize> {
    inp.split(',').map(|x|
        x.parse::<usize>().unwrap()
    ).collect::<Vec<usize>>()
}

#[derive(Debug)]
struct Num {
    val: usize,
    after: HashSet<usize>,
}

impl PartialEq for Num {
    fn eq(&self, _other: &Self) -> bool {
        false // Numbers are never equal
    }
}

impl Eq for Num {}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Num {
    fn cmp(&self, other: &Self) -> Ordering {
        if other.after.contains(&self.val) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}


pub fn part1(inp: &str) -> impl Display {
    let mut processing_rules = true;
    let mut sum_of_report_middles = 0;

    let mut rules: HashMap<usize, Num> = HashMap::new();

    for ln in inp.lines() {
        if ln.is_empty() {
            processing_rules = false;
        } else if processing_rules {
            // processing rules
            let (a, b) = parse_rule(ln);
            rules.entry(a).or_insert(Num { val: a, after: HashSet::new() }).after.insert(b);
        } else {
            // processing reports
            let rep = parse_report(ln);
            for &n in &rep {
                rules.entry(n).or_insert(Num { val: n, after: HashSet::new() });
            }
            let rep_num: Vec<&Num> = rep.iter()
                .map(|&n| rules.get(&n).unwrap())
                .collect();
            if rep_num.windows(2).all(|w| { w[0] <= w[1] }) {
                sum_of_report_middles += rep[rep.len() / 2];
            }
        }
    }
    sum_of_report_middles
}

pub fn part2(inp: &str) -> impl Display {
    let mut processing_rules = true;
    let mut sum_of_fixed_report_middles = 0;

    let mut rules: HashMap<usize, Num> = HashMap::new();

    for ln in inp.lines() {
        if ln.is_empty() {
            processing_rules = false;
        } else if processing_rules {
            // processing rules
            let (a, b) = parse_rule(ln);
            rules.entry(a).or_insert(Num { val: a, after: HashSet::new() }).after.insert(b);
        } else {
            // processing reports
            let rep = parse_report(ln);
            for &n in &rep {
                rules.entry(n).or_insert(Num { val: n, after: HashSet::new() });
            }
            let mut rep_num: Vec<&Num> = rep.iter()
                .map(|&n| rules.get(&n).unwrap())
                .collect();
            if !rep_num.windows(2).all(|w| { w[0] <= w[1] }) {
                rep_num.sort_unstable();
                sum_of_fixed_report_middles += rep_num[rep_num.len() / 2].val;
            }
        }
    }
    sum_of_fixed_report_middles
}