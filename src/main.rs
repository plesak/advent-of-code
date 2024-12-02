
mod utils;
mod y23;
use crate::y23::*;
mod y24;
use crate::y24::*;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let args:Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: program <question_id> [-r]".into());
    }

    let question_id = &args[1];
    let full = args.get(2).map_or(false, |arg| arg == "-f");
    let suffix = if full { ".txt" } else { "_test.txt" };

    let mut input = read_file(question_id, suffix)?;
    let true_result = if !full {
        input.pop().unwrap_or_else(|| String::from("No result provided"))
    } else {
        String::new()
    };

    let result:String;

    let start_time = Instant::now();
    match question_id.as_str() {
        "y23q1q1" => {result = y23q1q1(input)},
        "y23q1q2" => {result = y23q1q2(input)},
        "y23q2q1" => {result = y23q2q1(input)},
        "y23q2q2" => {result = y23q2q2(input)},
        "y23q3q1" => {result = y23q3q1(input)},
        "y23q3q2" => {result = y23q3q2(input)},
        "y23q4q1" => {result = y23q4q1(input)},
        "y23q4q2" => {result = y23q4q2(input)},
        "y23q5q1" => {result = y23q5q1(input)},
        "y23q5q2" => {result = y23q5q2(input)},
        "y23q6q1" => {result = y23q6q1(input)},
        "y23q6q2" => {result = y23q6q2(input)},
        "y23q7q1" => {result = y23q7q1(input)},
        "y23q7q2" => {result = y23q7q2(input)},
        "y23q8q1" => {result = y23q8q1(input)},
        "y23q8q2" => {result = y23q8q2(input)},
        "y23q9q1" => {result = y23q9q1(input)},
        "y23q9q2" => {result = y23q9q2(input)},
        "y23q10q1" => {result = y23q10q1(input)},
        "y23q10q2" => {result = y23q10q2(input)},
        "y23q11q1" => {result = y23q11q1(input)},
        "y23q11q2" => {result = y23q11q2(input)},
        "y23q12q1" => {result = y23q12q1(input)},
        "y23q12q2" => {result = y23q12q2(input)},
        "y23q13q1" => {result = y23q13q1(input)},
        "y23q13q2" => {result = y23q13q2(input)},
        "y23q14q1" => {result = y23q14q1(input)},
        "y23q14q2" => {result = y23q14q2(input)},
        "y23q15q1" => {result = y23q15q1(input)},
        "y23q15q2" => {result = y23q15q2(input)},
        "y23q16q1" => {result = y23q16q1(input)},
        "y23q16q2" => {result = y23q16q2(input)},
        "y23q17q1" => {result = y23q17q1(input)},
        "y23q17q2" => {result = y23q17q2(input)},
        "y23q18q1" => {result = y23q18q1(input)},
        "y23q18q2" => {result = y23q18q2(input)},
        "y23q19q1" => {result = y23q19q1(input)},
        "y23q19q2" => {result = y23q19q2(input)},
        "y23q20q1" => {result = y23q20q1(input)},
        "y23q20q2" => {result = y23q20q2(input)},
        "y23q21q1" => {result = y23q21q1(input)},
        "y23q21q2" => {result = y23q21q2(input)},
        "y23q22q1" => {result = y23q22q1(input)},
        "y23q22q2" => {result = y23q22q2(input)},
        "y23q23q1" => {result = y23q23q1(input)},
        "y23q23q2" => {result = y23q23q2(input)},
        "y23q24q1" => {result = y23q24q1(input)},
        "y23q24q2" => {result = y23q24q2(input)},
        "y23q25q1" => {result = y23q25q1(input)},
        "y23q25q2" => {result = y23q25q2(input)},
        "y24q1q1" => {result = y24q1q1(input)},
        "y24q1q2" => {result = y24q1q2(input)},
        _ => return Err(format!("Question not implemented: {}", question_id).into()),
    }
    println!("Time taken: {}", start_time.elapsed().as_secs_f64());

    if !full {
        println!("Expected result: {}", true_result);
        println!("Actual result: {}", result);
    } else {
        println!("Result of {}: {}", question_id, result);
    }

    Ok(())
}

fn read_file(q_id: &str, suffix: &str) -> Result<Vec<String>, std::io::Error> {
    let current_exe = env::current_exe()?;
    let project_root = current_exe.parent().unwrap().parent().unwrap().parent().unwrap();
    let filename = project_root.join("input_files").join(format!("{}{}", q_id, suffix));
    println!("Reading {}", filename.display());
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

// NICE