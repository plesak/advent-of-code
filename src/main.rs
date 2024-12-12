
mod utils;
mod y23;
use crate::y23::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day number (e.g., day1, day2, ...)
    #[arg(required = true)]
    day: String,

    /// Part number (part1 or part2)
    #[arg(required = true)]
    part: String,

    /// Year (optional e.g. 2023, defaults to current year)
    #[arg(short, long)]
    year: Option<String>,

    /// Use example input
    #[arg(short, long)]
    example: bool,
}

fn main() {
    let args = Args::parse();

    let year = args.year.unwrap_or_else(|| "latest".to_string());
    let input_file = construct_input_path(&args.day, &args.part, &year, args.example);

    println!("Input file path: {:?}", input_file);
    let q = format!("{}_{}", args.day, args.part);

    let mut input = fs::read_to_string(input_file).unwrap();

    if args.example {
        let lines: Vec<&str> = input.lines().collect();
        if let Some((last, rest)) = lines.split_last() {
            let expected_result = last.trim().to_string();
            input = rest.join("\n");
            println!("Expected result: {}", expected_result);
        } else {
            println!("Error: Example file is empty or malformed");
        }
    }

    let result:String;
    let start_time = Instant::now();
    if year == "latest" {
        result = match q.as_str() {
            "day1_part1" => day1::part1(&input).to_string(),
            "day1_part2" => day1::part2(&input).to_string(),
            "day2_part1" => day2::part1(&input).to_string(),
            "day2_part2" => day2::part2(&input).to_string(),
            "day3_part1" => day3::part1(&input).to_string(),
            "day3_part2" => day3::part2(&input).to_string(),
            "day4_part1" => day4::part1(&input).to_string(),
            "day4_part2" => day4::part2(&input).to_string(),
            "day5_part1" => day5::part1(&input).to_string(),
            "day5_part2" => day5::part2(&input).to_string(),
            "day6_part1" => day6::part1(&input).to_string(),
            "day6_part2" => day6::part2(&input).to_string(),
            "day7_part1" => day7::part1(&input).to_string(),
            "day7_part2" => day7::part2(&input).to_string(),
            "day8_part1" => day8::part1(&input).to_string(),
            "day8_part2" => day8::part2(&input).to_string(),
            "day9_part1" => day9::part1(&input).to_string(),
            "day9_part2" => day9::part2(&input).to_string(),
            "day10_part1" => day10::part1(&input).to_string(),
            "day10_part2" => day10::part2(&input).to_string(),
            "day11_part1" => day11::part1(&input).to_string(),
            "day11_part2" => day11::part2(&input).to_string(),
            "day12_part1" => day12::part1(&input).to_string(),
            "day12_part2" => day12::part2(&input).to_string(),
            _ => panic!("Question not implemented! Arguments: {}", q),
        }
    } else if year == "y23" {
        let inp = input.lines().map(String::from).collect::<Vec<String>>();
        result = match q.as_str() {
            "y23q1q1" => y23q1q1(inp),
            "y23q1q2" => y23q1q2(inp),
            "y23q2q1" => y23q2q1(inp),
            "y23q2q2" => y23q2q2(inp),
            "y23q3q1" => y23q3q1(inp),
            "y23q3q2" => y23q3q2(inp),
            "y23q4q1" => y23q4q1(inp),
            "y23q4q2" => y23q4q2(inp),
            "y23q5q1" => y23q5q1(inp),
            "y23q5q2" => y23q5q2(inp),
            "y23q6q1" => y23q6q1(inp),
            "y23q6q2" => y23q6q2(inp),
            "y23q7q1" => y23q7q1(inp),
            "y23q7q2" => y23q7q2(inp),
            "y23q8q1" => y23q8q1(inp),
            "y23q8q2" => y23q8q2(inp),
            "y23q9q1" => y23q9q1(inp),
            "y23q9q2" => y23q9q2(inp),
            "y23q10q1" => y23q10q1(inp),
            "y23q10q2" => y23q10q2(inp),
            "y23q11q1" => y23q11q1(inp),
            "y23q11q2" => y23q11q2(inp),
            "y23q12q1" => y23q12q1(inp),
            "y23q12q2" => y23q12q2(inp),
            "y23q13q1" => y23q13q1(inp),
            "y23q13q2" => y23q13q2(inp),
            "y23q14q1" => y23q14q1(inp),
            "y23q14q2" => y23q14q2(inp),
            "y23q15q1" => y23q15q1(inp),
            "y23q15q2" => y23q15q2(inp),
            "y23q16q1" => y23q16q1(inp),
            "y23q16q2" => y23q16q2(inp),
            "y23q17q1" => y23q17q1(inp),
            "y23q17q2" => y23q17q2(inp),
            "y23q18q1" => y23q18q1(inp),
            "y23q18q2" => y23q18q2(inp),
            "y23q19q1" => y23q19q1(inp),
            "y23q19q2" => y23q19q2(inp),
            "y23q20q1" => y23q20q1(inp),
            "y23q20q2" => y23q20q2(inp),
            "y23q21q1" => y23q21q1(inp),
            "y23q21q2" => y23q21q2(inp),
            "y23q22q1" => y23q22q1(inp),
            "y23q22q2" => y23q22q2(inp),
            "y23q23q1" => y23q23q1(inp),
            "y23q23q2" => y23q23q2(inp),
            "y23q24q1" => y23q24q1(inp),
            "y23q24q2" => y23q24q2(inp),
            "y23q25q1" => y23q25q1(inp),
            "y23q25q2" => y23q25q2(inp),
            _ => panic!("Question not implemented! Arguments: {}", q),
        };
    } else {
        panic!("Unknown year {} - only available `-2023` and `-2024` (default)", year);
    }

    println!("Result: {}", result);
    println!("Time taken: {}", start_time.elapsed().as_secs_f64());
}

fn construct_input_path(day: &str, part: &str, year: &str, example: bool) -> PathBuf {
    let mut path = PathBuf::from("inputs");

    if year != "latest" {
        path.push(year);
    }

    if example {
        path.push("tests");
        path.push(format!("{}_{}.txt", day, part));
    } else {
        path.push(format!("{}.txt", day));
    }

    path
}
