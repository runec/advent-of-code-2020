use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day10";

pub fn solve() -> Result<(), Box<dyn Error>> {
  let chargers = parse_input()?;
  solve_part_one(&chargers);
  solve_part_two(&chargers);
  return Ok(());
}

fn solve_part_one(chargers: &Vec<u32>) {
  let mut chargers_copy = chargers.clone();
  chargers_copy.sort();

  let mut diff_ones = 0;
  let mut diff_threes = 1;

  let mut prev = 0;
  for charger in chargers_copy {
    if charger - prev == 1 {
      diff_ones += 1;
    } else if charger - prev == 3 {
      diff_threes += 1;
    }

    prev = charger;
  }

  println!("Part 1 {}", diff_ones * diff_threes);
}

fn solve_part_two(chargers: &Vec<u32>) {
  let mut chargers_copy = chargers.clone();
  chargers_copy.sort();

  let max = chargers_copy[chargers_copy.len()-1];
  let mut arrangement_counts: Vec<u64> = vec![1]; // arrangement_counts[i] = number of arrangements to connect to joltage 'i'

  for i in 1..=max {
    if chargers_copy.contains(&i) {
      let mut arrangements_for_i = 0;
      let mut j = i as usize;
      while j > 0 && j + 3 > i as usize {
        arrangements_for_i += arrangement_counts[j-1];
        j -= 1;
      }
      arrangement_counts.push(arrangements_for_i);
    } else {
      arrangement_counts.push(0);
    }
  }

  println!("Part 2 {}", arrangement_counts[max as usize]);
}

fn parse_input() -> Result<Vec<u32>, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?;
  Ok(lines.map(|l| l.unwrap().parse::<u32>().unwrap()).collect())
}