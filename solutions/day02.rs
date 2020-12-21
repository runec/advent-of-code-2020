use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day02";

struct InputLine {
  min: u32,
  max: u32,
  character: char,
  password: String,
}

pub fn solve() -> Result<(), Box<dyn Error>> {
  let inputs = parse_input()?;
  
  solve_part_one(&inputs);
  solve_part_two(&inputs);

  return Ok(());
}

fn solve_part_one(inputs: &Vec<InputLine>) {
  let mut count = 0;
  for input in inputs {
    if check_password(&input) {
      count += 1;
    }
  }
  println!("Valid passwords: {}", count);
}

fn solve_part_two(inputs: &Vec<InputLine>) {
  let mut count = 0;
  for input in inputs {
    if check_password_part_two(&input) {
      count += 1;
    }
  }
  println!("Valid passwords: {}", count);
}

fn check_password(input: &InputLine) -> bool {
  let chars = input.password.chars();
  let mut count = 0;
  for c in chars {
    if c == input.character {
      count += 1;
    }
  }

  return count >= input.min && count <= input.max;
}

fn check_password_part_two(input: &InputLine) -> bool {
  let chars = input.password.chars().collect::<Vec<_>>();

  let first_matches = chars[(input.min - 1) as usize] == input.character;
  let second_matches = chars[(input.max - 1) as usize] == input.character;

  return first_matches != second_matches;
}

fn parse_input() -> Result<Vec<InputLine>, Box<dyn Error>> {

  let mut inputs: Vec<InputLine> = vec![];

  let lines = helpers::read_lines(INPUT_FILE)?;

  for line in lines {
    let line = line?;
    let parts = line.split(' ').collect::<Vec<_>>();
    let limits = parts[0].split('-').collect::<Vec<_>>();

    let min = limits[0].parse::<u32>()?;
    let max = limits[1].parse::<u32>()?;
    let character = parts[1].chars().next().unwrap();
    let password = parts[2];
    inputs.push(InputLine {
      min,
      max,
      character,
      password: password.to_string(),
    });
  }

  return Ok(inputs);
}