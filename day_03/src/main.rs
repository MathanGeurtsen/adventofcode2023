use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let file = File::open("../advent_of_code_secrets/2023/day_03/input.txt")?;
    let reader = io::BufReader::new(file);
   
    let mut possible: Vec<i32> = Vec::new();
    let mut powers = Vec::<i32>::new();
    
    for line_result in reader.lines() {
        let Ok(line) = line_result
        else { return Err("Failed to parse game nr".into()) };
    }
    Ok(())
}