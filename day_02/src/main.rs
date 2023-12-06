use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let file = File::open("../advent_of_code_secrets/2023/day_02/input.txt")?;
    let reader = io::BufReader::new(file);
    
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    
    let mut possible: Vec<i32> = Vec::new();
    
    // Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green
    
    for line_result in reader.lines() {
        let Ok(line) = line_result
        else { return Err("Failed to parse game nr".into()) };
        
        let split_line : Vec<String>= line.split(':').map(|x| x.to_string()).collect();
        
        let Some(game_nr_cap) = Regex::new("Game (?<nr>[0-9]+)")?.captures(&split_line[0])
        else { return Err("Failed to parse game nr".into()) };
        let game_nr = game_nr_cap["nr"].parse()?;
        
        let mut poss: bool = true;
       
        let hands: Vec<Vec<&str>> = split_line[1].split(';').map(|hand| hand.split(',').collect()).collect();
        for hand in hands{
            let mut nr_red = 0;
            let mut nr_green = 0;
            let mut nr_blue = 0;

            for group in hand {
                let Some(split_group) = Regex::new("(?<nr>[0-9]+) (?<color>[a-z]+)")?.captures(group)
                else { return Err("failed to parse group".into()) };
                let nr: i32 = split_group["nr"].parse()?;
                let color = &split_group["color"];
                
                match color {
                    "red" => { nr_red += nr},
                    "green" => { nr_green += nr},
                    "blue" => { nr_blue += nr},
                    _ => {return Err("failed to split with ':'".into()) }
                } 
            }

            poss = (nr_red <= red_max && nr_green <= green_max && nr_blue <= blue_max) && poss;
        }
        if poss{
            possible.push(game_nr)
        }
        
        println!("{:?}, {:?}", line, poss)
    }
    let i: i32 =    possible.iter().sum();
    println!("{:#?}, {:?}", possible, i);
    
    Ok(())
}