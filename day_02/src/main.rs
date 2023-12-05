use std::fs::File;
use std::io::{self, BufRead};
use regex::{Regex, Match};

fn main() -> io::Result<()> 
{
    let file = File::open("../advent_of_code_secrets/2023/day_02/input.txt")?;
    let reader = io::BufReader::new(file);
    
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    
    let mut games: Vec<Vec<String>> = Vec::new();
    
    // Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green
    let regex =  Regex::new(r"Game ([0-9])+:(( ([0-9])+ (blue|green|red)[,;]?)+)").unwrap();
    
    
    for line_result in reader.lines() {
        let line = line_result?;
        if let Some(captures) = regex.captures(&line) {
            let capture_group: Vec<String> = captures.iter().filter_map(
                |item| {
                    let re_match: Match = item?;
                    Some(re_match.as_str().to_string())
                }).collect();
                println!("{:#?}", capture_group);
            }
        }
        Ok(())
    }