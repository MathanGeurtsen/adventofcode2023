use std::collections::VecDeque;
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use regex::{Regex, Match};

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let file = File::open("../advent_of_code_secrets/2023/day_03/input.txt")?;
    let reader = io::BufReader::new(file);
    
    // .........917.......&....146........790.....*......*....*..........*.....*...............207*......................796..116......../...924..
    // 722...323.-................./...410.............72..748.442............384.708...............849..%............................470..........
    // .....*..........................*.....271..691....-...............4*...........388.................448......&........*....848&......751.....
    
    let mut lines: Vec<String> = Vec::new();
    for line_result in reader.lines() {
        let Ok(line) = line_result
        else { return Err("Failed to parse game nr".into()) };
        lines.push(line)
    }
    let first_line = lines[0].clone();
    let len = first_line.len();
    let mut part_nrs : Vec<i32> = Vec::new();

    let re_symbol = Regex::new(r"[^0-9\.]")?;
    let re_nr = Regex::new(r"([0-9]+)")?;
    for i in 1..lines.len()-2 {
        let line = &lines[i];
        let line_prev = &lines[i-1];
        let line_next = &lines[i+1];
        
        let nrs: Vec<Match> = re_nr.find_iter(&line).collect();
        for nr in nrs {
            let start = if nr.start() > 0 {nr.start() - 1} else { nr.start() };
            let end = if nr.end() < len {nr.end() + 1} else { nr.end() };
            
            let mut adjacent : String = "".into();
            adjacent.push_str(&line[start..end]);
            adjacent.push_str(&line_prev[start..end]);
            adjacent.push_str(&line_next[start..end]);
            
            let adjacent_symbol = re_symbol.is_match(&adjacent);
            if adjacent_symbol {
                part_nrs.push(nr.as_str().parse()?)
            }
            println!("{:?}\n{:?}\n{:?}\n{:?}\n{:#?}", line_prev, line, line_next, adjacent, adjacent_symbol);
            print!("")
        }
    }

    let first_nrs: Vec<Match> = re_nr.find_iter(&lines[0]).collect();
    for nr in first_nrs{
        let start = if nr.start() > 0 {nr.start() - 1} else { nr.start() };
        let end = if nr.end() < len {nr.end() + 1} else { nr.end() };
        
        let mut adjacent : String = "".into();
        adjacent.push_str(&lines[0][start..end]);
        adjacent.push_str(&lines[1][start..end]);
        
        let adjacent_symbol = re_symbol.is_match(&adjacent);
        if adjacent_symbol {
            part_nrs.push(nr.as_str().parse()?)
        }
        println!("{:?}\n{:?}\n{:?}\n{:?}\n{:#?}", nr, lines[0], lines[1], adjacent, adjacent_symbol);
        print!("")
    }

    let last_nrs: Vec<Match> = re_nr.find_iter(&lines[lines.len()-1]).collect();
    for nr in last_nrs{
        let start = if nr.start() > 0 {nr.start() - 1} else { nr.start() };
        let end = if nr.end() < len {nr.end() + 1} else { nr.end() };
        
        let mut adjacent : String = "".into();
        adjacent.push_str(&lines[lines.len()-2][start..end]);
        adjacent.push_str(&lines[lines.len()-1][start..end]);
        
        let adjacent_symbol = re_symbol.is_match(&adjacent);
        if adjacent_symbol {
            part_nrs.push(nr.as_str().parse()?)
        }
        println!("{:?}\n{:?}\n{:?}\n{:?}\n{:#?}", nr, lines[lines.len()-2], lines[lines.len()-1], adjacent, adjacent_symbol);
    }
    println!("{:?}", part_nrs.iter().sum::<i32>());
    Ok(())
}