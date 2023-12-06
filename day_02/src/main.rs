use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug)]
#[derive(Default)]
struct Game {
    game_nr: i32,
    nr_red: i32,
    nr_green: i32,
    nr_blue: i32,
    line: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let file = File::open("../advent_of_code_secrets/2023/day_02/input.txt")?;
    let reader = io::BufReader::new(file);
    
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    
    let mut games: Vec<Game> = Vec::new();
    
    // Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green
    
    for line_result in reader.lines() {
       
        let split_line : Vec<String>= line_result?.split(':').map(|x| x.to_string()).collect();

        let mut game = Game {
            game_nr: 0,
            nr_red: 0,
            nr_green: 0,
            nr_blue: 0,
            line: split_line[1].clone()
        };


        let Some(game_nr_cap) = Regex::new("Game (?<nr>[0-9]+)")?.captures(&split_line[0])
        else { return Err("Failed to parse game nr".into()) };
        game.game_nr = game_nr_cap["nr"].parse()?;
        
        let hands: Vec<Vec<&str>> = split_line[1].split(';').map(|hand| hand.split(',').collect()).collect();
        for hand in hands{
            for group in hand {
                let Some(split_group) = Regex::new("(?<nr>[0-9]+) (?<color>[a-z]+)")?.captures(group)
                else { return Err("failed to parse group".into()) };
                let nr: i32 = split_group["nr"].parse()?;
                let color = &split_group["color"];
                
                match color {
                    "red" => { game.nr_red += nr},
                    "green" => { game.nr_green += nr},
                    "blue" => { game.nr_blue += nr},
                    _ => {return Err("failed to split with ':'".into()) }
                } 
            }
        }
        games.push(game)
    }
    let mut possible: Vec<i32> = Vec::new();
    
    for game in games {
        let mut poss: bool = false;
        if game.nr_red <= red_max && game.nr_green <= green_max && game.nr_blue <= blue_max {
            poss = true;
            possible.push(game.game_nr)
        }

        println!("{:#?}, {:#?}", game, poss);
    }
    println!("{:?}", possible.iter().sum::<i32>());
    Ok(())
}