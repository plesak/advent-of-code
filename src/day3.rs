use regex::Regex;
use std::fmt::Display;
use std::sync::LazyLock;

static RE1: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap());
static RE2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|(?:do|don't)\(\))").unwrap());

pub fn part1(inp: &str) -> impl Display {
    let mut sum = 0;
    for mat in RE1.find_iter(inp) {
        let m = mat.as_str().chars()
            .skip_while(|&x| !x.is_digit(10))
            .take_while(|&x| x.is_digit(10) || x == ',').collect::<String>();
        let n = m.split(',')
            .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        sum += n[0] * n[1]
    }
    sum
}

pub fn part2(inp: &str) -> impl Display {
    let mut sum = 0;
    let mut enabled = true;
    for mat in RE2.find_iter(inp) {
        match mat.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            m if enabled => {
                let (a, b) = m
                    .trim_start_matches("mul(").trim_end_matches(')')
                    .split_once(',').map(|(a, b)| (
                    a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()
                )).unwrap();
                sum += a * b;
            }
            _ => {}
        }
    }
    sum
}