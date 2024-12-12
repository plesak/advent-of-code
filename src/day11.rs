use num::Integer;
use std::collections::HashMap;
use std::fmt::Display;
// If the stone is engraved with the number 0,
//     it is replaced by a stone engraved with the number 1.
// If the stone is engraved with a number that has an even number of digits,
//     it is replaced by two stones.
//     The left half of the digits are engraved on the new left stone,
//     and the right half of the digits are engraved on the new right stone.
//     (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
// If none of the other rules apply, the stone is replaced by a new stone;
//     the old stone's number multiplied by 2024 is engraved on the new stone.

fn next_stones(stone: usize) -> Vec<usize> {
    if stone == 0 {
        vec![1]
    } else if stone.to_string().len().is_even() {
        let st = stone.to_string();
        let left = st.chars().take(st.len() / 2)
            .collect::<String>().parse::<usize>().unwrap();
        let right = st.chars().skip(st.len() / 2)
            .collect::<String>().parse::<usize>().unwrap();
        vec![left, right]
    } else {
        vec![stone * 2024]
    }
}

fn blink(stones: HashMap<usize, usize>, mut memo: HashMap<usize, Vec<usize>>)
         -> (HashMap<usize, usize>, HashMap<usize, Vec<usize>>) {
    let mut new_stones: HashMap<usize, usize> = HashMap::new();
    for (stone, count) in stones {
        let next_up: Vec<usize>;
        if let Some(v) = memo.get(&stone) {
            next_up = v.clone();
        } else {
            next_up = next_stones(stone);
            memo.insert(stone, next_up.clone());
        }
        for new_stone in next_up {
            let e = new_stones.entry(new_stone).or_insert(0);
            *e += count;
        }
    }
    (new_stones, memo)
}

pub fn part1(inp: &str) -> impl Display {
    let st: Vec<(usize, usize)> = inp.split_whitespace()
        .map(|x| (x.parse::<usize>().unwrap(), 1)).collect();
    let mut stones: HashMap<usize, usize> = HashMap::from_iter(st);
    let mut memo: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..25 {
        (stones, memo) = blink(stones, memo);
    }

    stones.values().sum::<usize>()
}

pub fn part2(inp: &str) -> impl Display {
    let st: Vec<(usize, usize)> = inp.split_whitespace()
        .map(|x| (x.parse::<usize>().unwrap(), 1)).collect();
    let mut stones: HashMap<usize, usize> = HashMap::from_iter(st);
    let mut memo: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..75 {
        (stones, memo) = blink(stones, memo);
    }

    stones.values().sum::<usize>()
}