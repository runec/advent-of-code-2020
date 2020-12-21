use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day09";

type PairSum = (u64, u64, u64);

pub fn solve() -> Result<(), Box<dyn Error>> {
  let mut input_iterator = parse_input()?;
  let part_one_result = solve_part_one(&mut input_iterator);
  if let Some(r) = part_one_result {
    let mut input_iterator = parse_input()?;
    solve_part_two(r, &mut input_iterator);
  }
  return Ok(());
}

fn solve_part_one(input_iterator: &mut impl Iterator<Item=u64>) -> Option<u64> {
  let mut base_numbers: Vec<u64> = vec![];
  for _i in 0..25 {
    base_numbers.push(input_iterator.next().unwrap());
  }
  let mut pair_sums = all_pair_sums(&base_numbers);

  while let Some(number) = input_iterator.next() {

    if pair_sums.iter().find(|(sum, _, _)| *sum == number).is_none() {
      println!("Part 1: {}", number);
      return Some(number);
    }

    let oldest_base_number = base_numbers.remove(0);

    let new_pair_sums = base_numbers.iter().map(|num| (*num + number, *num, number));

    pair_sums = pair_sums.into_iter().filter(|(_, a, b)| oldest_base_number != *a && oldest_base_number != *b).collect();
    new_pair_sums.for_each(|ps| pair_sums.push(ps));

    base_numbers.push(number);
  }

  return None;
}

fn solve_part_two(target: u64, input_iterator: &mut impl Iterator<Item=u64>) {
  let mut current_sum = 0;
  let mut current_numbers: Vec<u64> = vec![];


  for num in input_iterator {
    current_sum += num;
    current_numbers.push(num);

    while current_sum > target {
      let removed = current_numbers.remove(0);
      current_sum -= removed;
    }

    if current_sum == target {
      let min = current_numbers.iter().min();
      let max = current_numbers.iter().max();
      println!("Part 2: {}, {:?}", min.unwrap() + max.unwrap(), current_numbers);
      break;
    }
  }
}

fn all_pair_sums(numbers: &Vec<u64>) -> Vec<PairSum> {
  let mut pair_sums: Vec<PairSum> = vec![]; 
  for i in 0..numbers.len()-1 {
    for j in i..numbers.len() {
      pair_sums.push((numbers[i] + numbers[j], numbers[i], numbers[j]));
    }
  }
  pair_sums
}

fn parse_input() -> Result<impl Iterator<Item=u64>, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?;
  Ok(lines.map(|l| l.unwrap().parse::<u64>().unwrap()))
}