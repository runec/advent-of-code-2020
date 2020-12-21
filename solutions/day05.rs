use crate::helpers;
use std::error::Error;

const INPUT_FILE: &str = "input/day05";

type Seat = (u32, u32);

pub fn solve() -> Result<(), Box<dyn Error>> {
  let seats = parse_input()?;
  let seat_numbers = seats.iter().map(|s| seat_number_from_bsp(s)).collect::<Vec<_>>();

  solve_part_one(&seat_numbers);
  solve_part_two(&seat_numbers);
  return Ok(());
}

fn solve_part_one(seats: &Vec<Seat>) {
  let max_id = seats.iter().map(|s| get_seat_id(s)).max().unwrap(); 

  println!("Part 1 answer: {}", max_id);
}

fn solve_part_two(seats: &Vec<Seat>) {
  let mut seat_ids = seats.iter().map(|s| get_seat_id(s)).collect::<Vec<_>>();
  seat_ids.sort();

  let mut prev_id: Option<u32> = None;

  for seat_id in seat_ids {
    if let Some(id) = prev_id {
      if id + 1 != seat_id {
        println!("Part 2 answer: {}", id + 1);
        break;
      }
    } 
    prev_id = Some(seat_id);
  }
}

fn seat_number_from_bsp(bsp: &String) -> Seat {
  bsp.chars().fold((0,0), |(row, col), c| {
    match c {
      'F' => (2 * row, col),
      'B' => (2 * row + 1, col),
      'L' => (row, 2 * col),
      'R' => (row, 2 * col + 1),
      _ => (row, col)
    }
  })
}

fn get_seat_id((row, col): &Seat) -> u32 {
  8 * row + col
}

fn parse_input() -> Result<Vec<String>, Box<dyn Error>> {
  let lines = helpers::read_lines(INPUT_FILE)?.map(|l| l.unwrap()).collect::<Vec<_>>();
  return Ok(lines);
}