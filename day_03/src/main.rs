use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use regex::{Regex, Match};


fn pt1(lines: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let len = lines[0].len();
    let mut part_nrs : Vec<i32> = Vec::new();
    
    let re_symbol = Regex::new(r"[^0-9\.]")?;
    let re_nr = Regex::new(r"([0-9]+)")?;
    
    for i in 0..lines.len() {
        
        let nrs: Vec<Match> = re_nr.find_iter(&lines[i]).collect();
        for nr in nrs {
            let nrnr = nr.as_str();
            
            let mut adjacent : String = "".into();
            let start = if nr.start() > 0 {nr.start() - 1} else {0};
            let end = if nr.end() < len-1 { nr.end()+1} else {nr.end()};
            
            println!("\nmatching: {:#?}", nr);
            
            if i > 0 {
                adjacent.push_str(&lines[i-1][start..end]);
                println!("{:#?}", lines[i-1]);
            }
            
            adjacent.push_str(&lines[i][start..end]);
            println!("{:#?}", lines[i]);
            if i < lines.len()-1 {
                adjacent.push_str(&lines[i+1][start..end]);
                println!("{:#?}", lines[i+1]);
            }
            
            let adjacent_symbol = re_symbol.is_match(&adjacent);
            if adjacent_symbol {
                part_nrs.push(nr.as_str().parse()?)
            }
            println!("{:#?}", adjacent);
            println!("{:#?}", adjacent_symbol);
        }
        
        
    }
    
    println!("{:?}", part_nrs.iter().sum::<i32>());
    Ok(())
}

fn pt2(lines: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let len = lines[0].len();
    let mut gear_ratios : Vec<i32> = Vec::new();
    
    let re_symbol = Regex::new(r"[^0-9\.]")?;
    let re_nr = Regex::new(r"([0-9]+)")?;
    let re_gear = Regex::new(r"(\*)")?;

    for i in 0..lines.len() {
        
        let nrs: Vec<Match> = re_nr.find_iter(&lines[i]).collect();
        for gear in re_gear.find_iter(&lines[i]) {
            let nrnr = nr.as_str();
            
            let mut adjacent : String = "".into();
            let start = if nr.start() > 0 {nr.start() - 1} else {0};
            let end = if nr.end() < len-1 { nr.end()+1} else {nr.end()};
            
            println!("\nmatching: {:#?}", nr);
            
            if i > 0 {
                adjacent.push_str(&lines[i-1][start..end]);
                println!("{:#?}", lines[i-1]);
            }
            
            adjacent.push_str(&lines[i][start..end]);
            println!("{:#?}", lines[i]);
            if i < lines.len()-1 {
                adjacent.push_str(&lines[i+1][start..end]);
                println!("{:#?}", lines[i+1]);
            }
            
            let adjacent_symbol = re_symbol.is_match(&adjacent);
            if adjacent_symbol {
                part_nrs.push(nr.as_str().parse()?)
            }
            println!("{:#?}", adjacent);
            println!("{:#?}", adjacent_symbol);
        }
        
        
    }
    
    println!("{:?}", part_nrs.iter().sum::<i32>());
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let file = File::open("../advent_of_code_secrets/2023/day_03/input.txt")?;
    let reader = io::BufReader::new(file);
    
    let mut lines: Vec<String> = Vec::new();
    for line_result in reader.lines() {
        lines.push(line_result?);
    }
    
    pt1(lines);
    
    
    Ok(())
}