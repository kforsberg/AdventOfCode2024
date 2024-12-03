use regex::Regex;
use std::fs;

fn main() {
    let binding = fs::read_to_string("./inputs/input.txt");
    let input = binding.as_ref().unwrap();
    
    let regex_expression = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let matches: Vec<&str> = regex_expression
        .find_iter(input)
        .map(|m| m.as_str())
        .collect();

    let mut total = 0;

    for instruction in matches {
        total += execute_instruction(instruction);
    }

    println!("The total is {}", total);
}

fn execute_instruction(instruction: &str) -> u64 {
    let regex = Regex::new(r"[0-9]{1,3}").unwrap();
    let matches: Vec<u64> = regex
        .find_iter(instruction)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    return matches.get(0).unwrap() * matches.get(1).unwrap();
}