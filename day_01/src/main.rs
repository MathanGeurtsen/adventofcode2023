use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn string_to_digit(in_string: &str) -> Option<i32>{
    let num = match in_string {
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),  
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None
    };
    return num;
}


fn main() -> io::Result<()> {
    let file = File::open("../advent_of_code_secrets/2023/day_01/calibration.txt")?;
    
    let mut numbers: Vec<i32> = Vec::new();
    
    let reader = io::BufReader::new(file);
    let re_pattern : String= .to_string();
    let re_forward = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_backward= Regex::new().unwrap();
    
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let first_number: i32 = re_forward.find(&line).unwrap().as_str();
                let line_numbers: Vec<i32> = re_forward.find_iter(&line).filter_map(|x| string_to_digit(x.as_str())).collect();
                numbers.push(line_numbers[0]*10+line_numbers.last().unwrap());
                println!("{:?}, {:?}, {:#?}", line, line_numbers ,line_numbers[0]*10+line_numbers.last().unwrap() )
                
            }
            Err(_) => eprintln!("Error reading line")
        }
    }
        println!("{:#?}", numbers.iter());
        println!("{:#?}", numbers.iter().sum::<i32>());
    Ok(())
}

