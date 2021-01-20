use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day25";

const MODULUS: u64 = 20201227;
const SUBJECT: u64 = 7;

pub fn solve() -> Result<(), Box<dyn Error>> {
  let public_keys = parse_input()?;
  solve_part_one(&public_keys);
  return Ok(());
}

fn solve_part_one(public_keys: &(u64, u64)) {

  let &(card_key, door_key) = public_keys;

  let card_loop = find_loop_size(card_key);
  let door_loop = find_loop_size(door_key);

  let card_enc_key = transform(1, door_key, card_loop);

  println!("{} {}", card_enc_key, card_loop);
}

fn transform (base: u64, subject: u64, n: u64) -> u64 {
  let mut k = base;
  for _ in 0..n {
    k = (k * subject) % MODULUS
  }
  k
}

fn find_loop_size (key: u64) -> u64 {
  let mut k = 1;
  let mut loops = 0;
  while k != key {
    k = transform(k, SUBJECT, 1);
    loops += 1;
  }

  loops
}

fn parse_input() -> Result<(u64, u64), Box<dyn Error>> {
  let mut result = vec![];
  let lines = helpers::read_lines(INPUT_FILE)?.map(|l| l.unwrap());
  for line in lines {
    result.push(line.parse().unwrap())
  }
  Ok((result[0], result[1]))
}
