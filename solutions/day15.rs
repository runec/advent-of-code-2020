use crate::helpers;
use std::error::Error;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/day15";

pub fn solve() -> Result<(), Box<dyn Error>> {
  let start_numbers = parse_input()?;
  solve_part_one(&start_numbers);
  solve_part_two(&start_numbers);
  return Ok(());
}

fn solve_part_one(start_numbers: &Vec<u32>) {
  let mut tracker = setup_start_map(start_numbers);
  let mut current_round = start_numbers.len();
  let mut prev_num = start_numbers[current_round - 1];
  while current_round < 2020 {
    let new_num = match tracker.get(&prev_num) {
      None => 0,
      Some(&x) => current_round as u32 - x -1,
    };

    tracker.insert(prev_num, current_round as u32 - 1);

    current_round += 1;
    prev_num = new_num;

  };
  println!("Part 1: {}", prev_num);

}

fn solve_part_two(start_numbers: &Vec<u32>) {
  let mut tracker = setup_start_map(start_numbers);
  let mut current_round = start_numbers.len();
  let mut prev_num = start_numbers[current_round - 1];
  while current_round < 30000000 {
    let new_num = match tracker.get(&prev_num) {
      None => 0,
      Some(&x) => current_round as u32 - x -1,
    };

    tracker.insert(prev_num, current_round as u32 - 1);

    current_round += 1;
    prev_num = new_num;

  };
  println!("Part 1: {}", prev_num);

}

fn setup_start_map(start_numbers: &Vec<u32>) -> HashMap<u32, u32> {
  let mut start_map = HashMap::new();
  for i in 0..start_numbers.len()-1 {
    let num = start_numbers[i];
    start_map.insert(num, i as u32);
  }

  start_map
}
      
fn parse_input() -> Result<Vec<u32>, Box<dyn Error>> {
  let mut lines = helpers::read_lines(INPUT_FILE)?;
  let line = lines.next().unwrap();

  let numbers: Vec<u32> = line?.split(',').map(|n| n.parse::<u32>().unwrap()).collect();

  Ok(numbers)
}