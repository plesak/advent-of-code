use std::collections::HashMap;
use num::integer::lcm;

pub fn y23q8q1(input: Vec<String>) -> String {
    // first line is instructions
    let path = &input[0].chars().collect::<Vec<char>>();

    // store all paths into hash table
    let mut paths:HashMap<&str, (&str, &str)> = HashMap::new();
    for ln in input[2..].iter() {
        // parse AAA = (BBB, BBB)
        let inpt = ln[..3].chars().as_str();
        let out1 = ln[7..10].chars().as_str();
        let out2 = ln[12..15].chars().as_str();
        paths.insert(inpt, (out1, out2));
    }

    // go over instructions until hitting ZZZ
    let mut curr_node = "AAA";
    let mut steps = 0;
    while curr_node != "ZZZ" {
        let next_dir = path[steps % path.len()];
        curr_node = match next_dir {
            'L' => paths.get(curr_node).unwrap().0,
            'R' => paths.get(curr_node).unwrap().1,
            _ => unreachable!("I don't know where to go!!")
        };
        steps += 1;
    }

    steps.to_string()
}

pub fn y23q8q2(input: Vec<String>) -> String {
    // first line is instructions
    let path = &input[0].chars().collect::<Vec<char>>();

    // store all paths into hash table
    let mut paths:HashMap<&str, (&str, &str)> = HashMap::new();
    let mut start_nodes:Vec<&str> = Vec::new();
    for ln in input[2..].iter() {
        // parse AAA = (BBB, BBB)
        let inpt = ln[..3].chars().as_str();
        if inpt.chars().last() == Some('A') { start_nodes.push(inpt) };
        let out1 = ln[7..10].chars().as_str();
        let out2 = ln[12..15].chars().as_str();
        paths.insert(inpt, (out1, out2));
    }

    // clue to trying out LCM
    // After examining the maps a bit longer, your attention is drawn to a curious fact:
    // the number of nodes with names ending in A is equal to the number ending in Z!

    let mut lens:Vec<usize> = Vec::new();
    for sn in start_nodes {
        let mut curr_node = sn;
        let mut steps = 0;
        while curr_node.chars().last() != Some('Z') {
            let next_dir = path[steps % path.len()];
            curr_node = match next_dir {
                'L' => paths.get(curr_node).unwrap().0,
                'R' => paths.get(curr_node).unwrap().1,
                _ => unreachable!("I don't know where to go!!")
            };
            steps += 1;
        }
        println!("steps {}", steps);
        lens.push(steps);
    }

    let mut n = lens.pop().unwrap();
    while let Some(val) = lens.pop() {
        n = lcm(n, val);
    }

    n.to_string()
}