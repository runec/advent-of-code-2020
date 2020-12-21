use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day01";

pub fn solve() -> Result<(), Box<dyn Error>> {
  let numbers = parse_input()?;
  
  solve_part_one(&numbers);
  solve_part_two(&numbers);



  return Ok(());
}

fn solve_part_one(numbers: &Vec<u32>) {
  for i in 0..(numbers.len()-1) {
    for j in (i+1)..numbers.len() {
      if numbers[i] + numbers[j] == 2020 {
        println!("Found numbers with sum 2020: {} and {}. Product is {}", numbers[i], numbers[j], numbers[i] * numbers[j]);
      }
    }
  }
}
fn solve_part_two(numbers: &Vec<u32>) {
  for i in 0..(numbers.len()-2) {
    for j in (i+1)..numbers.len()-1 {
      for k in j+1..numbers.len() {
        if numbers[i] + numbers[j] + numbers[k] == 2020 {
          println!("Found numbers with sum 2020: {}, {} and {}. Product is {}", numbers[i], numbers[j], numbers[k], numbers[i] * numbers[j] * numbers[k]);
        }
      }
    }
  }
} 

fn parse_input() -> Result<Vec<u32>, Box<dyn Error>> {

  let mut xs: Vec<u32> = vec![];

  println!("{}", INPUT_FILE);

  let lines = helpers::read_lines(INPUT_FILE)?;

  for line in lines {
    xs.push(line?.trim().parse::<u32>()?);
  }

  return Ok(xs);
}