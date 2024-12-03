
mod utils;
mod y23;
use crate::y23::*;

mod day1;
mod day2;
// mod day3;

use std::time::Instant;
use std::{env, fs, io};

fn main() {
    let mut args = env::args();

    if args.len() < 2 {
        panic!("Usage: cargo run <question_id> [-r]");
    }

    let _ = args.next();
    let question_id = args.next().unwrap();
    let full = args.next().map_or(false, |arg| arg == "-f");
    let suffix = if full { ".txt" } else { "_test.txt" };

    let inp: String;
    if !full {
        let input: String = read_file(&question_id, suffix).unwrap();
        let (inpt, true_result) = input.split_at(input.rfind('\n').unwrap());
        inp = inpt.to_string();
        println!("Expected result: {}", true_result.chars().skip(1).collect::<String>());
    } else {
        inp = read_file(&question_id, suffix).unwrap();
    }

    let yr = question_id.chars().take(3).collect::<String>();
    let result:String;
    let start_time = Instant::now();
    if yr == "y24" {
        result = match question_id.as_str() {
            "y24q1q1" => day1::part1(&inp).to_string(),
            "y24q1q2" => day1::part2(&inp).to_string(),
            "y24q2q1" => day2::part1(&inp).to_string(),
            "y24q2q2" => day2::part2(&inp).to_string(),
            _ => panic!("Question not implemented! {}", question_id),
        }
    } else if yr == "y23" {
        let input = inp.lines().map(String::from).collect();
        result = match question_id.as_str() {
            "y23q1q1" => y23q1q1(input),
            "y23q1q2" => y23q1q2(input),
            "y23q2q1" => y23q2q1(input),
            "y23q2q2" => y23q2q2(input),
            "y23q3q1" => y23q3q1(input),
            "y23q3q2" => y23q3q2(input),
            "y23q4q1" => y23q4q1(input),
            "y23q4q2" => y23q4q2(input),
            "y23q5q1" => y23q5q1(input),
            "y23q5q2" => y23q5q2(input),
            "y23q6q1" => y23q6q1(input),
            "y23q6q2" => y23q6q2(input),
            "y23q7q1" => y23q7q1(input),
            "y23q7q2" => y23q7q2(input),
            "y23q8q1" => y23q8q1(input),
            "y23q8q2" => y23q8q2(input),
            "y23q9q1" => y23q9q1(input),
            "y23q9q2" => y23q9q2(input),
            "y23q10q1" => y23q10q1(input),
            "y23q10q2" => y23q10q2(input),
            "y23q11q1" => y23q11q1(input),
            "y23q11q2" => y23q11q2(input),
            "y23q12q1" => y23q12q1(input),
            "y23q12q2" => y23q12q2(input),
            "y23q13q1" => y23q13q1(input),
            "y23q13q2" => y23q13q2(input),
            "y23q14q1" => y23q14q1(input),
            "y23q14q2" => y23q14q2(input),
            "y23q15q1" => y23q15q1(input),
            "y23q15q2" => y23q15q2(input),
            "y23q16q1" => y23q16q1(input),
            "y23q16q2" => y23q16q2(input),
            "y23q17q1" => y23q17q1(input),
            "y23q17q2" => y23q17q2(input),
            "y23q18q1" => y23q18q1(input),
            "y23q18q2" => y23q18q2(input),
            "y23q19q1" => y23q19q1(input),
            "y23q19q2" => y23q19q2(input),
            "y23q20q1" => y23q20q1(input),
            "y23q20q2" => y23q20q2(input),
            "y23q21q1" => y23q21q1(input),
            "y23q21q2" => y23q21q2(input),
            "y23q22q1" => y23q22q1(input),
            "y23q22q2" => y23q22q2(input),
            "y23q23q1" => y23q23q1(input),
            "y23q23q2" => y23q23q2(input),
            "y23q24q1" => y23q24q1(input),
            "y23q24q2" => y23q24q2(input),
            "y23q25q1" => y23q25q1(input),
            "y23q25q2" => y23q25q2(input),
            _ => panic!("Question not implemented: {}", question_id),
        };
    } else {
        panic!("Unknown year {} - only available `y23qXqX` and `y24qXqX`", yr);
    }

    println!("Result of {}: {}", question_id, result);
    println!("Time taken: {}", start_time.elapsed().as_secs_f64());
}

fn read_file(q_id: &str, suffix: &str) -> io::Result<String> {
    let current_exe = env::current_exe().expect("Failed to get current exe path");
    let project_root = current_exe.parent().unwrap().parent().unwrap().parent().unwrap();
    let filename = project_root
        .join("input_files").join(format!("{}{}", q_id, suffix));
    println!("Reading {}", filename.display());
    fs::read_to_string(filename)
}

// NICE