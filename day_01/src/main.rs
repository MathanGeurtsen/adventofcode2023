use std::fs::File;
use std::io::{self, BufRead};

fn string_to_digit(in_string: &str) -> Option<i32>{
    return match in_string {
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
}


fn main() -> io::Result<()> {
    let file = File::open("../advent_of_code_secrets/2023/day_01/calibration.txt")?;
    
    let mut numbers: Vec<i32> = Vec::new();
    
    let reader = io::BufReader::new(file);
    let number_words =  ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let mut line_numbers: Vec<i32> = Vec::new();
                for (i, _c) in line.chars().enumerate() {
                    for word in number_words {
                        if line.len() >= word.len() + i && line[i..].starts_with(word) {
                            if let Some(nr) = string_to_digit(word) {line_numbers.push(nr)}
                        }
                    }
                }
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

